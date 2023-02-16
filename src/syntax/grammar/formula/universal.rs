use crate::syntax::{Replace, Variable};
use std::fmt::{Debug, Display};

/// A syntax node for the a universal quantifier
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Universal<Right> {
    /// The variable being quantified
    pub left: Variable,
    /// The formula for which the quantifier applies
    pub right: Right,
}

impl<F: Replace> Replace for Universal<F> {
    fn replace(&mut self, old: Variable, right: Variable) {
        self.left = right;
        self.right.replace(old, right);
    }
}

impl<R: Display> Display for Universal<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("∀")?;
        std::fmt::Display::fmt(&self.left, f)?;
        f.write_str(".")?;
        self.right.fmt(f)?;
        Ok(())
    }
}
