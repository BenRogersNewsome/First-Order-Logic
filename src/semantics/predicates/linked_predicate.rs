use crate::{Element, Predicate, TruthValue, graph::{GraphTraversalSignature, ElementCollection}};


/// Create a predicate which is linked to some external node.
#[derive(Debug)]
pub struct LinkedPredicate<E> {
    linked: Box<dyn Predicate<E>>,
}

impl<E: Element> Predicate<E> for LinkedPredicate<E> {
    fn call_for_element(&self, element_node: &E, sig: &mut GraphTraversalSignature) -> TruthValue {
        self.linked.call_for_element(element_node, sig)
    }

    fn get_elements_for_false(&self, sig: &mut GraphTraversalSignature) -> ElementCollection<E> {
        self.linked.get_elements_for_false(sig)
    }

    fn get_elements_for_true(&self, sig: &mut GraphTraversalSignature) -> ElementCollection<E> {
        self.linked.get_elements_for_true(sig)
    }
}

impl<E> From<Box<dyn Predicate<E>>> for LinkedPredicate<E> {
    fn from(linked: Box<dyn Predicate<E>>) -> Self {
        Self {
            linked,
        }
    }
}