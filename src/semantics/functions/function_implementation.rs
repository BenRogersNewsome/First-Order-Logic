use crate::{Element, Function};

use super::super::FunctionNode;

pub type FunctionCallable<'a, E> = fn(element: &E) -> &'a E;

/// Declare an implementation for a function using a fn pointer.
/// 
/// Return a unique element for every unique combination of input args. Function
/// must cache which elements it has given out, for which elements previously to
/// maintain consistency. Lazy load the elements on function calls and cache.
/// 
pub struct FunctionImplementation<'a, E: Element> {
    call: FunctionCallable<'a, E>,
}

impl<'a, E: Element> Function<E> for FunctionImplementation<'a, E> {
    fn call_for_element(&self, element: &E) -> &'a E {
        (self.call)(element)
    }
}

impl<'a, E: Element> FunctionImplementation<'a, E> {
    pub fn new(call: FunctionCallable<'a, E>) -> FunctionNode<E> {
        FunctionNode::new(
            Box::new(
                Self {
                    call,
                }
            )
        )
    }
}

