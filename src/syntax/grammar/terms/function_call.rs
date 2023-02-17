use std::fmt::Display;

use crate::syntax::Replace;

use super::variable::Variable;

/// A syntax node for a function call, such as `f(x, y)`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionCall<T> {
    /// The label of the function - the 'f' in `f(x, y)`
    pub function: u64,
    /// The terms to call the function with - 'x' and 'y' in `f(x, y)`
    pub terms: Vec<T>,
}

impl<T: Replace> Replace for FunctionCall<T> {
    fn replace(&mut self, old: Variable, right: Variable) {
        self.terms.iter_mut().for_each(|t| t.replace(old, right));
    }
}

impl<T: Display> Display for FunctionCall<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match u8::try_from(self.function) {
            Ok(x) => {
                f.write_fmt(format_args!("{}", std::char::from_u32(x as u32).unwrap()))?;
            }
            Err(_) => {
                f.write_fmt(format_args!("{}", self.function))?;
            }
        };
        f.write_str("(")?;

        let mut first = true;
        for term in &self.terms {
            if !first {
                f.write_str(",")?;
            };
            first = false;
            term.fmt(f)?;
        }

        f.write_str(")")?;
        Ok(())
    }
}
