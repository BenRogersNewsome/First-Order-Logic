use crate::{TruthValue, Element, Predicate, graph::{GraphTraversalSignature, ElementCollection}};

use super::super::PredicateNode;


/// Represents the implication `Implication(e) <=> ( Left(e) => Right(e) )`
#[derive(Debug)]
pub struct Implication<E: Element> {
    left: PredicateNode<E>,
    right: PredicateNode<E>,

    sig: u64,
}

impl<E: 'static + Element> Predicate<E> for Implication<E> {
    fn call_for_element(&self, element_node: &E, sig: &mut GraphTraversalSignature) -> TruthValue {
        use TruthValue::*;
        if sig.contains(&self.sig) {
            self._call_for_element_naive(element_node, sig)
        } else {
            sig.push(self.sig);
            match (
                self.left.call_for_element(element_node, sig),
                self.right.call_for_element(element_node, sig)
            ) {
                (Determined(true), Determined(true)) |
                (Determined(false), _)   | 
                (_, Determined(true)) => Determined(true),
                (Determined(true), Determined(false)) => Determined(false),
                (Determined(true), Undetermined) => Undetermined,
                (Undetermined, Determined(false)) => Undetermined,
                (Undetermined, Undetermined) => Undetermined,
            }
        }
    }

    fn get_elements_for_false(&self, sig: &mut GraphTraversalSignature) -> ElementCollection<E> {
        if sig.contains(&self.sig) {
            return self._get_elements_for_false_naive(sig)
        } else {
            sig.push(self.sig);
            ElementCollection::intersection(
                self.left.get_elements_for_true(sig),
                self.right.get_elements_for_false(sig),
            )
        }
    }

    fn get_elements_for_true(&self, sig: &mut GraphTraversalSignature) -> ElementCollection<E> {
        if sig.contains(&self.sig) {
            return self._get_elements_for_true_naive(sig)
        } else {
            sig.push(self.sig);
            ElementCollection::union(vec![
                self.left.get_elements_for_false(sig),
                ElementCollection::intersection(
                    self.left.get_elements_for_true(sig),
                    self.right.get_elements_for_true(sig),
                )
            ].into_iter())
        }
    }
}

impl<E: 'static + Element> Implication<E> {
    pub fn new(left: &PredicateNode<E>, right: &PredicateNode<E>) -> PredicateNode<E> {
        let sig: u64 = rand::random();

        let implication_predicate_node = PredicateNode::new(Box::new(Self {
            left: left.clone(),
            right: right.clone(),
            sig,
        }));

        left.replace(|inner| {
            Box::new(Implies {
                implies: right.clone(),
                when_true: implication_predicate_node.clone(),
                inner,
                sig,
            })
        });

        right.replace(|inner| {
            Box::new(ImpliedBy {
                implied_by: left.clone(),
                when_true: implication_predicate_node.clone(),
                inner,
                sig,
            })
        });

        implication_predicate_node
    }

    fn _call_for_element_naive(&self, element_node: &E, sig: &mut GraphTraversalSignature) -> TruthValue {
        TruthValue::Undetermined
    }

    fn _get_elements_for_false_naive(&self, sig: &mut GraphTraversalSignature) -> ElementCollection<E> {
        ElementCollection::from(vec![])
    }

    fn _get_elements_for_true_naive(&self, sig: &mut GraphTraversalSignature) -> ElementCollection<E> {
        ElementCollection::from(vec![])
    }
}

/// Predicate which states that this predicate implies another predicate when a
/// third predicate is true. In other words, the left part of the implication:
/// A => B.
#[derive(Debug)]
pub struct Implies<E: Element> {
    implies: PredicateNode<E>, when_true: PredicateNode<E>,
    inner: Box<dyn Predicate<E>>,

    sig: u64,
}

impl<E: Element> Predicate<E> for Implies<E> {
    fn call_for_element(&self, element_node: &E, sig: &mut GraphTraversalSignature) -> TruthValue {
        self.inner.call_for_element(&element_node, sig)
    }

    fn get_elements_for_false(&self, sig: &mut GraphTraversalSignature) -> ElementCollection<E> {
        self.inner.get_elements_for_false(sig)
    }

    fn get_elements_for_true(&self, sig: &mut GraphTraversalSignature) -> ElementCollection<E> {
        self.inner.get_elements_for_true(sig)
    }
}

/// Predicate which states that this predicate is implied by another predicate
/// when a third predicate is true. In other words, the right part of the
/// implication: A => B.
#[derive(Debug)]
pub struct ImpliedBy<E: Element> {
    implied_by: PredicateNode<E>, when_true: PredicateNode<E>,
    inner: Box<dyn Predicate<E>>,

    sig: u64,
}

impl<E: Element> Predicate<E> for ImpliedBy<E> {
    fn call_for_element(&self, element_node: &E, sig: &mut GraphTraversalSignature) -> TruthValue {
        if sig.contains(&self.sig) {
            self._call_for_element_naive(element_node, sig)
        } else {
            sig.push(self.sig);
            if let TruthValue::Determined(b) = self.inner.call_for_element(&element_node, sig) {
                TruthValue::Determined(b)
            } else if let TruthValue::Determined(true) = self.when_true.call_for_element(&element_node, sig){
                match self.implied_by.call_for_element(&element_node, sig) {
                    TruthValue::Determined(true) => TruthValue::Determined(true),
                    TruthValue::Determined(false) => TruthValue::Undetermined,
                    TruthValue::Undetermined => TruthValue::Undetermined,
                }
            } else {
                TruthValue::Undetermined
            }
        }
    }

    fn get_elements_for_false(&self, sig: &mut GraphTraversalSignature) -> ElementCollection<E> {
        self.inner.get_elements_for_false(sig)
    }
    
    fn get_elements_for_true(&self, sig: &mut GraphTraversalSignature) -> ElementCollection<E> {
        if sig.contains(&self.sig) {
            self._get_elements_for_true_naive(sig)
        } else {
            sig.push(self.sig);
            ElementCollection::union(vec![
                ElementCollection::intersection(
                    self.implied_by.get_elements_for_true(sig),
                    self.when_true.get_elements_for_true(sig),
                ),
                ElementCollection::intersection(
                    self.implied_by.get_elements_for_false(sig),
                    self.when_true.get_elements_for_true(sig),
                ),
                ElementCollection::intersection(
                    self.implied_by.get_elements_for_false(sig),
                    self.when_true.get_elements_for_false(sig),
                ),
            ].into_iter())
        }
    }
}

impl<E: Element> ImpliedBy<E> {
    fn _call_for_element_naive(&self, element_node: &E, sig: &mut GraphTraversalSignature) -> TruthValue {
        self.inner.call_for_element(element_node, sig)
    }

    fn _get_elements_for_true_naive(&self, sig: &mut GraphTraversalSignature) -> ElementCollection<E> {
        self.inner.get_elements_for_true(sig)
    }
}