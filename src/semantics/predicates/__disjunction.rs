use super::{Predicate, PredicateNode};

pub struct Disjunction {
    // TODO: These should be weak references so that they don't hold onto
    // dangling references to dropped values. Same should be true of other
    // relationships in graph.
    left: PredicateNode,
    right: PredicateNode,
}

impl Predicate for Disjunction {
    fn call_for_element(&self, element_node: &crate::graph::set::SetNode) -> TruthValue {
        self.left.call_for_element(element_node) | self.right.call_for_element(element_node)
    }

    fn get_elements_for_false(&self) -> Vec<&crate::graph::set::SetNode> {
        todo!()
    }

    fn get_elements_for_true(&self) -> Vec<&crate::graph::set::SetNode> {
        todo!()
    }
}

impl Disjunction {
    pub fn new(left: &PredicateNode, right: &PredicateNode) -> PredicateNode {
        let disjunction_node = PredicateNode::new(
            Box::new(
                Self {
                    left: left.clone(),
                    right: right.clone(),
                }
            )
        );

        left.replace(|old| {
            Box::new(
                Disjuncted {
                    disjunction: disjunction_node.clone(),
                    with: right.clone(),
                    inner: old,
                }
            )
        });

        right.replace(|old| {
            Box::new(
                Disjuncted {
                    disjunction: disjunction_node.clone(),
                    with: left.clone(),
                    inner: old,
                }
            )
        });

        disjunction_node
    }
}

pub struct Disjuncted {
    pub(self) disjunction: PredicateNode,
    pub(self) with: PredicateNode,
    pub(self) inner: Box<dyn Predicate>,
}

impl Predicate for Disjuncted {
    fn call_for_element(&self, element_node: &crate::graph::set::SetNode) -> TruthValue {
        self.inner.call_for_element(element_node) | (
            self.disjunction.call_for_element(element_node) &
            !self.with.call_for_element(element_node)
        )
    }

    fn get_elements_for_false(&self) -> Vec<&crate::graph::set::SetNode> {
        todo!()
    }

    fn get_elements_for_true(&self) -> Vec<&crate::graph::set::SetNode> {
        todo!()
    }
}