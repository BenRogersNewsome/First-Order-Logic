use first_order_logic::{
    args, one_to_one,
    semantics::{
        predicates::{Disjunction, Negation, TrueForArguments, UniversallyObeyed},
        Predicate, PredicateNode,
    },
    AssertionResponse, TruthValue,
};

/// Create an implication of the form A -> B, and assert the result is
/// universally true.
///
/// Assert that when A is true, B is true.
///
/// Implement the implication using the identity:
///     A -> B ~= (!A) \/ B
#[test]
fn test_simple_implication() {
    let predicate_a: PredicateNode<usize, 1> = PredicateNode::default();
    let predicate_b: PredicateNode<usize, 1> = PredicateNode::default();
    let implication: PredicateNode<usize, 1> = Disjunction::create(
        &Negation::create(&predicate_a),
        one_to_one!(1),
        &predicate_b,
        one_to_one!(1),
    );

    // |- A -> B ~= True
    assert!(matches!(
        UniversallyObeyed::assert_on(&implication),
        AssertionResponse::AssertionMade,
    ));

    // |- A(3) ~= True
    assert!(matches!(
        TrueForArguments::assert_on(&predicate_a, vec![args!(3),]),
        AssertionResponse::AssertionMade,
    ));

    // B(3) ?
    assert_eq!(
        predicate_b.call_for_elements(&args!(3), &mut Vec::new(),),
        TruthValue::Determined(true),
    );
}

/// Test that in the implication A -> B, knowledge of B gives no knowledge of A.
#[test]
fn test_reverse_implication_undetermined() {
    let predicate_a: PredicateNode<usize, 1> = PredicateNode::default();
    let predicate_b: PredicateNode<usize, 1> = PredicateNode::default();
    let implication: PredicateNode<usize, 1> = Disjunction::create(
        &Negation::create(&predicate_a),
        one_to_one!(1),
        &predicate_b,
        one_to_one!(1),
    );

    // |- A -> B ~= True
    assert!(matches!(
        UniversallyObeyed::assert_on(&implication),
        AssertionResponse::AssertionMade,
    ));

    // |- B(3) ~= True
    assert!(matches!(
        TrueForArguments::assert_on(&predicate_b, vec![args!(3),]),
        AssertionResponse::AssertionMade,
    ));

    // A(3) ? should = Undetermined
    assert_eq!(
        predicate_a.call_for_elements(&args!(3), &mut Vec::new(),),
        TruthValue::Undetermined,
    );
}
