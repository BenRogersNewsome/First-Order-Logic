use std::hash::Hash;

use crate::{
    semantics::{
        ArgumentMap, Arguments, ElementQuantifier, ElementSet, GraphTraversalSignature, Predicate,
        PredicateNode,
    },
    TruthValue,
};

////////////////////////////////////////////////////////////////////////////////
// Conjunction
////////////////////////////////////////////////////////////////////////////////

/// A predicate representing the conjunction between two nodes.
pub struct Conjunction<E, const L_ARITY: usize, const R_ARITY: usize, const C_ARITY: usize> {
    left: PredicateNode<E, L_ARITY>,
    map_left: ArgumentMap<C_ARITY, L_ARITY>,
    map_right: ArgumentMap<C_ARITY, R_ARITY>,
    right: PredicateNode<E, R_ARITY>,
    sig: u64,
}

impl<
        E: 'static + Hash + Eq + Clone,
        const L_ARITY: usize,
        const R_ARITY: usize,
        const C_ARITY: usize,
    > Conjunction<E, L_ARITY, R_ARITY, C_ARITY>
{
    /// Create a predicate node from the conjunction of two other nodes, given
    /// the specified argument maps between the conjunction node and the
    /// operand nodes.
    pub fn create(
        left: &PredicateNode<E, L_ARITY>,
        map_left: ArgumentMap<C_ARITY, L_ARITY>,
        right: &PredicateNode<E, R_ARITY>,
        map_right: ArgumentMap<C_ARITY, R_ARITY>,
    ) -> PredicateNode<E, C_ARITY> {
        let sig: u64 = rand::random();

        let conjunction_node = PredicateNode::new(Box::new(Self {
            left: left.clone(),
            map_left,
            map_right,
            right: right.clone(),
            sig,
        }));

        left.replace(|inner| {
            Box::new(IsConjunctionPart {
                for_conjunction: conjunction_node.clone(),
                inner,
                map_this: map_left,
                map_other: map_right,
                sig,
                with: right.clone(),
            })
        });

        right.replace(|inner| {
            Box::new(IsConjunctionPart {
                for_conjunction: conjunction_node.clone(),
                inner,
                map_this: map_right,
                map_other: map_left,
                sig,
                with: left.clone(),
            })
        });

        conjunction_node
    }
}

impl<E: Hash + Eq + Clone, const L_ARITY: usize, const R_ARITY: usize, const C_ARITY: usize>
    Predicate<E, C_ARITY> for Conjunction<E, L_ARITY, R_ARITY, C_ARITY>
{
    fn call_for_elements(
        &self,
        element_nodes: &Arguments<ElementQuantifier<E>, C_ARITY>,
        sig: &mut GraphTraversalSignature,
    ) -> crate::TruthValue {
        if sig.contains(&self.sig) {
            TruthValue::Undetermined
        } else {
            sig.push(self.sig);

            let args_for_left = self.map_left.forward(element_nodes);
            let args_for_right = self.map_right.forward(element_nodes);

            self.left.call_for_elements(&args_for_left, sig)
                & self.right.call_for_elements(&args_for_right, sig)
        }
    }

    fn get_elements_for_true(&self) -> Vec<Arguments<ElementSet<E>, C_ARITY>> {
        let left_trues = self
            .left
            .get_elements_for_true()
            .into_iter()
            .map(|a| self.map_left.backward(&a, ElementSet::<E>::All));

        let right_trues: Vec<Arguments<ElementSet<E>, C_ARITY>> = self
            .right
            .get_elements_for_true()
            .into_iter()
            .map(|a| self.map_right.backward(&a, ElementSet::<E>::All))
            .collect();

        left_trues
            .flat_map(|l| right_trues.iter().map(move |r| (l.clone(), r.clone())))
            .map(|(mut l, r)| {
                l &= r;
                l
            })
            .collect()
    }

    fn get_elements_for_false(&self) -> Vec<Arguments<ElementSet<E>, C_ARITY>> {
        let left_falses = self
            .left
            .get_elements_for_false()
            .into_iter()
            .map(|a| self.map_left.backward(&a, ElementSet::<E>::All));

        let right_falses: Vec<Arguments<ElementSet<E>, C_ARITY>> = self
            .right
            .get_elements_for_false()
            .into_iter()
            .map(|a| self.map_right.backward(&a, ElementSet::<E>::All))
            .collect();

        left_falses
            .flat_map(|l| right_falses.iter().map(move |r| (l.clone(), r.clone())))
            .map(|(mut l, r)| {
                l |= r;
                l
            })
            .collect()
    }
}

////////////////////////////////////////////////////////////////////////////////
// Conjunction Part
////////////////////////////////////////////////////////////////////////////////

/// Added to conjunction operands to link them to their parent conjunction.
struct IsConjunctionPart<E, const WITH_ARITY: usize, const C_ARITY: usize, const THIS_ARITY: usize>
{
    pub(self) for_conjunction: PredicateNode<E, C_ARITY>,
    pub(self) inner: Box<dyn Predicate<E, THIS_ARITY>>,
    pub(self) map_other: ArgumentMap<C_ARITY, WITH_ARITY>,
    pub(self) map_this: ArgumentMap<C_ARITY, THIS_ARITY>,
    pub(self) sig: u64,
    pub(self) with: PredicateNode<E, WITH_ARITY>,
}

impl<E: Hash + Eq + Clone, const WITH_ARITY: usize, const C_ARITY: usize, const ARITY: usize>
    Predicate<E, ARITY> for IsConjunctionPart<E, WITH_ARITY, C_ARITY, ARITY>
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

            let mapped_args_for_conjunction = self
                .map_this
                .backward(element_nodes, ElementQuantifier::Any);

            let mapped_args_for_other_conjunction_part =
                self.map_other.forward(&mapped_args_for_conjunction);

            match (
                self.for_conjunction
                    .call_for_elements(&mapped_args_for_conjunction, sig),
                self.with
                    .call_for_elements(&mapped_args_for_other_conjunction_part, sig),
            ) {
                (TruthValue::Determined(true), _) => TruthValue::Determined(true),

                (TruthValue::Determined(false), TruthValue::Determined(true)) => {
                    TruthValue::Determined(false)
                }

                _ => TruthValue::Undetermined,
            }
        }
    }

    fn get_elements_for_false(&self) -> Vec<Arguments<ElementSet<E>, ARITY>> {
        todo!()
    }

    fn get_elements_for_true(&self) -> Vec<Arguments<ElementSet<E>, ARITY>> {
        todo!()
    }
}
