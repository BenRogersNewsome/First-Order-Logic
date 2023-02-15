use std::fmt::Display;

use crate::syntax::{grammar::terms::Variable, Replace};


/// A syntax node for an equality between to other nodes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Equality<Left, Right> {
    /// The left item of the equality
    pub left: Left,
    /// The right item of the equality
    pub right: Right,
}

impl<Left: Replace, Right: Replace> Replace for Equality<Left, Right> {
    fn replace(&mut self, old: Variable, right: Variable) {
        self.left.replace(old, right);
        self.right.replace(old, right);
    }
}

impl<Left: Display, Right: Display> Display for Equality<Left, Right> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.left.fmt(f)?;
        f.write_str("=")?;
        self.right.fmt(f)?;
        Ok(())
    }
}
