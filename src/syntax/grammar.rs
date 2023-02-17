mod atomic {
    use std::fmt::{Debug, Display};

    use super::{
        terms::{GenericTerm, Variable},
        Replace,
    };

    mod equality;
    mod predicate_call;

    pub use equality::Equality;
    pub use predicate_call::PredicateCall;

    /// A dynamically-typed atomic formula.
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum GenericAtomicFormula {
        /// An equality, such as x = y
        Equality(Equality<GenericTerm, GenericTerm>),
        /// A predicate call, such as `P(x)`
        Predicate(PredicateCall<GenericTerm>),
        /// A true literal, corresponding to the logical symbol `⊤`
        True,
        /// A false literal, corresponding to the logical symbol `⊥`
        False,
    }

    impl From<PredicateCall<GenericTerm>> for GenericAtomicFormula {
        fn from(p: PredicateCall<GenericTerm>) -> Self {
            Self::Predicate(p)
        }
    }

    impl From<Equality<GenericTerm, GenericTerm>> for GenericAtomicFormula {
        fn from(x: Equality<GenericTerm, GenericTerm>) -> Self {
            Self::Equality(x)
        }
    }

    impl From<bool> for GenericAtomicFormula {
        fn from(p: bool) -> Self {
            if p {
                Self::True
            } else {
                Self::False
            }
        }
    }

    impl Replace for GenericAtomicFormula {
        fn replace(&mut self, old: Variable, right: Variable) {
            match self {
                Self::Equality(e) => e.replace(old, right),
                Self::Predicate(p) => p.replace(old, right),
                Self::True | Self::False => {}
            }
        }
    }

    impl Display for GenericAtomicFormula {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Equality(x) => std::fmt::Display::fmt(x, f),
                Self::Predicate(x) => std::fmt::Display::fmt(x, f),
                Self::True => f.write_str("True"),
                Self::False => f.write_str("False"),
            }
        }
    }
}

mod terms {

    mod constant;
    mod function_call;
    mod variable;

    use std::fmt::Display;

    pub use constant::Constant;
    pub use function_call::FunctionCall;
    pub use variable::Variable;

    use super::Replace;

    /// A dynamically typed formula term.
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum GenericTerm {
        /// A variable
        Variable(Variable),
        /// A constant
        Constant(Constant),
        /// A call to a function, such as `f(x, y)`
        FunctionCall(Box<FunctionCall<GenericTerm>>),
    }

    impl Replace for GenericTerm {
        fn replace(&mut self, old: Variable, right: Variable) {
            match self {
                Self::Variable(v) => v.replace(old, right),
                Self::Constant(c) => c.replace(old, right),
                Self::FunctionCall(f) => f.replace(old, right),
            }
        }
    }

    impl From<Variable> for GenericTerm {
        fn from(v: Variable) -> Self {
            Self::Variable(v)
        }
    }

    impl From<Constant> for GenericTerm {
        fn from(v: Constant) -> Self {
            Self::Constant(v)
        }
    }

    impl From<FunctionCall<GenericTerm>> for GenericTerm {
        fn from(v: FunctionCall<GenericTerm>) -> Self {
            Self::FunctionCall(Box::new(v))
        }
    }

    impl Display for GenericTerm {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Constant(x) => x.fmt(f),
                Self::Variable(x) => x.fmt(f),
                Self::FunctionCall(x) => x.fmt(f),
            }
        }
    }
}

mod formula {
    use std::fmt::Display;

    use super::{atomic::GenericAtomicFormula, terms::Variable, Replace};

    mod biconditional;
    mod conjunction;
    mod disjunction;
    mod existential;
    mod implication;
    mod negation;
    mod universal;

    pub use biconditional::Biconditional;
    pub use conjunction::Conjunction;
    pub use disjunction::Disjunction;
    pub use existential::Existential;
    pub use implication::Implication;
    pub use negation::Negation;
    pub use universal::Universal;

    /// A dynamically typed FOL formula.
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum GenericFormula {
        /// An atomic formula
        Atomic(GenericAtomicFormula),

        /// A universal quantifier, such as `∀x.P(x)`
        Universal(Box<Universal<GenericFormula>>),
        /// An existential quantifier, such as `∃x.P(x)`
        Existential(Box<Existential<GenericFormula>>),

        /// A conjunction such as `P ∧ Q`
        Conjunction(Box<Conjunction<GenericFormula, GenericFormula>>),
        /// A disjunction such as `P ∨ Q`
        Disjunction(Box<Disjunction<GenericFormula, GenericFormula>>),
        /// An implication such as `P -> Q`
        Implication(Box<Implication<GenericFormula, GenericFormula>>),
        /// An negation such as `¬P`
        Negation(Box<Negation<GenericFormula>>),
    }

    impl From<Universal<GenericFormula>> for GenericFormula {
        fn from(inner: Universal<GenericFormula>) -> Self {
            Self::Universal(Box::new(inner))
        }
    }

    impl From<Existential<GenericFormula>> for GenericFormula {
        fn from(inner: Existential<GenericFormula>) -> Self {
            Self::Existential(Box::new(inner))
        }
    }

    // impl From<Biconditional<GenericFormula, GenericFormula>> for GenericFormula {
    //     fn from(inner: Biconditional<GenericFormula, GenericFormula>) -> Self {
    //         Self::Biconditional(Box::new(inner))
    //     }
    // }

    impl From<Conjunction<GenericFormula, GenericFormula>> for GenericFormula {
        fn from(inner: Conjunction<GenericFormula, GenericFormula>) -> Self {
            Self::Conjunction(Box::new(inner))
        }
    }

    impl From<Disjunction<GenericFormula, GenericFormula>> for GenericFormula {
        fn from(inner: Disjunction<GenericFormula, GenericFormula>) -> Self {
            Self::Disjunction(Box::new(inner))
        }
    }

    impl From<Implication<GenericFormula, GenericFormula>> for GenericFormula {
        fn from(inner: Implication<GenericFormula, GenericFormula>) -> Self {
            Self::Implication(Box::new(inner))
        }
    }

    impl From<Negation<GenericFormula>> for GenericFormula {
        fn from(inner: Negation<GenericFormula>) -> Self {
            Self::Negation(Box::new(inner))
        }
    }

    impl<T: Into<GenericAtomicFormula>> From<T> for GenericFormula {
        fn from(t: T) -> Self {
            Self::Atomic(t.into())
        }
    }

    impl Replace for GenericFormula {
        fn replace(&mut self, old: Variable, right: Variable) {
            match self {
                Self::Atomic(x) => x.replace(old, right),
                Self::Conjunction(x) => x.replace(old, right),
                Self::Disjunction(x) => x.replace(old, right),
                Self::Existential(x) => x.replace(old, right),
                Self::Implication(x) => x.replace(old, right),
                Self::Negation(x) => x.replace(old, right),
                Self::Universal(x) => x.replace(old, right),
            };
        }
    }

    impl Display for GenericFormula {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Atomic(x) => x.fmt(f),
                Self::Conjunction(x) => x.fmt(f),
                Self::Disjunction(x) => x.fmt(f),
                Self::Existential(x) => x.fmt(f),
                Self::Implication(x) => x.fmt(f),
                Self::Negation(x) => x.fmt(f),
                Self::Universal(x) => x.fmt(f),
            }
        }
    }
}

pub use atomic::*;
pub use formula::*;
pub use terms::*;

/// A trait for allowing formula to replace-in-place variables.
pub trait Replace {
    /// Perform the replacement, in-place.
    fn replace(&mut self, old: Variable, right: Variable);
}
