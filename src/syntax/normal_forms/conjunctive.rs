use std::{ops::{BitOr, BitAnd}, fmt::Display};

use crate::syntax::{Negation, GenericAtomicFormula, Conjunction, Disjunction, Implication};

use super::{PrenexNormalFormulaTerm, SkolemNormalFormula};


/// A flat conjunction of the form `A ∧ B ∧ ...`
#[derive(Clone)]
pub struct FlatConjunction<T> {
    pub terms: Vec<T>,
}

impl<T> BitAnd for FlatConjunction<T> {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            terms: self.terms.into_iter()
                .chain(
                    rhs.terms.into_iter()
                ).collect()
        }
    }
}

impl<T: Display> Display for FlatConjunction<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.terms.len() > 1 {
            f.write_str("(")?;
        };

        let mut first = true;
        for term in &self.terms {
            if !first {
                f.write_str("∧")?;
            };
            first = false;
            term.fmt(f)?;
        };

        if self.terms.len() > 1 {
            f.write_str(")")?;
        };
        Ok(())
    }
}

/// A flat disjunction of the form `A ∨ B ∨ ...`
#[derive(Clone)]
pub struct FlatDisjunction <T> {
    pub terms: Vec<T>,
}

impl<T: Display> Display for FlatDisjunction<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.terms.len() > 1 {
            f.write_str("(")?;
        };

        let mut first = true;
        for term in &self.terms {
            if !first {
                f.write_str("∨")?;
            };
            first = false;
            term.fmt(f)?;
        };

        if self.terms.len() > 1 {
            f.write_str(")")?;
        };
        Ok(())
    }
}

/// Chain two flat disjunctions together. I.e. `(A ∨ B) | (C ∨ D) =
/// (A ∨ B ∨ C ∨ D)`.
impl<T> BitOr for FlatDisjunction<T> {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            terms: self.terms.into_iter()
                .chain(
                    rhs.terms.into_iter()
                ).collect()
        }
    }
}

#[derive(Clone)]
pub enum CNFInnerForm {
    Atomic(GenericAtomicFormula),
    Negated(Negation<GenericAtomicFormula>),
}

impl CNFInnerForm {
    pub fn negate(&self) -> Self {
        match self {
            Self::Atomic(x) => {
                Self::Negated(Negation { right: x.clone() })
            },
            Self::Negated(Negation { right: x }) => {
                Self::Atomic(x.clone())
            },
        }
    }
}

impl Display for CNFInnerForm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Atomic(x) => {x.fmt(f)},
            Self::Negated(x) => x.fmt(f),
        }
    }
}

pub type ConjunctiveNormalFormula = FlatConjunction<
    FlatDisjunction<
        CNFInnerForm
    >
>;

impl<T: Into<SkolemNormalFormula>> From<T> for ConjunctiveNormalFormula {
    fn from(f: T) -> Self {
        let skolem: SkolemNormalFormula = f.into();
        skolem.terms.into()
    }
}

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

impl From<GenericAtomicFormula> for ConjunctiveNormalFormula {
    fn from(f: GenericAtomicFormula) -> Self {
        ConjunctiveNormalFormula {
            terms: vec![
                FlatDisjunction {
                    terms: vec![
                        CNFInnerForm::Atomic(f),
                    ]
                }
            ]
        }
    }
}

impl From<Conjunction<ConjunctiveNormalFormula, ConjunctiveNormalFormula>> for ConjunctiveNormalFormula {
    fn from(f: Conjunction<ConjunctiveNormalFormula, ConjunctiveNormalFormula>) -> ConjunctiveNormalFormula {
        f.left & f.right
    }
}

impl From<Conjunction<PrenexNormalFormulaTerm, PrenexNormalFormulaTerm>> for ConjunctiveNormalFormula {
    fn from(f: Conjunction<PrenexNormalFormulaTerm, PrenexNormalFormulaTerm>) -> ConjunctiveNormalFormula {
        Conjunction::<ConjunctiveNormalFormula, ConjunctiveNormalFormula> {
            left: f.left.into(),
            right: f.right.into(),
        }.into()
    }
}

impl From<Disjunction<ConjunctiveNormalFormula, ConjunctiveNormalFormula>> for ConjunctiveNormalFormula {
    fn from(f: Disjunction<ConjunctiveNormalFormula, ConjunctiveNormalFormula>) -> ConjunctiveNormalFormula {
        let mut terms: Vec<FlatDisjunction<CNFInnerForm>> = Vec::with_capacity(f.left.terms.len() * f.right.terms.len());

        for l_term in &f.left.terms {
            for r_term in &f.right.terms {
                terms.push(l_term.clone() | r_term.clone())
            };
        };

        ConjunctiveNormalFormula {
            terms,
        }
    }
}

impl From<Disjunction<PrenexNormalFormulaTerm, PrenexNormalFormulaTerm>> for ConjunctiveNormalFormula {
    fn from(f: Disjunction<PrenexNormalFormulaTerm, PrenexNormalFormulaTerm>) -> ConjunctiveNormalFormula {
        Disjunction::<ConjunctiveNormalFormula, ConjunctiveNormalFormula> {
            left: f.left.into(),
            right: f.right.into(),
        }.into()
    }
}

impl From<Implication<PrenexNormalFormulaTerm, PrenexNormalFormulaTerm>> for Disjunction<PrenexNormalFormulaTerm, PrenexNormalFormulaTerm> {
    fn from(f: Implication<PrenexNormalFormulaTerm, PrenexNormalFormulaTerm>) -> Self {
        Disjunction {
            left: Negation { right: f.left }.into(),
            right: f.right,
        }
    }
}

impl From<Implication<PrenexNormalFormulaTerm, PrenexNormalFormulaTerm>> for ConjunctiveNormalFormula {
    fn from(f: Implication<PrenexNormalFormulaTerm, PrenexNormalFormulaTerm>) -> Self {
        Disjunction::<PrenexNormalFormulaTerm, PrenexNormalFormulaTerm>::from(f).into()
    }
}

impl From<Negation<ConjunctiveNormalFormula>> for ConjunctiveNormalFormula {
    fn from(f: Negation<ConjunctiveNormalFormula>) -> Self {
        let mut right = f.right;
        for conjuncted in &mut right.terms {
            for disjuncted in &mut conjuncted.terms {
                *disjuncted = disjuncted.negate()
            } 
        };
        right
    }
}

impl From<Negation<PrenexNormalFormulaTerm>> for ConjunctiveNormalFormula {
    fn from(f: Negation<PrenexNormalFormulaTerm>) -> ConjunctiveNormalFormula {
        Negation::<ConjunctiveNormalFormula> {
            right: f.right.into()
        }.into()
    }
}

#[cfg(test)]
mod tests {
    use crate::syntax::{
        Conjunction, Disjunction, Existential, GenericFormula, PredicateCall, Universal, Variable, normal_forms::{PrenexNormalFormula, ConjunctiveNormalFormula, SkolemNormalFormula},
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
