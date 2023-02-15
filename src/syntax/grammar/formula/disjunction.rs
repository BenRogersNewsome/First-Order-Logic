use std::fmt::Display;

use crate::syntax::{grammar::terms::Variable, Replace};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Disjunction<Left, Right> {
    pub left: Left,
    pub right: Right,
}

impl<L: Replace, R: Replace> Replace for Disjunction<L, R> {
    fn replace(&mut self, old: Variable, right: Variable) {
        self.left.replace(old, right);
        self.right.replace(old, right);
    }
}

impl<Left: Display, Right: Display> Display for Disjunction<Left, Right> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("(")?;
        self.left.fmt(f)?;
        f.write_str(")")?;
        f.write_str("∨")?;
        f.write_str("(")?;
        self.right.fmt(f)?;
        f.write_str(")")?;
        Ok(())
    }
}
