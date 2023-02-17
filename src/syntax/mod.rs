mod grammar;

/// The normal forms, and methods for converting between them.
///
/// Supports:
/// - Prenex-normal form (PNF)
/// - Skolem-normal form (SNF)
/// - Conjunctive-normal form / clause-normal form (CNF)
///
/// Each normal form has an associated struct, and conversion between them can
/// be done using the [`From`] trait, e.g:
/// ```
/// # use first_order_logic::syntax::{
/// #   normal_forms::{
/// #       PrenexNormalFormula,
/// #       SkolemNormalFormula,
/// #       PrenexNormalFormulaTerm,
/// #   },
/// #   GenericAtomicFormula,
/// # };
/// # let pnf = PrenexNormalFormula {
/// #   quantifiers: vec![],
/// #   formula: PrenexNormalFormulaTerm::Atomic(GenericAtomicFormula::True),
/// # };
/// let snf: SkolemNormalFormula = SkolemNormalFormula::from(pnf);
/// ```
///
/// Note that conversion can only take place moving up the 'conversion ladder',
/// i.e.
/// ```text
/// General -> Prenex -> Skolem -> Conjunctive
/// ```
///
/// There is, in general, no unique way to make the conversions in the reverse
/// direction.
pub mod normal_forms;

pub use grammar::*;
