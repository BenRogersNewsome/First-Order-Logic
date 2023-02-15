use std::fmt::Display;

use crate::syntax::{grammar::terms::Variable, Replace};


/// A syntax node for a predicate call.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PredicateCall<T> {
    /// The label of the predicate being called.
    pub predicate: u64,
    /// An array of terms to call the predicate with.
    pub terms: Vec<T>,
}

impl<T: Replace> Replace for PredicateCall<T> {
    fn replace(&mut self, old: Variable, right: Variable) {
        self.terms.iter_mut().for_each(|t| t.replace(old, right));
    }
}

impl<T: Display> Display for PredicateCall<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match u8::try_from(self.predicate) {
            Ok(x) => {
                f.write_fmt(format_args!("{}", std::char::from_u32(x as u32).unwrap()))?;
            }
            Err(_) => {
                f.write_fmt(format_args!("{}", self.predicate))?;
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
