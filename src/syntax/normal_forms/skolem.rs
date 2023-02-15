use std::collections::HashMap;

use crate::syntax::{
    Conjunction,
    Constant,
    Disjunction,
    Equality,
    GenericAtomicFormula,
    GenericTerm,
    Implication,
    Negation,
    PredicateCall,
    Variable, FunctionCall,
};

use super::{PrenexNormalFormulaTerm, PrenexNormalFormula, PrenexNormalQuantifier};

////////////////////////////////////////////////////////////////////////////////
// Skolem-Normal Formula
////////////////////////////////////////////////////////////////////////////////

/// A SNF is the same as a PNF, only without any quantifiers. Any variable in a
/// SNF is treat as a free variable.
pub struct SkolemNormalFormula {
    /// The underlying terms of the formula (no quantifiers).
    pub terms: PrenexNormalFormulaTerm,
}

impl<T: Into<PrenexNormalFormula>> From<T> for SkolemNormalFormula {
    fn from(f: T) -> Self {
        let prenex: PrenexNormalFormula = f.into();
        let skol_state = SkolemisationState::from(prenex.quantifiers);

        let terms = prenex.formula.skolemise(&skol_state);

        Self {
            terms,
        }
    }
}

struct SkolemisationState {
    existential_vars: HashMap<Variable, FunctionCall<GenericTerm>>,
}

impl SkolemisationState {
    pub fn get_function(&self, var: &Variable) -> Option<&FunctionCall<GenericTerm>> {
        self.existential_vars.get(var)
    }
}

impl From<Vec<PrenexNormalQuantifier>> for SkolemisationState {
    fn from(quants: Vec<PrenexNormalQuantifier>) -> Self {
        
        let mut frees_accum: Vec<Variable> = Vec::new();
        let mut state: Self = Self {
            existential_vars: HashMap::new(),
        };

        for quant in quants {
            match quant {
                PrenexNormalQuantifier::Universal(v) => {
                    frees_accum.push(v);
                },
                PrenexNormalQuantifier::Existential(v) => {
                    if let Some(_) = state.existential_vars.insert(
                        v,
                        FunctionCall {
                            function: rand::random(),
                            terms: frees_accum.iter().copied().map(|v| GenericTerm::Variable(v)).collect(),
                        }
                    ) {
                        panic!()
                    }
                }
            }
        }

        state
    }
}

/// Private trait: Doesn't serve a function other than making refactoring
/// easier.
trait Skolemise {
    type Output;
    fn skolemise(self, state: &SkolemisationState) -> Self::Output;
}

impl Skolemise for Variable {
    type Output = GenericTerm;
    fn skolemise(self, state: &SkolemisationState) -> Self::Output {
        if let Some(f) = state.get_function(&self) {
            GenericTerm::FunctionCall(Box::new(f.clone()))
        } else {
            GenericTerm::Variable(self)
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Trivial Impls
////////////////////////////////////////////////////////////////////////////////

impl Skolemise for PrenexNormalFormulaTerm {
    type Output = Self;

    fn skolemise(self, state: &SkolemisationState) -> Self::Output {
        match self {
            Self::Atomic(x) => x.skolemise(state).into(),
            Self::Conjunction(x) => x.skolemise(state),
            Self::Disjunction(x) => x.skolemise(state),
            Self::Implication(x) => x.skolemise(state),
            Self::Negation(x) => x.skolemise(state),
        }
    }
}

impl Skolemise for GenericAtomicFormula {
    type Output = GenericAtomicFormula;

    fn skolemise(self, state: &SkolemisationState) -> Self::Output {
        match self {
            Self::Equality(x) => x.skolemise(state),
            Self::Predicate(x) => x.skolemise(state),
            Self::True => GenericAtomicFormula::True,
            Self::False => GenericAtomicFormula::False,
        }
    }
}

impl Skolemise for GenericTerm {
    type Output = GenericTerm;

    fn skolemise(self, state: &SkolemisationState) -> Self::Output {
        match self {
            Self::Variable(x) => x.skolemise(state),
            Self::Constant(x) => x.skolemise(state),
            Self::FunctionCall(x) => x.skolemise(state),
        }
    }
}

impl Skolemise for Conjunction<PrenexNormalFormulaTerm, PrenexNormalFormulaTerm> {
    type Output = PrenexNormalFormulaTerm;

    fn skolemise(self, state: &SkolemisationState) -> Self::Output {
        Self {
            left: self.left.skolemise(state),
            right: self.right.skolemise(state),
        }.into()
    }
}

impl Skolemise for Disjunction<PrenexNormalFormulaTerm, PrenexNormalFormulaTerm> {
    type Output = PrenexNormalFormulaTerm;

    fn skolemise(self, state: &SkolemisationState) -> Self::Output {
        Self {
            left: self.left.skolemise(state),
            right: self.right.skolemise(state),
        }.into()
    }
}

impl Skolemise for Implication<PrenexNormalFormulaTerm, PrenexNormalFormulaTerm> {
    type Output = PrenexNormalFormulaTerm;

    fn skolemise(self, state: &SkolemisationState) -> Self::Output {
        Self {
            left: self.left.skolemise(state),
            right: self.right.skolemise(state),
        }.into()
    }
}

impl Skolemise for Negation<PrenexNormalFormulaTerm> {
    type Output = PrenexNormalFormulaTerm;

    fn skolemise(self, state: &SkolemisationState) -> Self::Output {
        Self {
            right: self.right.skolemise(state),
        }.into()
    }
}

impl Skolemise for PredicateCall<GenericTerm> {
    type Output = GenericAtomicFormula;

    fn skolemise(mut self, state: &SkolemisationState) -> Self::Output {
        self.terms = self.terms.into_iter().map(|t|t.skolemise(state)).collect();
        self.into()
    }
}

impl Skolemise for Equality<GenericTerm, GenericTerm> {
    type Output = GenericAtomicFormula;

    fn skolemise(self, state: &SkolemisationState) -> Self::Output {
        Self {
            left: self.left.skolemise(state),
            right: self.right.skolemise(state),
        }.into()
    }
}

impl Skolemise for Constant {
    type Output = GenericTerm;

    fn skolemise(self, _: &SkolemisationState) -> Self::Output {
        self.into()
    }
}

impl Skolemise for FunctionCall<GenericTerm> {
    type Output = GenericTerm;

    fn skolemise(mut self, state: &SkolemisationState) -> Self::Output {
        self.terms = self.terms.into_iter().map(|t|t.skolemise(state)).collect();
        self.into()
    }
}