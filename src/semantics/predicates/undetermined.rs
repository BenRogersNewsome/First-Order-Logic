use crate::{
    semantics::{Arguments, ElementQuantifier, ElementSet, GraphTraversalSignature, Predicate},
    TruthValue,
};

/// A base predicate which makes no assertions on the predicate.
#[derive(Debug)]
pub struct Undetermined();

impl<E: Clone, const ARITY: usize> Predicate<E, ARITY> for Undetermined {
    fn call_for_elements(
        &self,
        _: &Arguments<ElementQuantifier<E>, ARITY>,
        _: &mut GraphTraversalSignature,
    ) -> TruthValue {
        TruthValue::Undetermined
    }

    fn get_elements_for_false(&self) -> Vec<Arguments<ElementSet<E>, ARITY>> {
        vec![]
    }

    fn get_elements_for_true(&self) -> Vec<Arguments<ElementSet<E>, ARITY>> {
        vec![]
    }
}
