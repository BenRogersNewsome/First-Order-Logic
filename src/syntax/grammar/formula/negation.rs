use std::fmt::Display;

use crate::syntax::{grammar::terms::Variable, Replace};

/// A syntax node for the negation of a formula
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Negation<F> {
    /// The formula to negate
    pub right: F,
}

impl<F: Replace> Replace for Negation<F> {
    fn replace(&mut self, old: Variable, right: Variable) {
        self.right.replace(old, right);
    }
}

impl<F: Display> Display for Negation<F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("¬")?;
        f.write_str("(")?;
        self.right.fmt(f)?;
        f.write_str(")")?;
        Ok(())
    }
}
