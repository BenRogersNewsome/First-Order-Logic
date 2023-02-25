use std::hash::Hash;

use crate::{
    semantics::{
        ArgumentMap, Arguments, ElementQuantifier, ElementSet, GraphTraversalSignature, Predicate,
        PredicateNode,
    },
    TruthValue,
};

////////////////////////////////////////////////////////////////////////////////
// Disjunction
////////////////////////////////////////////////////////////////////////////////

/// A predicate representing the disjunction between two nodes.
pub struct Disjunction<E, const L_ARITY: usize, const R_ARITY: usize, const D_ARITY: usize> {
    left: PredicateNode<E, L_ARITY>,
    right: PredicateNode<E, R_ARITY>,
    map_left: ArgumentMap<D_ARITY, L_ARITY>,
    map_right: ArgumentMap<D_ARITY, R_ARITY>,
    sig: u64,
}

impl<
        E: 'static + Hash + Eq + Clone,
        const L_ARITY: usize,
        const R_ARITY: usize,
        const D_ARITY: usize,
    > Disjunction<E, L_ARITY, R_ARITY, D_ARITY>
{
    /// Create a predicate node from the disjunction of two other nodes, given
    /// the specified argument maps between the disjunction node and the
    /// operand nodes.
    pub fn create(
        left: &PredicateNode<E, L_ARITY>,
        map_left: ArgumentMap<D_ARITY, L_ARITY>,
        right: &PredicateNode<E, R_ARITY>,
        map_right: ArgumentMap<D_ARITY, R_ARITY>,
    ) -> PredicateNode<E, D_ARITY> {
        let sig: u64 = rand::random();

        let disjunction_node = PredicateNode::new(Box::new(Self {
            left: left.clone(),
            map_left,
            map_right,
            right: right.clone(),
            sig,
        }));

        left.replace(|inner| {
            Box::new(IsDisjunctionPart {
                for_disjunction: disjunction_node.clone(),
                inner,
                map_this: map_left,
                map_other: map_right,
                sig,
                with: right.clone(),
            })
        });

        right.replace(|inner| {
            Box::new(IsDisjunctionPart {
                for_disjunction: disjunction_node.clone(),
                inner,
                map_this: map_right,
                map_other: map_left,
                sig,
                with: left.clone(),
            })
        });

        disjunction_node
    }
}

impl<E: Hash + Eq + Clone, const L_ARITY: usize, const R_ARITY: usize, const D_ARITY: usize>
    Predicate<E, D_ARITY> for Disjunction<E, L_ARITY, R_ARITY, D_ARITY>
{
    fn call_for_elements(
        &self,
        element_nodes: &Arguments<ElementQuantifier<E>, D_ARITY>,
        sig: &mut GraphTraversalSignature,
    ) -> crate::TruthValue {
        if sig.contains(&self.sig) {
            TruthValue::Undetermined
        } else {
            sig.push(self.sig);

            let args_for_left = self.map_left.forward(element_nodes);
            let args_for_right = self.map_right.forward(element_nodes);

            self.left.call_for_elements(&args_for_left, sig)
                | self.right.call_for_elements(&args_for_right, sig)
        }
    }

    fn get_elements_for_true(
        &self,
        sig: &mut GraphTraversalSignature,
    ) -> Vec<Arguments<ElementSet<E>, D_ARITY>> {
        if sig.contains(&self.sig) {
            vec![]
        } else {
            sig.push(self.sig);
            let left_trues = self
                .left
                .get_elements_for_true(sig)
                .into_iter()
                .map(|a| self.map_left.backward(&a, ElementSet::<E>::All));

            let right_trues: Vec<_> = self
                .right
                .get_elements_for_true(sig)
                .into_iter()
                .map(|a| self.map_right.backward(&a, ElementSet::<E>::All))
                .collect();

            left_trues
                .flat_map(|l| right_trues.iter().map(move |r| (l.clone(), r.clone())))
                .map(|(mut l, r)| {
                    l |= r;
                    l
                })
                .collect()
        }
    }

    fn get_elements_for_false(
        &self,
        sig: &mut GraphTraversalSignature,
    ) -> Vec<Arguments<ElementSet<E>, D_ARITY>> {
        if sig.contains(&self.sig) {
            vec![]
        } else {
            sig.push(self.sig);
            let left_falses = self
                .left
                .get_elements_for_false(sig)
                .into_iter()
                .map(|a| self.map_left.backward(&a, ElementSet::<E>::All));

            let right_falses: Vec<_> = self
                .right
                .get_elements_for_false(sig)
                .into_iter()
                .map(|a| self.map_right.backward(&a, ElementSet::<E>::All))
                .collect();

            left_falses
                .flat_map(|l| right_falses.iter().map(move |r| (l.clone(), r.clone())))
                .map(|(mut l, r)| {
                    l &= r;
                    l
                })
                .collect()
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Disjunction Part
////////////////////////////////////////////////////////////////////////////////

/// Added to disjunction operands to link them to their parent disjunction.
struct IsDisjunctionPart<E, const WITH_ARITY: usize, const D_ARITY: usize, const THIS_ARITY: usize>
{
    pub(self) for_disjunction: PredicateNode<E, D_ARITY>,
    pub(self) inner: Box<dyn Predicate<E, THIS_ARITY>>,
    pub(self) map_other: ArgumentMap<D_ARITY, WITH_ARITY>,
    pub(self) map_this: ArgumentMap<D_ARITY, THIS_ARITY>,
    pub(self) sig: u64,
    pub(self) with: PredicateNode<E, WITH_ARITY>,
}

impl<E: Hash + Eq + Clone, const WITH_ARITY: usize, const D_ARITY: usize, const ARITY: usize>
    Predicate<E, ARITY> for IsDisjunctionPart<E, WITH_ARITY, D_ARITY, ARITY>
{
    fn call_for_elements(
        &self,
        element_nodes: &Arguments<ElementQuantifier<E>, ARITY>,
        sig: &mut GraphTraversalSignature,
    ) -> TruthValue {
        if sig.contains(&self.sig) {
            self.inner.call_for_elements(element_nodes, sig)
        } else {
            sig.push(self.sig);

            let mapped_args_for_disjunction: Arguments<_, D_ARITY> = self
                .map_this
                .backward(element_nodes, ElementQuantifier::Any);

            let mapped_args_for_other_disjunction_part: Arguments<_, WITH_ARITY> =
                self.map_other.forward(&mapped_args_for_disjunction);

            match (
                self.for_disjunction
                    .call_for_elements(&mapped_args_for_disjunction, sig),
                self.with
                    .call_for_elements(&mapped_args_for_other_disjunction_part, sig),
            ) {
                (TruthValue::Determined(false), _) => TruthValue::Determined(false),

                (TruthValue::Determined(true), TruthValue::Determined(false)) => {
                    TruthValue::Determined(true)
                }

                _ => TruthValue::Undetermined,
            }
        }
    }

    /// Operand will be false for all args for which the disjunction itself is
    /// false.
    fn get_elements_for_false(
        &self,
        sig: &mut GraphTraversalSignature,
    ) -> Vec<Arguments<ElementSet<E>, ARITY>> {
        if sig.contains(&self.sig) {
            self.inner.get_elements_for_false(sig)
        } else {
            sig.push(self.sig);
            self.for_disjunction
                .get_elements_for_false(sig)
                .into_iter()
                .map(|args| self.map_this.forward(&args))
                .chain(self.inner.get_elements_for_false(sig).into_iter())
                .collect()
        }
    }

    /// Operand will be true for all args for which the disjunction is true and
    /// the other operand is false.
    fn get_elements_for_true(
        &self,
        sig: &mut GraphTraversalSignature,
    ) -> Vec<Arguments<ElementSet<E>, ARITY>> {
        if sig.contains(&self.sig) {
            self.inner.get_elements_for_true(sig)
        } else {
            sig.push(self.sig);
            let disjunction_true = self
                .for_disjunction
                .get_elements_for_true(sig)
                .into_iter()
                .map(|args| self.map_this.forward(&args));

            let other_operand_false: Vec<_> = self
                .with
                .get_elements_for_false(sig)
                .into_iter()
                .map(|args| self.map_other.backward(&args, ElementSet::All))
                .map(|args| self.map_this.forward(&args))
                .collect();

            disjunction_true
                .flat_map(|l| {
                    other_operand_false
                        .iter()
                        .map(move |r| (l.clone(), r.clone()))
                })
                .map(|(mut l, r)| {
                    l &= r;
                    l
                })
                .chain(self.inner.get_elements_for_true(sig))
                .collect()
        }
    }
}

#[cfg(test)]
mod test_disjunction {
    use crate::{
        one_to_one,
        semantics::{
            elements::{Arguments, ElementQuantifier},
            predicates::{Negation, TrueForArguments},
            Predicate, PredicateNode,
        },
        TruthValue,
    };

    use super::Disjunction;

    #[test]
    fn test_disjunction_forward_assertions() {
        let predicate_a: PredicateNode<usize, 1> = PredicateNode::default();
        let args: Arguments<ElementQuantifier<usize>, 1> =
            Arguments::from([ElementQuantifier::One(4)]);
        TrueForArguments::assert_on(&predicate_a, vec![args.clone()]);

        let predicate_b: PredicateNode<usize, 1> = PredicateNode::default();

        let disjunction: PredicateNode<usize, 1> =
            Disjunction::create(&predicate_a, one_to_one!(1), &predicate_b, one_to_one!(1));

        assert_eq!(
            disjunction.call_for_elements(&args, &mut Vec::new()),
            TruthValue::Determined(true)
        );
    }

    #[test]
    fn test_disjunction_reverse_assertions() {
        let args: Arguments<ElementQuantifier<usize>, 1> =
            Arguments::from([ElementQuantifier::One(4)]);
        let predicate_a: PredicateNode<usize, 1> = PredicateNode::default();

        let predicate_b: PredicateNode<usize, 1> = PredicateNode::default();

        let disjunction: PredicateNode<usize, 1> =
            Disjunction::create(&predicate_a, one_to_one!(1), &predicate_b, one_to_one!(1));

        TrueForArguments::assert_on(&Negation::create(&disjunction), vec![args.clone()]);

        assert_eq!(
            predicate_a.call_for_elements(&args, &mut Vec::new()),
            TruthValue::Determined(false)
        );
    }
}
