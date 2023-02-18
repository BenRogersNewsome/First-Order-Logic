use crate::semantics::Predicate;

/// Create a predicate which is linked to some external node.
///
/// For two predicate nodes, this assertion essentially asserts a one-way
/// equivalence between the nodes. Asserting a two-way equivalence can be done
/// by manually implementing this predicate on both sides of the equality.
pub struct LinkedPredicate<E, const ARITY: usize> {
    linked: Box<dyn Predicate<E, ARITY>>,
}

impl<E, const ARITY: usize> Predicate<E, ARITY> for LinkedPredicate<E, ARITY> {
    fn call_for_elements(
        &self,
        arguments: &crate::semantics::elements::Arguments<
            crate::semantics::elements::ElementQuantifier<E>,
            ARITY,
        >,
        sig: &mut crate::semantics::GraphTraversalSignature,
    ) -> crate::TruthValue {
        self.linked.call_for_elements(arguments, sig)
    }

    fn get_elements_for_false(
        &self,
    ) -> Vec<crate::semantics::elements::Arguments<crate::semantics::elements::ElementSet<E>, ARITY>>
    {
        self.linked.get_elements_for_false()
    }

    fn get_elements_for_true(
        &self,
    ) -> Vec<crate::semantics::elements::Arguments<crate::semantics::elements::ElementSet<E>, ARITY>>
    {
        self.linked.get_elements_for_true()
    }
}

impl<E, const ARITY: usize> From<Box<dyn Predicate<E, ARITY>>> for LinkedPredicate<E, ARITY> {
    fn from(linked: Box<dyn Predicate<E, ARITY>>) -> Self {
        Self { linked }
    }
}
