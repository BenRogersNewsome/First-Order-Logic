use std::ops::{BitAnd, BitOr, Deref, Not};

////////////////////////////////////////////////////////////////////////////////
// Truth Value
////////////////////////////////////////////////////////////////////////////////

/// An enum representing the possible truth values in a FOL formula.
///
/// In standard first order logic, where complete knowledge over the Domain of
/// Discourse (DOD) is assumed, truth values can only assume either `True` or
/// `False`. The addition of the additional value `Unknown` is to account for
/// situations in which there is incomplete knowledge of the DOD.
///
/// # Examples
///
/// Logical operations are implemented based on what can be proven to be true:
/// ```
/// # use first_order_logic::TruthValue;
/// assert_eq!(
///     TruthValue::Determined(true) & TruthValue::Undetermined,
///     TruthValue::Undetermined,
/// );
///
/// assert_eq!(
///     TruthValue::Determined(true) | TruthValue::Undetermined,
///     TruthValue::Determined(true),
/// );
/// ```
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TruthValue {
    /// The truth value is known to be either true or false.
    Determined(bool),
    /// The truth value is unknown.
    Undetermined,
}

////////////////////////////////////////////////////////////////////////////////
// Casting to and from Booleans
////////////////////////////////////////////////////////////////////////////////

impl From<bool> for TruthValue {
    fn from(b: bool) -> Self {
        match b {
            true => Self::Determined(true),
            false => Self::Determined(false),
        }
    }
}

impl Deref for TruthValue {
    type Target = bool;
    fn deref(&self) -> &Self::Target {
        match self {
            Self::Determined(b) => &b,
            Self::Undetermined => &false,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Logical Operations
////////////////////////////////////////////////////////////////////////////////

impl BitOr for TruthValue {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        use TruthValue::*;
        match (self, rhs) {
            (Determined(true), _) | (_, Determined(true)) => Self::Determined(true),
            (Determined(false), Determined(false)) => Self::Determined(false),
            _ => Undetermined,
        }
    }
}

impl BitAnd for TruthValue {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        use TruthValue::*;
        match (self, rhs) {
            (Determined(false), _) | (_, Determined(false)) => Self::Determined(false),
            (Determined(true), Determined(true)) => Self::Determined(true),
            _ => Undetermined,
        }
    }
}

impl Not for TruthValue {
    type Output = Self;
    fn not(self) -> Self::Output {
        match self {
            Self::Determined(x) => Self::Determined(!x),
            Self::Undetermined => Self::Undetermined,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Assertion Response
////////////////////////////////////////////////////////////////////////////////

/// The possible responses which can be returned from an assertion of a FOL
/// formula.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AssertionResponse {
    /// Assertion invalid due to some logical contradiction.
    AssertionInvalid,
    /// Assertion was valid and has been asserted.
    AssertionMade,
    /// Assertion is already valid - no action taken.
    AssertionRedundant,
    /// Can't make assertion due to some internal limitation.
    /// TODO: Remove
    CannotMakeAssertion,
}

////////////////////////////////////////////////////////////////////////////////
// Element Value
////////////////////////////////////////////////////////////////////////////////

/// The possible values of an element in a FOL formula.
pub enum ElementValue<E: Eq> {
    /// The element value is known
    Determined(E),
    /// The element value is unknown
    Undetermined,
}

impl<E: Eq> BitOr for ElementValue<E> {
    type Output = ElementValue<E>;
    fn bitor(self, rhs: Self) -> Self::Output {
        use ElementValue::*;
        match (self, rhs) {
            (Undetermined, Undetermined) => Undetermined,
            (Undetermined, Determined(e)) | (Determined(e), Undetermined) => Determined(e),
            (Determined(e_1), Determined(e_2)) if e_1 == e_2 => Determined(e_1),
            (Determined(_), Determined(_)) => panic!("Unexpected element value or"),
        }
    }
}

#[cfg(test)]
mod truth_value_logical_ops_tests {

    use super::TruthValue::*;

    #[test]
    fn test_logical_or() {
        assert_eq!(Determined(true) | Determined(true), Determined(true));
        assert_eq!(Determined(true) | Determined(false), Determined(true));
        assert_eq!(Determined(false) | Determined(true), Determined(true));
        assert_eq!(Determined(false) | Determined(false), Determined(false));
        assert_eq!(Undetermined | Determined(false), Undetermined);
        assert_eq!(Determined(false) | Undetermined, Undetermined);
        assert_eq!(Undetermined | Determined(true), Determined(true));
        assert_eq!(Determined(true) | Undetermined, Determined(true));
        assert_eq!(Undetermined | Undetermined, Undetermined);
    }

    #[test]
    fn test_logical_and() {
        assert_eq!(Determined(true) & Determined(true), Determined(true));
        assert_eq!(Determined(true) & Determined(false), Determined(false));
        assert_eq!(Determined(false) & Determined(true), Determined(false));
        assert_eq!(Determined(false) & Determined(false), Determined(false));
        assert_eq!(Undetermined & Determined(false), Determined(false));
        assert_eq!(Determined(false) & Undetermined, Determined(false));
        assert_eq!(Undetermined & Determined(true), Undetermined);
        assert_eq!(Determined(true) & Undetermined, Undetermined);
        assert_eq!(Undetermined & Undetermined, Undetermined);
    }

    #[test]
    fn test_logical_not() {
        assert_eq!(!Determined(true), Determined(false));
        assert_eq!(!Determined(false), Determined(true));
        assert_eq!(!Undetermined, Undetermined);
    }
}
