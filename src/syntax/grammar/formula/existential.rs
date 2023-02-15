use crate::syntax::{Replace, Variable};
use std::fmt::{Debug, Display};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Existential<Right> {
    pub left: Variable,
    pub right: Right,
}

impl<F: Replace> Replace for Existential<F> {
    fn replace(&mut self, old: Variable, right: Variable) {
        self.left = right;
        self.right.replace(old, right);
    }
}

impl<R: Display> Display for Existential<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("∃")?;
        std::fmt::Display::fmt(&self.left, f)?;
        f.write_str(&".")?;
        self.right.fmt(f)?;
        Ok(())
    }
}
