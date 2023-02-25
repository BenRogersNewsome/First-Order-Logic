use std::{
    collections::HashSet,
    hash::Hash,
    mem::MaybeUninit,
    ops::{BitAndAssign, BitOrAssign, Deref, DerefMut, Index, IndexMut},
};

/// Whether a given data structure *exists* within the Domain of Discourse (DOD)
/// in the broadest sense.
pub trait Existential {
    /// Does the item exist in the DOD.
    fn exists(&self) -> bool;
    /// Does the item encompass the whole of the DOD.
    fn maximal(&self) -> bool;
}

/// Quantify element(s).
///
/// Mainly used to pass arguments to [predicates](super::Predicate) and
/// [functions](super::Function).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ElementQuantifier<E> {
    /// Specify a single element
    One(E),
    /// Specify any element
    Any,
}

impl<E> From<E> for ElementQuantifier<E> {
    fn from(e: E) -> Self {
        Self::One(e)
    }
}

/// Return a set of elements within the Domain of Discourse (DOD).
///
/// Subtly different to `ElementQuantifier`: `ElementQuantifier` specifies a
/// range of elements for passing to a predicate, whereas `ElementSet`
/// corresponds to an explicit list of elements.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ElementSet<E> {
    /// Every element in the DOD.
    All,
    /// A list of specific elements in the DOD.
    Some(Vec<E>),
    /// Zero elements in the DOD.
    None,
}

impl<E> From<E> for ElementSet<E> {
    fn from(e: E) -> Self {
        Self::Some(vec![e])
    }
}

impl<E> Existential for ElementSet<E> {
    fn exists(&self) -> bool {
        !matches!(self, Self::None)
    }

    fn maximal(&self) -> bool {
        matches!(self, Self::All)
    }
}

impl<E: Hash + Eq + Clone> BitAndAssign for ElementSet<E> {
    fn bitand_assign(&mut self, rhs: Self) {
        match (&*self, &rhs) {
            (&Self::All, x) | (x, &Self::All) => *self = x.clone(),

            (Self::None, _) | (_, Self::None) => *self = Self::None,

            (Self::Some(x), Self::Some(y)) => {
                let set_x: HashSet<E> = x.iter().cloned().collect();
                let set_y: HashSet<E> = y.iter().cloned().collect();
                *self = Self::Some(set_x.intersection(&set_y).into_iter().cloned().collect());
            }
        }
    }
}

impl<E: Clone> BitOrAssign for ElementSet<E> {
    fn bitor_assign(&mut self, rhs: Self) {
        match (&*self, rhs) {
            (Self::All, _) | (_, Self::All) => *self = Self::All,

            (Self::None, x) => *self = x,

            (x, Self::None) => *self = x.clone(),

            (Self::Some(_), Self::Some(y)) => {
                if let Self::Some(x) = self {
                    x.extend(y);
                } else {
                    unreachable!(
                        "Only here because of the reference moving in the match statement."
                    )
                }
            }
        }
    }
}

/// Arguments for predicates and functions.
///
/// Generic over the element, and arity of the function/predicate.
///
/// Essentially a wrapper type over an array, to allow for trait
/// implementations.
#[derive(Debug)]
pub struct Arguments<E, const ARITY: usize> {
    _inner: [E; ARITY],
}

impl<E, const ARITY: usize> Arguments<E, ARITY> {
    /// Convert arguments from one type to another using a callback function
    /// applied to each arg.
    pub fn map<F>(self, callback: fn(args: E) -> F) -> Arguments<F, ARITY> {
        Arguments::from(self._inner.map(callback))
    }
}

impl<E, const ARITY: usize> PartialEq for Arguments<E, ARITY>
where
    E: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self._inner.eq(&other._inner)
    }
}

impl<E, const ARITY: usize> Eq for Arguments<E, ARITY> where E: Eq {}

impl<E: Clone + Sized, const ARITY: usize> Arguments<E, ARITY> {
    /// Construct arguments where every position equals `item`.
    pub fn every(item: E) -> Self {
        let mut arr: [MaybeUninit<E>; ARITY] = unsafe { MaybeUninit::uninit().assume_init() };

        for elem in &mut arr[..] {
            elem.write(item.clone());
        }

        let _inner: [E; ARITY] = unsafe { std::mem::transmute_copy::<_, [E; ARITY]>(&arr) };

        Self { _inner }
    }
}

impl<E: Existential, const ARITY: usize> Existential for Arguments<E, ARITY> {
    /// Return whether or not the given arguments exist in the DOD.
    /// This basically just checks whether all the items are None, and if true
    /// returns false
    fn exists(&self) -> bool {
        self._inner.iter().all(|arg| arg.exists())
    }

    fn maximal(&self) -> bool {
        self._inner.iter().all(|arg| arg.maximal())
    }
}

impl<E: Clone, const ARITY: usize> Clone for Arguments<E, ARITY> {
    fn clone(&self) -> Self {
        Self {
            _inner: self._inner.clone(),
        }
    }
}

impl<E, const ARITY: usize> From<[E; ARITY]> for Arguments<E, ARITY> {
    fn from(_inner: [E; ARITY]) -> Self {
        Self { _inner }
    }
}

impl<E, const ARITY: usize> Deref for Arguments<E, ARITY> {
    type Target = [E; ARITY];

    fn deref(&self) -> &Self::Target {
        &self._inner
    }
}

impl<E, const ARITY: usize> DerefMut for Arguments<E, ARITY> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self._inner
    }
}

impl<E, const ARITY: usize> Index<usize> for Arguments<E, ARITY> {
    type Output = E;

    fn index(&self, index: usize) -> &Self::Output {
        &self._inner[index]
    }
}

impl<E, const ARITY: usize> IndexMut<usize> for Arguments<E, ARITY> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self._inner[index]
    }
}

impl<E: BitAndAssign + Clone, const ARITY: usize> BitAndAssign for Arguments<E, ARITY> {
    fn bitand_assign(&mut self, rhs: Self) {
        rhs._inner.into_iter().enumerate().for_each(|(i, rr)| {
            self._inner[i] &= rr;
        });
    }
}

impl<E: BitOrAssign, const ARITY: usize> BitOrAssign for Arguments<E, ARITY> {
    fn bitor_assign(&mut self, rhs: Self) {
        rhs._inner.into_iter().enumerate().for_each(|(i, rr)| {
            self._inner[i] |= rr;
        });
    }
}

/// Map between arguments of different arity.
///
/// The map is generic over the type of element that is being mapped.
///
/// Note that the forward map is required to be surjective - that is every element
/// in the right domain must be specified by the map - whereas the reverse map
/// must be injective (seen in the use of `Option`) in the reverse map - that
/// is, every element in the left domain must map to at least one element in
/// the right domain. This means that in the reverse map, some elements of the
/// left domain may remain undetermined. In such a case, those elements will be
/// set to the value specified by the argument `default`.
///
/// # Examples
///
/// A reverse map with a default specified:
/// ```
/// # use first_order_logic::{args, semantics::elements::{
/// #   ArgumentMap, Arguments, ElementQuantifier,
/// # }};
/// let map: ArgumentMap<2, 1> = ArgumentMap::new([0]);
///
/// let args: Arguments<ElementQuantifier<usize>, 1> = args!(ElementQuantifier::One(2));
///
/// assert_eq!(
///     map.backward(&args, ElementQuantifier::Any),
///     args!(ElementQuantifier::One(2), ElementQuantifier::Any),
/// );
/// ```
#[derive(Debug, Clone, Copy)]
pub struct ArgumentMap<const FROM_ARITY: usize, const TO_ARITY: usize> {
    _forward: [usize; TO_ARITY],
    _backward: [Option<usize>; FROM_ARITY],
}

impl<const FROM_ARITY: usize, const TO_ARITY: usize> ArgumentMap<FROM_ARITY, TO_ARITY> {
    /// Construct a new argument map by specifying the forward map.
    pub fn new(forward: [usize; TO_ARITY]) -> Self {
        Self {
            _forward: forward,
            _backward: Self::generate_backward_map_from_forward(forward),
        }
    }

    fn generate_backward_map_from_forward(
        forward: [usize; TO_ARITY],
    ) -> [Option<usize>; FROM_ARITY] {
        let mut backward: [Option<usize>; FROM_ARITY] = [None; FROM_ARITY];

        for (i, &elem) in forward.iter().enumerate() {
            backward[elem] = Some(i);
        }

        backward
    }

    /// Apply the map forwards.
    pub fn forward<E: Clone>(&self, args: &Arguments<E, FROM_ARITY>) -> Arguments<E, TO_ARITY> {
        Arguments::from(self._forward.map(|i| args[i].clone()))
    }

    /// Apply the map in reverse, and apply default where argument is
    /// undetermined.
    pub fn backward<E: Clone>(
        &self,
        args: &Arguments<E, TO_ARITY>,
        default: E,
    ) -> Arguments<E, FROM_ARITY> {
        Arguments::from(self._backward.map(|oi| {
            oi.map(|i| args[i].clone())
                .unwrap_or_else(|| default.clone())
        }))
    }
}

/// Generate some args
///
/// # Examples
/// ```
/// # use first_order_logic::{args, semantics::elements::Arguments};
/// let args: Arguments<usize, 3> = args!(3 as usize, 6 as usize, 4 as usize);
/// ```
///
/// Automatically performs the conversion when used with `ElementQuantifier`:
/// ```
/// # use first_order_logic::{args, semantics::elements::{Arguments, ElementQuantifier}};
/// let args: Arguments<ElementQuantifier<usize>, 3> = args!(3, 6, 4);
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "semantics")))]
#[macro_export]
macro_rules! args {
    ($($x:expr),+ $(,)?) => (
        $crate::semantics::elements::Arguments::from(
            [$($x.into()),+]
        )
    );
}

/// Generate a one-to-one argument map
///
/// # Examples
///
/// ```
/// # use first_order_logic::{semantics::elements::ArgumentMap, one_to_one};
/// let one_to_one_map: ArgumentMap<1,1> = one_to_one!(1);
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "semantics")))]
#[macro_export]
macro_rules! one_to_one {
    ($ar:ident) => {{
        use $crate::semantics::elements::ArgumentMap;
        let forward = [0; $ar];
        ArgumentMap::new(forward)
    }};
    ($ar:literal) => {{
        use $crate::semantics::elements::ArgumentMap;
        let forward = [0; $ar];
        ArgumentMap::new(forward)
    }};
}
