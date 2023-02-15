use super::{conjunctive::CNFInnerForm, ConjunctiveNormalFormula};

/// A Clause normal form (CNF) logical formula.
/// 
/// A CNF is a logical formula of the form `(P ∨ Q ∨ ...) ∧ (R ∨ ...) ∧ ` where
/// `P, Q, R, ...` are atomic formulae, expressed as a set of sets - I.e.
/// `{{P, Q, ...}, {R, ...}, ...}`.
/// 
pub struct ClauseNormalForm {
    /// The underlying sets of the CNF.
    pub sets: Vec<Vec<CNFInnerForm>>,
}

impl<T: Into<ConjunctiveNormalFormula>> From<T> for ClauseNormalForm {
    fn from(f: T) -> Self {
        let conj: ConjunctiveNormalFormula = f.into();
        Self {
            sets: conj.terms.into_iter().map(|x| {
                x.terms
            }).collect(),
        }
    }
}