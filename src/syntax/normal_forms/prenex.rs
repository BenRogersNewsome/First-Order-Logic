use std::{fmt::Display, ops::Not};

use crate::syntax::{
    Conjunction, Disjunction, GenericAtomicFormula, GenericFormula, Implication, Replace, Variable,
};

use super::super::grammar::{Existential, Negation, Universal};

////////////////////////////////////////////////////////////////////////////////
// PrenexNormalFormula
////////////////////////////////////////////////////////////////////////////////

/// A formula in Prenex-Normal Form (PNF).
pub struct PrenexNormalFormula {
    /// A reverse-ordered list of the PNF's leading quantifiers.
    pub quantifiers: Vec<PrenexNormalQuantifier>,
    /// The rest of the terms in the PNF
    pub formula: PrenexNormalFormulaTerm,
}

impl PrenexNormalFormula {
    /// Replace every variable in the formula with a new randomly-named
    /// variable.
    ///
    /// This is to prevent variable collisions when quantifiers are pulled out
    /// of the formula from multiple sub-formula.
    pub fn replace_all_vars(&mut self) {
        self.quantifiers.iter_mut().for_each(|q| {
            let new_var = Variable::rand();
            self.formula.replace(q.inner(), new_var);
            q.replace(new_var)
        });
    }
}

impl Display for PrenexNormalFormula {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for q in self.quantifiers.iter().rev() {
            q.fmt(f)?;
        }
        self.formula.fmt(f)?;
        Ok(())
    }
}

////////////////////////////////////////////////////////////////////////////////
// PrenexNormalFormulaTerm
////////////////////////////////////////////////////////////////////////////////

/// Similar to [GenericFormula], without quantifiers.
pub enum PrenexNormalFormulaTerm {
    /// Atomic Formula
    Atomic(GenericAtomicFormula),
    /// A conjunction of PNF formula.
    Conjunction(Box<Conjunction<PrenexNormalFormulaTerm, PrenexNormalFormulaTerm>>),
    /// A disjunction of PNF formula.
    Disjunction(Box<Disjunction<PrenexNormalFormulaTerm, PrenexNormalFormulaTerm>>),
    /// An implication of PNF formula.
    Implication(Box<Implication<PrenexNormalFormulaTerm, PrenexNormalFormulaTerm>>),
    /// A negation of a PNF formula.
    Negation(Box<Negation<PrenexNormalFormulaTerm>>),
}

impl Replace for PrenexNormalFormulaTerm {
    fn replace(&mut self, old: Variable, right: Variable) {
        match self {
            Self::Atomic(x) => x.replace(old, right),
            Self::Conjunction(x) => x.replace(old, right),
            Self::Disjunction(x) => x.replace(old, right),
            Self::Implication(x) => x.replace(old, right),
            Self::Negation(x) => x.replace(old, right),
        }
    }
}

impl From<GenericAtomicFormula> for PrenexNormalFormulaTerm {
    fn from(f: GenericAtomicFormula) -> Self {
        Self::Atomic(f)
    }
}

#[doc(hidden)]
impl From<Conjunction<PrenexNormalFormulaTerm, PrenexNormalFormulaTerm>>
    for PrenexNormalFormulaTerm
{
    fn from(f: Conjunction<PrenexNormalFormulaTerm, PrenexNormalFormulaTerm>) -> Self {
        Self::Conjunction(Box::new(f))
    }
}

#[doc(hidden)]
impl From<Disjunction<PrenexNormalFormulaTerm, PrenexNormalFormulaTerm>>
    for PrenexNormalFormulaTerm
{
    fn from(f: Disjunction<PrenexNormalFormulaTerm, PrenexNormalFormulaTerm>) -> Self {
        Self::Disjunction(Box::new(f))
    }
}

#[doc(hidden)]
impl From<Implication<PrenexNormalFormulaTerm, PrenexNormalFormulaTerm>>
    for PrenexNormalFormulaTerm
{
    fn from(f: Implication<PrenexNormalFormulaTerm, PrenexNormalFormulaTerm>) -> Self {
        Self::Implication(Box::new(f))
    }
}

#[doc(hidden)]
impl From<Negation<PrenexNormalFormulaTerm>> for PrenexNormalFormulaTerm {
    fn from(f: Negation<PrenexNormalFormulaTerm>) -> Self {
        Self::Negation(Box::new(f))
    }
}

impl Display for PrenexNormalFormulaTerm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Atomic(x) => x.fmt(f),
            Self::Conjunction(x) => x.fmt(f),
            Self::Disjunction(x) => x.fmt(f),
            Self::Implication(x) => x.fmt(f),
            Self::Negation(x) => x.fmt(f),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// PrenexNormalFormulaQuantifier
////////////////////////////////////////////////////////////////////////////////

/// A leading quantifier in a PNF formula.
///
/// Used in [PrenexNormalFormula] in the leading quantifiers.
#[derive(Clone, Copy)]
pub enum PrenexNormalQuantifier {
    /// A universal quantifier over a variable
    Universal(Variable),
    /// An existential quantifier over a variable
    Existential(Variable),
}

impl Display for PrenexNormalQuantifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Universal(x) => {
                f.write_str("∀")?;
                x.fmt(f)?;
                f.write_str(".")?;
                Ok(())
            }
            Self::Existential(x) => {
                f.write_str("∃")?;
                x.fmt(f)?;
                f.write_str(".")?;
                Ok(())
            }
        }
    }
}

/// Turns a universal into an existential and vice-versa.
impl Not for &PrenexNormalQuantifier {
    type Output = PrenexNormalQuantifier;
    fn not(self) -> Self::Output {
        use PrenexNormalQuantifier::*;
        match self {
            Universal(v) => Existential(*v),
            Existential(v) => Universal(*v),
        }
    }
}

impl PrenexNormalQuantifier {
    /// Get the underlying variable from a quantifier.
    pub fn inner(&self) -> Variable {
        match self {
            Self::Existential(x) => *x,
            Self::Universal(x) => *x,
        }
    }

    /// Replace the variable inside a quantifier.
    pub fn replace(&mut self, var: Variable) {
        *self = match self {
            Self::Existential(_) => Self::Existential(var),
            Self::Universal(_) => Self::Universal(var),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Algorithm
////////////////////////////////////////////////////////////////////////////////

impl From<GenericFormula> for PrenexNormalFormula {
    fn from(f: GenericFormula) -> Self {
        match f {
            GenericFormula::Atomic(x) => x.into(),
            GenericFormula::Conjunction(x) => (*x).into(),
            GenericFormula::Disjunction(x) => (*x).into(),
            GenericFormula::Existential(x) => (*x).into(),
            GenericFormula::Implication(x) => (*x).into(),
            GenericFormula::Negation(x) => (*x).into(),
            GenericFormula::Universal(x) => (*x).into(),
        }
    }
}

impl From<Universal<GenericFormula>> for PrenexNormalFormula {
    fn from(f: Universal<GenericFormula>) -> Self {
        let PrenexNormalFormula {
            mut quantifiers,
            formula,
        } = f.right.into();

        quantifiers.push(PrenexNormalQuantifier::Universal(f.left));

        PrenexNormalFormula {
            quantifiers,
            formula,
        }
    }
}

impl From<Existential<GenericFormula>> for PrenexNormalFormula {
    fn from(f: Existential<GenericFormula>) -> Self {
        let PrenexNormalFormula {
            mut quantifiers,
            formula,
        } = f.right.into();

        quantifiers.push(PrenexNormalQuantifier::Existential(f.left));

        PrenexNormalFormula {
            quantifiers,
            formula,
        }
    }
}

impl From<Conjunction<GenericFormula, GenericFormula>> for PrenexNormalFormula {
    fn from(f: Conjunction<GenericFormula, GenericFormula>) -> Self {
        let left: PrenexNormalFormula = f.left.into();
        let mut right: PrenexNormalFormula = f.right.into();
        right.replace_all_vars();

        let mut combined_quantifiers: Vec<PrenexNormalQuantifier> = Vec::new();

        let left_formula = left.formula;
        let mut right_formula = right.formula;

        let mut left_quantifiers = left.quantifiers.into_iter().peekable();
        let mut right_quantifiers = right.quantifiers.into_iter().peekable();

        loop {
            use PrenexNormalQuantifier::*;
            match (left_quantifiers.peek(), right_quantifiers.peek().take()) {
                (Some(Universal(u1)), Some(Universal(u2))) => {
                    // Merge Universals in conjunction
                    right_formula.replace(*u2, *u1);
                    combined_quantifiers.push(left_quantifiers.next().unwrap());
                    right_quantifiers.next().unwrap();
                }
                (Some(Universal(_)), _) => {
                    combined_quantifiers.push(left_quantifiers.next().unwrap());
                }
                (_, Some(Universal(_))) => {
                    combined_quantifiers.push(right_quantifiers.next().unwrap());
                }
                (Some(Existential(_)), _) => {
                    combined_quantifiers.push(left_quantifiers.next().unwrap());
                }
                (_, Some(Existential(_))) => {
                    combined_quantifiers.push(right_quantifiers.next().unwrap());
                }
                (None, None) => {
                    break;
                }
            };
        }

        Self {
            quantifiers: combined_quantifiers,
            formula: Conjunction {
                left: left_formula,
                right: right_formula,
            }
            .into(),
        }
    }
}

impl From<Disjunction<GenericFormula, GenericFormula>> for PrenexNormalFormula {
    fn from(f: Disjunction<GenericFormula, GenericFormula>) -> Self {
        let left: PrenexNormalFormula = f.left.into();
        let mut right: PrenexNormalFormula = f.right.into();
        right.replace_all_vars();

        let mut combined_quantifiers: Vec<PrenexNormalQuantifier> = Vec::new();

        let left_formula = left.formula;
        let mut right_formula = right.formula;

        let mut left_quantifiers = left.quantifiers.into_iter().peekable();
        let mut right_quantifiers = right.quantifiers.into_iter().peekable();

        loop {
            use PrenexNormalQuantifier::*;
            match (left_quantifiers.peek(), right_quantifiers.peek()) {
                (Some(Existential(e1)), Some(Existential(e2))) => {
                    // Merge existentials in disjunction
                    right_formula.replace(*e2, *e1);
                    combined_quantifiers.push(left_quantifiers.next().unwrap());
                    right_quantifiers.next().unwrap();
                }
                (Some(Universal(_)), _) => {
                    combined_quantifiers.push(left_quantifiers.next().unwrap());
                }
                (_, Some(Universal(_))) => {
                    combined_quantifiers.push(right_quantifiers.next().unwrap());
                }
                (Some(Existential(_)), _) => {
                    combined_quantifiers.push(left_quantifiers.next().unwrap());
                }
                (_, Some(Existential(_))) => {
                    combined_quantifiers.push(right_quantifiers.next().unwrap());
                }
                (None, None) => {
                    break;
                }
            };
        }

        Self {
            quantifiers: combined_quantifiers,
            formula: Disjunction {
                left: left_formula,
                right: right_formula,
            }
            .into(),
        }
    }
}

impl From<Implication<GenericFormula, GenericFormula>> for PrenexNormalFormula {
    fn from(f: Implication<GenericFormula, GenericFormula>) -> Self {
        let PrenexNormalFormula {
            quantifiers: left_quantifiers,
            formula: left_formula,
        } = f.left.into();

        let mut right: PrenexNormalFormula = f.right.into();
        right.replace_all_vars();
        let PrenexNormalFormula {
            quantifiers: mut right_quantifiers,
            formula: right_formula,
        } = right;

        right_quantifiers.iter_mut().for_each(|q| *q = !&*q);

        let mut left_quantifiers_iter = left_quantifiers.into_iter().peekable();

        let mut right_quantifiers_iter = right_quantifiers.into_iter().peekable();

        let mut combined_quantifiers: Vec<PrenexNormalQuantifier> = Vec::new();

        loop {
            use PrenexNormalQuantifier::*;
            match (left_quantifiers_iter.peek(), right_quantifiers_iter.peek()) {
                (Some(Universal(_)), _) => {
                    combined_quantifiers.push(left_quantifiers_iter.next().unwrap());
                }
                (_, Some(Universal(_))) => {
                    combined_quantifiers.push(right_quantifiers_iter.next().unwrap());
                }
                (Some(Existential(_)), _) => {
                    combined_quantifiers.push(left_quantifiers_iter.next().unwrap());
                }
                (_, Some(Existential(_))) => {
                    combined_quantifiers.push(right_quantifiers_iter.next().unwrap());
                }
                (None, None) => {
                    break;
                }
            };
        }

        Self {
            quantifiers: combined_quantifiers,
            formula: Implication {
                left: left_formula,
                right: right_formula,
            }
            .into(),
        }
    }
}

impl From<Negation<GenericFormula>> for PrenexNormalFormula {
    fn from(f: Negation<GenericFormula>) -> Self {
        let right: PrenexNormalFormula = f.right.into();
        let right_formula = right.formula;
        let mut right_quantifiers = right.quantifiers;

        right_quantifiers.iter_mut().for_each(|q| *q = !&*q);

        Self {
            quantifiers: right_quantifiers,
            formula: Negation {
                right: right_formula,
            }
            .into(),
        }
    }
}

impl From<GenericAtomicFormula> for PrenexNormalFormula {
    fn from(f: GenericAtomicFormula) -> Self {
        Self {
            quantifiers: Vec::with_capacity(0),
            formula: f.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::syntax::{
        Conjunction, Disjunction, Existential, GenericFormula, PredicateCall, Universal, Variable,
    };

    use super::PrenexNormalFormula;

    #[test]
    fn test_pnf() {
        let var_x = Variable::new(b'x'.into());
        let var_y = Variable::new(b'y'.into());
        let var_z = Variable::new(b'z'.into());

        // (\-/x. ( -]y. P(y)) \/ Q(x)) /\ Q(z)
        let before: GenericFormula = Disjunction {
            left: Universal {
                left: var_x,
                right: Disjunction {
                    left: Existential {
                        left: var_y,
                        right: PredicateCall {
                            predicate: b'P'.into(),
                            terms: vec![var_y.into()],
                        }
                        .into(),
                    }
                    .into(),
                    right: PredicateCall {
                        predicate: b'Q'.into(),
                        terms: vec![var_x.into()],
                    }
                    .into(),
                }
                .into(),
            }
            .into(),
            right: Existential {
                left: var_z,
                right: PredicateCall {
                    predicate: b'Q'.into(),
                    terms: vec![var_z.into()],
                }
                .into(),
            }
            .into(),
        }
        .into();

        println!("{}", before);

        // (\-/x. ( -]y. P(y)) \/ Q(x)) /\ Q(z)
        // (\-/x. -]y. (P(y) \/ Q(x)) /\ Q(z)
        // \-/x.-]y.((P(y) \/ Q(x)) /\ Q(z)
        let expected_after: GenericFormula = Universal {
            left: var_x,
            right: Existential {
                left: var_y,
                right: Conjunction {
                    left: Disjunction {
                        left: PredicateCall {
                            predicate: b'P'.into(),
                            terms: vec![var_y.into()],
                        }
                        .into(),
                        right: PredicateCall {
                            predicate: b'Q'.into(),
                            terms: vec![var_x.into()],
                        }
                        .into(),
                    }
                    .into(),
                    right: PredicateCall {
                        predicate: b'Q'.into(),
                        terms: vec![var_z.into()],
                    }
                    .into(),
                }
                .into(),
            }
            .into(),
        }
        .into();
        println!("{}", expected_after);

        let after = PrenexNormalFormula::from(before);
        println!("{}", after);
    }
}
