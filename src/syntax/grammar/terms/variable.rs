use std::fmt::Display;

use crate::syntax::Replace;

/// A syntax node for a variable
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Variable {
    /// The u64 label of the variable.
    pub label: u64,
}

impl Variable {
    /// Create a new variable from a u64
    /// # Examples
    /// 
    /// ```
    /// # use first_order_logic::syntax::grammer::Variable;
    /// let var_x: Variable = Variable::new(b"x".into());
    /// ```
    pub fn new(label: u64) -> Self {
        Self::from(label)
    }

    /// Create a variable with a random label. Useful for variable replacements
    /// in when combining formula, to prevent variable collisions.
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use first_order_logic::syntax::grammer::Variable;
    /// let var_x: Variable = Variable::rand();
    /// ```
    pub fn rand() -> Self {
        Self::new(rand::random())
    }
}

impl From<u64> for Variable {
    fn from(label: u64) -> Self {
        Self { label }
    }
}

impl Replace for Variable {
    fn replace(&mut self, old: Variable, right: Variable) {
        if old == *self {
            self.label = right.label;
        };
    }
}

impl Display for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match u8::try_from(self.label) {
            Ok(x) => f.write_fmt(format_args!("{}", std::char::from_u32(x as u32).unwrap())),
            Err(_) => f.write_fmt(format_args!("{}", self.label)),
        }
    }
}
