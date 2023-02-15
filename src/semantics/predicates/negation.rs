use crate::{graph::nodes::ElementArgs, Predicate};


pub struct Negation<E, A: ElementArgs<E>> {
    inner: Box<dyn Predicate<E, A>>,
}

impl<E, A: ElementArgs<E>> Predicate<E, A> for Negation<E, A>{

    fn call_for_elements(&self, element_nodes: A, sig: &mut crate::graph::GraphTraversalSignature) -> crate::TruthValue {
        
    }
}