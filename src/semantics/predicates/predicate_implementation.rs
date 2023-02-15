use crate::{TruthValue, AssertionResponse, Element, Predicate};

use super::super::PredicateNode;

pub type PredicateCallable<E> = fn(element: &E) -> TruthValue;
pub type PredicateTakeAssertionCallable<E> =
    fn(element: &E, truth: TruthValue) -> AssertionResponse;

/// Declare an concrete implementation for a predicate using a fn pointer. Can
/// be used to link FOL with other logical frameworks, such as ZFST or higher
/// order logics.
pub struct PredicateImplementation<'a, E: Element> {
    call_assertion: PredicateCallable<E>,
    take_assertion: PredicateTakeAssertionCallable<E>,
    get_known_true: fn() -> Vec<&'a E>,
    get_known_false: fn() -> Vec<&'a E>,
}

impl<E: Element> Predicate<E> for PredicateImplementation<'_, E> {
    fn call_for_element(&self, element: &E) -> TruthValue {
        (self.call_assertion)(element)
    }

    fn get_elements_for_false(&self) -> Vec<&E> {
        (self.get_known_true)()
    }

    fn get_elements_for_true(&self) -> Vec<&E> {
        (self.get_known_true)()
    }
}

impl<'a, E: Element> PredicateImplementation<'a, E> {
    pub fn new(
        call_assertion: PredicateCallable<E>,
        take: PredicateTakeAssertionCallable<E>,
        get_known_true: fn() -> Vec<&'a E>,
        get_known_false: fn() -> Vec<&'a E>,
    ) -> PredicateNode<'a, E> {
        PredicateNode::new(
            Box::new(
                Self {
                    call_assertion,
                    take_assertion: take,
                    get_known_true,
                    get_known_false,
                }
            )
        )
    }
}

