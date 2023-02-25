use super::predicates::Undetermined;
use crate::primitives::TruthValue;
use crate::ElementValue;
use std::fmt::Debug;
use std::{cell::UnsafeCell, rc::Rc};

use super::{Arguments, ElementQuantifier, ElementSet};

/// A signature for preventing cycles in graph traversals.
///
/// Used by implementors of [Predicate] and [Function] to prevent cycles during
/// graph traversals. The FOL graph will, in general, be highly cyclic.
pub type GraphTraversalSignature = Vec<u64>; // Could this be a private member?

////////////////////////////////////////////////////////////////////////////////
// Base Traits
////////////////////////////////////////////////////////////////////////////////

/// A type which can be applied to elements to produce new elements.
pub trait Function<E: Eq, const ARITY: usize> {
    /// Call the function with the specified args.
    fn call_for_args(
        &self,
        args: &Arguments<ElementQuantifier<E>, ARITY>,
        sig: &mut GraphTraversalSignature,
    ) -> ElementValue<&E>;
}

/// A type which can be called on elements to return boolean (truth) values.
pub trait Predicate<E, const ARITY: usize> {
    /// Call the predicate with the specified args.
    fn call_for_elements(
        &self,
        arguments: &Arguments<ElementQuantifier<E>, ARITY>,
        sig: &mut GraphTraversalSignature,
    ) -> TruthValue;

    /// Get a set of arguments for which the predicate is known to be true.
    ///
    /// Note: An empty array does **not** mean that the predicate is false for
    /// all sets of arguments, it simply means that there is not knowledge of a
    /// specific set of arguments for which the predicate is definitely true.
    fn get_elements_for_true(
        &self,
        sig: &mut GraphTraversalSignature,
    ) -> Vec<Arguments<ElementSet<E>, ARITY>>;

    /// Get a set of arguments for which the predicate is known to be false.
    ///
    /// See [Self::get_elements_for_true()] for specifics.
    fn get_elements_for_false(
        &self,
        sig: &mut GraphTraversalSignature,
    ) -> Vec<Arguments<ElementSet<E>, ARITY>>;
}

////////////////////////////////////////////////////////////////////////////////
// Graph Node
////////////////////////////////////////////////////////////////////////////////

/// The underlying type for [`PredicateNode`] and [`FunctionNode`].
///
/// A wrapper for `RawGraphNode` which has the `Rc` property baked into it.
#[derive(Debug)]
pub struct GraphNode<T> {
    _uid: u64,
    _raw: Rc<RawGraphNode<T>>,
}

impl<T> Clone for GraphNode<T> {
    fn clone(&self) -> Self {
        Self {
            _uid: self._uid,
            _raw: self._raw.clone(),
        }
    }
}

impl<T> PartialEq for GraphNode<T> {
    fn eq(&self, other: &Self) -> bool {
        self._uid == other._uid
    }
}

impl<T> Eq for GraphNode<T> {}

impl<T> GraphNode<T> {
    /// Create a graph node from an inner type.
    pub fn new(inner: T) -> Self {
        Self {
            _uid: rand::random(),
            _raw: Rc::new(RawGraphNode::new(inner)),
        }
    }

    pub(super) unsafe fn get_inner_as_ref(&self) -> &T {
        self._raw.get_inner_as_ref()
    }
}

impl<'a, T: 'a> GraphNode<T> {
    /// Replace the inner value based on the old value.
    ///
    /// TODO: Test safety
    pub fn replace<Callback>(&self, creator: Callback)
    where
        Callback: Fn(T) -> T,
    {
        self._raw.replace(creator)
    }
}

impl<T> From<T> for GraphNode<T> {
    fn from(t: T) -> Self {
        Self {
            _uid: rand::random(),
            _raw: Rc::new(RawGraphNode::new(t)),
        }
    }
}

// The raw internal cell of a graph node. This type is unsafe when used by
// itself.
#[derive(Debug)]
struct RawGraphNode<T> {
    _raw: UnsafeCell<T>,
}

impl<T> RawGraphNode<T> {
    pub fn new(inner: T) -> Self {
        Self {
            _raw: UnsafeCell::new(inner),
        }
    }

    pub(super) unsafe fn get_inner_as_ref(&self) -> &T {
        &*self._raw.get()
    }
}

impl<'a, T: 'a> RawGraphNode<T> {
    /// SAFETY: We wrap the old inner with the new inner so that there is no
    /// duplication. We use a creator callback so that we never expose a
    /// dangling reference to the internal type. The use of `Fn` ensures that
    /// the callback cannot expose the internal struct to external code.
    pub fn replace<Callback>(&self, creator: Callback)
    where
        Callback: Fn(T) -> T,
    {
        // TODO: Is this safe with the bound as an Fn?
        // I am fairly certain it is unsafe as FnOnce or FnMut
        // Maybe a fn(T) -> T would be better?
        unsafe {
            let inner_mut: &mut T = &mut *self._raw.get();
            let inner: T = std::ptr::read(inner_mut);
            let new: T = creator(inner);
            std::ptr::write(inner_mut, new);
        }
    }
}

impl<T> From<T> for RawGraphNode<T> {
    fn from(t: T) -> Self {
        Self {
            _raw: UnsafeCell::new(t),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Predicate Node
////////////////////////////////////////////////////////////////////////////////

/// A graph node representing a predicate.
///
/// Implements the [`Predicate`] trait itself, to call underlying predicate
/// layers.
///
/// # Examples
///
/// ```
/// # use first_order_logic::semantics::PredicateNode;
/// // A binary predicate
/// let predicate_a: PredicateNode<(), 2> = PredicateNode::default();
/// ```
pub type PredicateNode<E, const ARITY: usize> = GraphNode<Box<dyn Predicate<E, ARITY>>>;

impl<E, const ARITY: usize> Predicate<E, ARITY> for PredicateNode<E, ARITY> {
    fn call_for_elements(
        &self,
        element_nodes: &Arguments<ElementQuantifier<E>, ARITY>,
        sig: &mut GraphTraversalSignature,
    ) -> TruthValue {
        let inner: &dyn Predicate<E, ARITY> = unsafe { self.get_inner_as_ref() }.as_ref();
        inner.call_for_elements(element_nodes, sig)
    }

    fn get_elements_for_true(
        &self,
        sig: &mut GraphTraversalSignature,
    ) -> Vec<Arguments<ElementSet<E>, ARITY>> {
        // Safety: We use unsafe code to get a reference to the inner element,
        // but we immediately consume the reference, and return a value which
        // doesn't reference the inner element.
        let inner: &dyn Predicate<E, ARITY> = unsafe { self.get_inner_as_ref() }.as_ref();
        inner.get_elements_for_true(sig)
    }

    fn get_elements_for_false(
        &self,
        sig: &mut GraphTraversalSignature,
    ) -> Vec<Arguments<ElementSet<E>, ARITY>> {
        // Safety: We use unsafe code to get a reference to the inner element,
        // but we immediately consume the reference, and return a value which
        // doesn't reference the inner element.
        let inner: &dyn Predicate<E, ARITY> = unsafe { self.get_inner_as_ref() }.as_ref();
        inner.get_elements_for_false(sig)
    }
}

impl<E: Clone, const ARITY: usize> Default for PredicateNode<E, ARITY> {
    fn default() -> Self {
        Self::new(Box::new(Undetermined()))
    }
}

////////////////////////////////////////////////////////////////////////////////
// Function Node
////////////////////////////////////////////////////////////////////////////////

/// A graph node representing a function.
///
/// Implements the [`Function`] trait itself, to call underlying predicate
/// layers.
pub type FunctionNode<E, const ARITY: usize> = GraphNode<Box<dyn Function<E, ARITY>>>;

impl<E: Eq, const ARITY: usize> Function<E, ARITY> for FunctionNode<E, ARITY> {
    fn call_for_args(
        &self,
        args: &Arguments<ElementQuantifier<E>, ARITY>,
        sig: &mut GraphTraversalSignature,
    ) -> ElementValue<&E> {
        // Safety: We use unsafe code to get a reference to the inner element,
        // but we immediately consume the reference, and return a value which
        // doesn't reference the inner element.
        let inner = unsafe { self.get_inner_as_ref() }.as_ref();
        inner.call_for_args(args, sig)
    }
}
