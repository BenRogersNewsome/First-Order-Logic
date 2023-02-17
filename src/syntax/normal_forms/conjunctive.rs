use std::{
    fmt::Display,
    ops::{BitAnd, BitOr},
};

use crate::syntax::{Conjunction, Disjunction, GenericAtomicFormula, Implication, Negation};

use super::{PrenexNormalFormulaTerm, SkolemNormalFormula};

/// A Clause normal Form (CNF).
///
/// Semantically equivalent to a conjunctive normal form
/// (`(P ∨ Q ∨ ...) ∧ (R ∨ ...) ∧ `), only expressed as a set of sets
/// (`{{P, Q, ...}, {R, ...}, ...}`).
pub type ClauseNormalForm = Vec<Vec<Literal>>;

impl Into<ClauseNormalForm> for ConjunctiveNormalFormula {
    fn into(self) -> Vec<Vec<Literal>> {
        self.clauses.into_iter().map(|x| x.literals).collect()
    }
}

/// A formula in Conjunctive Normal Form.
///
/// A formula of the form `(P ∨ Q ∨ ...) ∧ (R ∨ ...) ∧ ...`
#[derive(Clone)]
pub struct ConjunctiveNormalFormula {
    /// The clauses of the CNF.
    pub clauses: Vec<Clause>,
}

/// Convert a formula, in any other form, into its CNF.
impl<T: Into<SkolemNormalFormula>> From<T> for ConjunctiveNormalFormula {
    fn from(f: T) -> Self {
        let skolem: SkolemNormalFormula = f.into();
        skolem.terms.into()
    }
}

/// Combine two conjunctive normal formula together.
impl BitAnd for ConjunctiveNormalFormula {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            clauses: self
                .clauses
                .into_iter()
                .chain(rhs.clauses.into_iter())
                .collect(),
        }
    }
}

impl Display for ConjunctiveNormalFormula {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.clauses.len() > 1 {
            f.write_str("(")?;
        };

        let mut first = true;
        for term in &self.clauses {
            if !first {
                f.write_str("∧")?;
            };
            first = false;
            term.fmt(f)?;
        }

        if self.clauses.len() > 1 {
            f.write_str(")")?;
        };
        Ok(())
    }
}

/// A clause of literals.
///
/// A flat disjunction of the form `A ∨ B ∨ ...`.
#[derive(Clone)]
pub struct Clause {
    /// The literals which make up the clause.
    pub literals: Vec<Literal>,
}

impl Display for Clause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.literals.len() > 1 {
            f.write_str("(")?;
        };

        let mut first = true;
        for term in &self.literals {
            if !first {
                f.write_str("∨")?;
            };
            first = false;
            term.fmt(f)?;
        }

        if self.literals.len() > 1 {
            f.write_str(")")?;
        };
        Ok(())
    }
}

/// Chain two clauses together. I.e. `(A ∨ B) | (C ∨ D) =
/// (A ∨ B ∨ C ∨ D)`.
impl BitOr for Clause {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            literals: self
                .literals
                .into_iter()
                .chain(rhs.literals.into_iter())
                .collect(),
        }
    }
}

/// A logical literal - an atom or its negation.
#[derive(Clone)]
pub enum Literal {
    /// An atom
    Atom(GenericAtomicFormula),
    /// A negated atom
    Negated(Negation<GenericAtomicFormula>),
}

impl Literal {
    /// Negate a literal, returning the new literal.
    pub fn negate(&self) -> Self {
        match self {
            Self::Atom(x) => Self::Negated(Negation { right: x.clone() }),
            Self::Negated(Negation { right: x }) => Self::Atom(x.clone()),
        }
    }
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Atom(x) => x.fmt(f),
            Self::Negated(x) => x.fmt(f),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Conversion Algorithm
////////////////////////////////////////////////////////////////////////////////

#[doc(hidden)]
impl From<PrenexNormalFormulaTerm> for ConjunctiveNormalFormula {
    fn from(f: PrenexNormalFormulaTerm) -> Self {
        match f {
            PrenexNormalFormulaTerm::Atomic(x) => x.into(),
            PrenexNormalFormulaTerm::Conjunction(x) => (*x).into(),
            PrenexNormalFormulaTerm::Disjunction(x) => (*x).into(),
            PrenexNormalFormulaTerm::Implication(x) => (*x).into(),
            PrenexNormalFormulaTerm::Negation(x) => (*x).into(),
        }
    }
}

#[doc(hidden)]
impl From<Conjunction<ConjunctiveNormalFormula, ConjunctiveNormalFormula>>
    for ConjunctiveNormalFormula
{
    fn from(
        f: Conjunction<ConjunctiveNormalFormula, ConjunctiveNormalFormula>,
    ) -> ConjunctiveNormalFormula {
        f.left & f.right
    }
}

#[doc(hidden)]
impl From<Conjunction<PrenexNormalFormulaTerm, PrenexNormalFormulaTerm>>
    for ConjunctiveNormalFormula
{
    fn from(
        f: Conjunction<PrenexNormalFormulaTerm, PrenexNormalFormulaTerm>,
    ) -> ConjunctiveNormalFormula {
        Conjunction::<ConjunctiveNormalFormula, ConjunctiveNormalFormula> {
            left: f.left.into(),
            right: f.right.into(),
        }
        .into()
    }
}

#[doc(hidden)]
impl From<Disjunction<ConjunctiveNormalFormula, ConjunctiveNormalFormula>>
    for ConjunctiveNormalFormula
{
    fn from(
        f: Disjunction<ConjunctiveNormalFormula, ConjunctiveNormalFormula>,
    ) -> ConjunctiveNormalFormula {
        let mut terms: Vec<Clause> =
            Vec::with_capacity(f.left.clauses.len() * f.right.clauses.len());

        for l_term in &f.left.clauses {
            for r_term in &f.right.clauses {
                terms.push(l_term.clone() | r_term.clone())
            }
        }

        ConjunctiveNormalFormula { clauses: terms }
    }
}

#[doc(hidden)]
impl From<Disjunction<PrenexNormalFormulaTerm, PrenexNormalFormulaTerm>>
    for ConjunctiveNormalFormula
{
    fn from(
        f: Disjunction<PrenexNormalFormulaTerm, PrenexNormalFormulaTerm>,
    ) -> ConjunctiveNormalFormula {
        Disjunction::<ConjunctiveNormalFormula, ConjunctiveNormalFormula> {
            left: f.left.into(),
            right: f.right.into(),
        }
        .into()
    }
}

#[doc(hidden)]
impl From<Implication<PrenexNormalFormulaTerm, PrenexNormalFormulaTerm>>
    for Disjunction<PrenexNormalFormulaTerm, PrenexNormalFormulaTerm>
{
    fn from(f: Implication<PrenexNormalFormulaTerm, PrenexNormalFormulaTerm>) -> Self {
        Disjunction {
            left: Negation { right: f.left }.into(),
            right: f.right,
        }
    }
}

#[doc(hidden)]
impl From<Implication<PrenexNormalFormulaTerm, PrenexNormalFormulaTerm>>
    for ConjunctiveNormalFormula
{
    fn from(f: Implication<PrenexNormalFormulaTerm, PrenexNormalFormulaTerm>) -> Self {
        Disjunction::<PrenexNormalFormulaTerm, PrenexNormalFormulaTerm>::from(f).into()
    }
}

#[doc(hidden)]
impl From<Negation<ConjunctiveNormalFormula>> for ConjunctiveNormalFormula {
    fn from(f: Negation<ConjunctiveNormalFormula>) -> Self {
        let mut right = f.right;
        for conjuncted in &mut right.clauses {
            for disjuncted in &mut conjuncted.literals {
                *disjuncted = disjuncted.negate()
            }
        }
        right
    }
}

#[doc(hidden)]
impl From<Negation<PrenexNormalFormulaTerm>> for ConjunctiveNormalFormula {
    fn from(f: Negation<PrenexNormalFormulaTerm>) -> ConjunctiveNormalFormula {
        Negation::<ConjunctiveNormalFormula> {
            right: f.right.into(),
        }
        .into()
    }
}

#[cfg(test)]
mod tests {
    use crate::syntax::{
        normal_forms::{ConjunctiveNormalFormula, PrenexNormalFormula, SkolemNormalFormula},
        Conjunction, Disjunction, Existential, GenericFormula, PredicateCall, Universal, Variable,
    };

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
                left: var_z.into(),
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

        let pnf = PrenexNormalFormula::from(before);
        println!("{}", pnf);

        let snf = SkolemNormalFormula::from(pnf);
        println!("A");

        let cjnf = ConjunctiveNormalFormula::from(snf);
        println!("{}", cjnf);
    }
}
