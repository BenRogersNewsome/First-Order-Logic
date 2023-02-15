use crate::{PredicateNode, Element, AssertionResponse, predicates};


pub fn implies<E: 'static + Element>(left: &PredicateNode<E>, right: &PredicateNode<E>) -> AssertionResponse {
    let assertion_predicate =
        predicates::Implication::new(&left, &right);
    
    predicates::UniversallyObeyed::assert_on(assertion_predicate)
}