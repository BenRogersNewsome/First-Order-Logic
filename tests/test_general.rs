#[macro_use(one_to_one)]
extern crate first_order_logic;

use rand;

use first_order_logic::{
    elements::Arguments,
    predicates::{self, Conjunction, TrueForElement, UniversallyObeyed},
    AssertionResponse, ElementQuantifier, Predicate, PredicateNode, TruthValue,
};

// enum UniversalOrEquality<L: Term, R: Term> {
//     Universal(Box<Universal<UniversalOrEquality<L, R>>>),
//     Equality(Equality<L, R>)
// }
// impl<L: Term, R: Term> Formula for UniversalOrEquality<L, R> {}

// type AssertionEquation = Universal<
//     Equality<Function, Function>
// >;

// fn test_identity_assertion() {

//     let identity_assertion: AssertionEquation = Universal {
//         left: vec![
//             Variable { label: b'a', index: None, prime: false },
//             Variable { label: b'b', index: None, prime: false },
//         ],
//         right: Equality {
//             left: Function {
//                 arity: 2,
//                 label: b'+',
//                 terms: vec![
//                     Box::new(Variable { label: b'a', index: None, prime: false }),
//                     Box::new(Variable { label: b'b', index: None, prime: false }),
//                 ]
//             },
//             right: Function {
//                 arity: 2,
//                 label: b'+',
//                 terms: vec![
//                     Box::new(Variable { label: b'b', index: None, prime: false }),
//                     Box::new(Variable { label: b'a', index: None, prime: false }),
//                 ]
//             },

//         },
//     };

// }

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Item {
    id: u64,
}

impl Item {
    pub fn new() -> Self {
        Self { id: rand::random() }
    }
}

// #[test]
// fn test_implication() {

//     let predicate_a = PredicateNode::new(
//         Box::new(predicates::Undetermined())
//     );

//     let predicate_b = PredicateNode::new(
//         Box::new(predicates::Undetermined())
//     );

//     assert_eq!(
//         assertions::implies(&predicate_a, &predicate_b),
//         AssertionResponse::AssertionMade,
//     );

//     assert_eq!(
//         assertions::implies(&predicate_a, &predicate_b),
//         AssertionResponse::AssertionRedundant,
//     );

//     let elem_a = Item::new();
//     predicates::TrueForElement::assert_on(predicate_a,  &elem_a);

//     assert_eq!(predicate_b.call_for_element(&elem_a, &mut Vec::new()), TruthValue::Determined(true))
// }

#[test]
fn test_conjunction() {
    let predicate_a: PredicateNode<usize, 1> =
        PredicateNode::new(Box::new(predicates::Undetermined()));

    let predicate_b: PredicateNode<usize, 1> =
        PredicateNode::new(Box::new(predicates::Undetermined()));

    assert_eq!(
        UniversallyObeyed::assert_on(&predicate_a),
        AssertionResponse::AssertionMade,
    );

    assert_eq!(
        UniversallyObeyed::assert_on(&predicate_a),
        AssertionResponse::AssertionRedundant,
    );

    assert_eq!(
        TrueForElement::assert_on(
            &predicate_b,
            vec![Arguments::from([ElementQuantifier::One(3)])],
        ),
        AssertionResponse::AssertionMade,
    );

    let conjunction_node =
        Conjunction::new(&predicate_a, one_to_one!(), &predicate_b, one_to_one!());

    assert_eq!(
        conjunction_node.call_for_elements(
            &Arguments::from([ElementQuantifier::One(3)]),
            &mut Vec::new()
        ),
        TruthValue::Determined(true),
    );

    assert_eq!(
        conjunction_node.call_for_elements(
            &Arguments::from([ElementQuantifier::One(2)]),
            &mut Vec::new()
        ),
        TruthValue::Undetermined
    );

    // assert_eq!(
    //     assertions::implies(&predicate_a, &predicate_b),
    //     AssertionResponse::AssertionRedundant,
    // );

    // let elem_a = Item::new();
    // predicates::TrueForElement::assert_on(predicate_a,  &elem_a);

    // assert_eq!(predicate_b.call_for_element(&elem_a, &mut Vec::new()), TruthValue::Determined(true))
}
