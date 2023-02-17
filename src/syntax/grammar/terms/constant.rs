use std::fmt::Display;

use crate::syntax::Replace;

use super::variable::Variable;

/// A syntax node for a constant.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Constant {
    /// The label of the constant.
    pub label: u64,
}

impl Replace for Constant {
    #[inline]
    fn replace(&mut self, _: Variable, _: Variable) {}
}

impl Display for Constant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.label))?;
        Ok(())
    }
}
