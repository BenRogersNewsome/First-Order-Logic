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
