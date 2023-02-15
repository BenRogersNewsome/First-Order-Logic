use std::fmt::Display;

use crate::syntax::Replace;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Variable {
    pub label: u64,
}

impl Variable {
    pub fn new(label: u64) -> Self {
        Self::from(label)
    }

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
