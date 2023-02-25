use crate::{
    semantics::GraphTraversalSignature,
    semantics::{
        elements::{Arguments, ElementQuantifier, ElementSet},
        Predicate, PredicateNode,
    },
    AssertionResponse, TruthValue,
};

/// Assert that a given predicate is true for some set of arguments.
pub struct TrueForArguments<E, const ARITY: usize> {
    arguments: Vec<Arguments<ElementQuantifier<E>, ARITY>>,
    inner: Box<dyn Predicate<E, ARITY>>,
}

impl<E: Eq + Clone, const ARITY: usize> Predicate<E, ARITY> for TrueForArguments<E, ARITY> {
    fn call_for_elements(
        &self,
        arguments: &Arguments<ElementQuantifier<E>, ARITY>,
        sig: &mut GraphTraversalSignature,
    ) -> TruthValue {
        let element_matches = self
            .arguments
            .iter()
            .any(|true_args| true_args == arguments);

        TruthValue::Determined(element_matches) | self.inner.call_for_elements(arguments, sig)
    }

    fn get_elements_for_false(
        &self,
        sig: &mut GraphTraversalSignature,
    ) -> Vec<Arguments<ElementSet<E>, ARITY>> {
        self.inner.get_elements_for_false(sig)
    }

    fn get_elements_for_true(
        &self,
        sig: &mut GraphTraversalSignature,
    ) -> Vec<Arguments<ElementSet<E>, ARITY>> {
        self.arguments
            .iter()
            .cloned()
            .map(|args| {
                args.map::<ElementSet<E>>(|e| match e {
                    ElementQuantifier::One(x) => ElementSet::Some(vec![x]),
                    ElementQuantifier::Any => ElementSet::All,
                })
            })
            .chain(self.inner.get_elements_for_true(sig).into_iter())
            .collect()
    }
}

impl<E: 'static + Eq + Clone, const ARITY: usize> TrueForArguments<E, ARITY> {
    /// Make the assertion
    pub fn assert_on(
        predicate_node: &PredicateNode<E, ARITY>,
        args: Vec<Arguments<ElementQuantifier<E>, ARITY>>,
    ) -> AssertionResponse {
        let mut undermined_args = Vec::with_capacity(args.capacity());
        for arg in args {
            match predicate_node.call_for_elements(&arg, &mut Vec::new()) {
                TruthValue::Determined(false) => return AssertionResponse::AssertionInvalid,
                TruthValue::Determined(true) => {}
                TruthValue::Undetermined => undermined_args.push(arg),
            };
        }

        if undermined_args.is_empty() {
            return AssertionResponse::AssertionRedundant;
        };

        predicate_node.replace(move |inner| {
            Box::new(TrueForArguments {
                arguments: undermined_args.clone(),
                inner,
            })
        });
        AssertionResponse::AssertionMade
    }
}

#[cfg(test)]
mod test_true_for_argument {
    use crate::{
        args,
        semantics::{Predicate, PredicateNode},
        AssertionResponse, TruthValue,
    };

    use super::TrueForArguments;

    #[test]
    fn test_call_for_args() {
        let predicate: PredicateNode<usize, 2> = PredicateNode::default();
        assert_eq!(
            TrueForArguments::assert_on(&predicate, vec![args!(2, 4), args!(2, 3),]),
            AssertionResponse::AssertionMade,
        );

        assert_eq!(
            predicate.call_for_elements(&args!(2, 4), &mut Vec::new()),
            TruthValue::Determined(true),
        );
        assert_eq!(
            predicate.call_for_elements(&args!(2, 3), &mut Vec::new()),
            TruthValue::Determined(true),
        );
        assert_eq!(
            predicate.call_for_elements(&args!(3, 4), &mut Vec::new()),
            TruthValue::Undetermined,
        );
    }

    #[test]
    fn test_get_elements_for_true() {
        let predicate: PredicateNode<usize, 1> = PredicateNode::default();
        assert_eq!(
            TrueForArguments::assert_on(&predicate, vec![args!(2), args!(3),]),
            AssertionResponse::AssertionMade,
        );

        assert_eq!(
            predicate.get_elements_for_true(&mut Vec::new()),
            vec![args!(2), args!(3),],
        );
    }

    #[test]
    fn test_get_elements_for_false() {
        let predicate: PredicateNode<usize, 1> = PredicateNode::default();
        assert_eq!(
            TrueForArguments::assert_on(&predicate, vec![args!(2), args!(3),]),
            AssertionResponse::AssertionMade,
        );

        assert_eq!(predicate.get_elements_for_false(&mut Vec::new()), vec![],);
    }

    #[test]
    fn test_repeated_assertion_without_redundancy() {
        let predicate: PredicateNode<usize, 1> = PredicateNode::default();
        assert_eq!(
            TrueForArguments::assert_on(&predicate, vec![args!(2), args!(3),]),
            AssertionResponse::AssertionMade,
        );

        assert_eq!(
            TrueForArguments::assert_on(&predicate, vec![args!(2), args!(4),]),
            AssertionResponse::AssertionMade,
        );

        // Shouldn't return `args!(2)` twice.
        assert_eq!(
            predicate.get_elements_for_true(&mut Vec::new()),
            vec![args!(4), args!(2), args!(3),],
        );
    }

    #[test]
    fn test_repeated_assertion_with_redundancy() {
        let predicate: PredicateNode<usize, 1> = PredicateNode::default();
        assert_eq!(
            TrueForArguments::assert_on(&predicate, vec![args!(2), args!(3),]),
            AssertionResponse::AssertionMade,
        );

        assert_eq!(
            TrueForArguments::assert_on(&predicate, vec![args!(2), args!(3),]),
            AssertionResponse::AssertionRedundant,
        );
    }
}
