use crate::{
    semantics::{Predicate, PredicateNode},
    TruthValue,
};

////////////////////////////////////////////////////////////////////////////////
// Negation
////////////////////////////////////////////////////////////////////////////////

/// A predicate for the negation of another predicate
pub struct Negation<E, const ARITY: usize> {
    of: PredicateNode<E, ARITY>,
    sig: u64,
}

impl<E, const ARITY: usize> Predicate<E, ARITY> for Negation<E, ARITY> {
    fn call_for_elements(
        &self,
        arguments: &crate::semantics::elements::Arguments<
            crate::semantics::elements::ElementQuantifier<E>,
            ARITY,
        >,
        sig: &mut crate::semantics::GraphTraversalSignature,
    ) -> crate::TruthValue {
        if !sig.contains(&self.sig) {
            !self.of.call_for_elements(arguments, sig)
        } else {
            sig.push(self.sig);
            TruthValue::Undetermined
        }
    }

    fn get_elements_for_false(
        &self,
    ) -> Vec<crate::semantics::elements::Arguments<crate::semantics::elements::ElementSet<E>, ARITY>>
    {
        self.of.get_elements_for_true()
    }

    fn get_elements_for_true(
        &self,
    ) -> Vec<crate::semantics::elements::Arguments<crate::semantics::elements::ElementSet<E>, ARITY>>
    {
        self.of.get_elements_for_false()
    }
}

impl<E: 'static, const ARITY: usize> Negation<E, ARITY> {
    /// Create a new predicate node from the negation of another predicate node.
    ///
    /// Arity is preserved when taking the negation.
    pub fn create(of: &PredicateNode<E, ARITY>) -> PredicateNode<E, ARITY> {
        let sig: u64 = rand::random();
        let negation = PredicateNode::new(Box::new(Self {
            of: of.clone(),
            sig,
        }));

        of.replace(|inner| {
            Box::new(IsNegated {
                negation: negation.clone(),
                inner,
                sig,
            })
        });

        negation
    }
}

////////////////////////////////////////////////////////////////////////////////
// Is Negated
////////////////////////////////////////////////////////////////////////////////

/// The reverse relationship for a negation, so that a negated node can keep
/// track of what it has been negated to become.
struct IsNegated<E, const ARITY: usize> {
    pub(self) negation: PredicateNode<E, ARITY>,
    pub(self) inner: Box<dyn Predicate<E, ARITY>>,
    pub(self) sig: u64,
}

impl<E, const ARITY: usize> Predicate<E, ARITY> for IsNegated<E, ARITY> {
    fn call_for_elements(
        &self,
        arguments: &crate::semantics::elements::Arguments<
            crate::semantics::elements::ElementQuantifier<E>,
            ARITY,
        >,
        sig: &mut crate::semantics::GraphTraversalSignature,
    ) -> crate::TruthValue {
        if sig.contains(&self.sig) {
            self.inner.call_for_elements(arguments, sig)
        } else {
            sig.push(self.sig);

            if let TruthValue::Determined(t) = !self.negation.call_for_elements(arguments, sig) {
                TruthValue::Determined(t)
            } else {
                self.inner.call_for_elements(arguments, sig)
            }
        }
    }

    fn get_elements_for_false(
        &self,
    ) -> Vec<crate::semantics::elements::Arguments<crate::semantics::elements::ElementSet<E>, ARITY>>
    {
        self.negation
            .get_elements_for_true()
            .into_iter()
            .chain(self.inner.get_elements_for_false().into_iter())
            .collect()
    }

    fn get_elements_for_true(
        &self,
    ) -> Vec<crate::semantics::elements::Arguments<crate::semantics::elements::ElementSet<E>, ARITY>>
    {
        self.negation
            .get_elements_for_false()
            .into_iter()
            .chain(self.inner.get_elements_for_true().into_iter())
            .collect()
    }
}

#[cfg(test)]
mod test_negations {
    use crate::{
        semantics::{
            elements::{Arguments, ElementQuantifier},
            predicates::TrueForArguments,
            Predicate, PredicateNode,
        },
        TruthValue,
    };

    use super::Negation;

    #[test]
    fn test_forward_negation_assertion() {
        let args = Arguments::from([ElementQuantifier::One(3)]);

        let predicate_a: PredicateNode<usize, 1> = PredicateNode::default();
        TrueForArguments::assert_on(&predicate_a, vec![args.clone()]);

        let negated = Negation::create(&predicate_a);

        assert_eq!(
            negated.call_for_elements(&args, &mut Vec::new()),
            TruthValue::Determined(false)
        );
    }

    #[test]
    fn test_reverse_negation_assertion() {
        let args = Arguments::from([ElementQuantifier::One(3)]);

        let predicate_a: PredicateNode<usize, 1> = PredicateNode::default();
        let negated = Negation::create(&predicate_a);

        TrueForArguments::assert_on(&negated, vec![args.clone()]);

        assert_eq!(
            predicate_a.call_for_elements(&args, &mut Vec::new()),
            TruthValue::Determined(false)
        );
    }
}
