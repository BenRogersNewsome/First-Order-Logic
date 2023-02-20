
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

    fn get_elements_for_false(
        &self,
        _: &mut GraphTraversalSignature,
    ) -> Vec<Arguments<ElementSet<E>, ARITY>> {
        vec![]
    }

    fn get_elements_for_true(
        &self,
        _: &mut GraphTraversalSignature,
    ) -> Vec<Arguments<ElementSet<E>, ARITY>> {
        vec![]
    }
}

#[cfg(test)]
mod test_undetermined {
    use crate::{
        args,
        semantics::{Predicate, PredicateNode},
        TruthValue,
    };

    use super::Undetermined;

    fn setup() -> PredicateNode<usize, 1> {
        PredicateNode::new(Box::new(Undetermined()))
    }

    #[test]
    fn test_call_for_args() {
        let predicate = setup();

        assert_eq!(
            predicate.call_for_elements(&args!(1), &mut Vec::new()),
            TruthValue::Undetermined,
        );
    }

    #[test]
    fn test_get_elements_for_true() {
        let predicate = setup();

        assert_eq!(predicate.get_elements_for_true(&mut Vec::new()), vec![],);
    }

    #[test]
    fn test_get_elements_for_false() {
        let predicate = setup();

        assert_eq!(predicate.get_elements_for_false(&mut Vec::new()), vec![],);
    }
}
