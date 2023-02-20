use std::hash::Hash;

use crate::{
    one_to_one,
    semantics::{
        predicates::{Disjunction, Negation, UniversallyObeyed},
        PredicateNode,
    },
    AssertionResponse,
};

/// Assert that left implies right:
///     left -> right
pub fn implies<E: 'static + Clone + Eq + Hash, const ARITY: usize>(
    left: &PredicateNode<E, ARITY>,
    right: &PredicateNode<E, ARITY>,
) -> AssertionResponse {
    let implication: PredicateNode<E, ARITY> = Disjunction::create(
        &Negation::create(left),
        one_to_one!(ARITY),
        right,
        one_to_one!(ARITY),
    );

    UniversallyObeyed::assert_on(&implication)
}
