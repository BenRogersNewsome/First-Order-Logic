use crate::{
    semantics::{
        Arguments, ElementQuantifier, ElementSet, Existential, GraphTraversalSignature, Predicate,
    },
    AssertionResponse, TruthValue,
};

use super::super::PredicateNode;

/// Assert that the predicate is true for all combinations of arguments.
#[derive(Debug)]
pub struct UniversallyObeyed();

impl<E: Clone, const ARITY: usize> Predicate<E, ARITY> for UniversallyObeyed {
    fn call_for_elements(
        &self,
        _: &Arguments<ElementQuantifier<E>, ARITY>,
        _: &mut GraphTraversalSignature,
    ) -> TruthValue {
        TruthValue::Determined(true)
    }

    fn get_elements_for_true(
        &self,
        _: &mut GraphTraversalSignature,
    ) -> Vec<Arguments<ElementSet<E>, ARITY>> {
        vec![Arguments::every(ElementSet::All)]
    }

    fn get_elements_for_false(
        &self,
        _: &mut GraphTraversalSignature,
    ) -> Vec<Arguments<ElementSet<E>, ARITY>> {
        vec![Arguments::every(ElementSet::None)]
    }
}

impl UniversallyObeyed {
    /// Make the assertion.
    pub fn assert_on<E: Clone, const ARITY: usize>(
        predicate: &PredicateNode<E, ARITY>,
    ) -> AssertionResponse {
        for args in predicate.get_elements_for_false(&mut Vec::new()) {
            if args.exists() {
                return AssertionResponse::AssertionInvalid;
            };
        }

        for args in predicate.get_elements_for_true(&mut Vec::new()) {
            if args.maximal() {
                return AssertionResponse::AssertionRedundant;
            };
        }

        predicate.replace(|_| Box::new(UniversallyObeyed()));

        AssertionResponse::AssertionMade
    }

    fn _assert_on_unchecked<E: Clone, const ARITY: usize>(predicate: PredicateNode<E, ARITY>) {
        predicate.replace(|_| Box::new(Self()))
    }
}

#[cfg(test)]
mod test_universally_obeyed {
    use crate::{
        args,
        semantics::{
            elements::{ElementQuantifier, ElementSet},
            Predicate, PredicateNode,
        },
        AssertionResponse, TruthValue,
    };

    use super::UniversallyObeyed;

    fn setup() -> PredicateNode<usize, 1> {
        let predicate: PredicateNode<usize, 1> = PredicateNode::default();
        assert_eq!(
            UniversallyObeyed::assert_on(&predicate),
            AssertionResponse::AssertionMade,
        );
        predicate
    }

    #[test]
    fn test_call_for_args() {
        let predicate = setup();

        assert_eq!(
            predicate.call_for_elements(&args!(1), &mut Vec::new()),
            TruthValue::Determined(true),
        );

        assert_eq!(
            predicate.call_for_elements(&args!(ElementQuantifier::Any), &mut Vec::new()),
            TruthValue::Determined(true),
        );
    }

    #[test]
    fn test_get_elements_for_true() {
        let predicate = setup();

        assert_eq!(
            predicate.get_elements_for_true(&mut Vec::new()),
            vec![args!(ElementSet::All)],
        );
    }

    #[test]
    fn test_get_elements_for_false() {
        let predicate = setup();

        assert_eq!(
            predicate.get_elements_for_false(&mut Vec::new()),
            vec![args!(ElementSet::None)],
        );
    }
}
