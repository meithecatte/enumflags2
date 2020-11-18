//! # Enum Flags
//! `enumflags2` defines a `BitFlags<T>` type, which is a `Set<T>`
//! for enums without associated data.
//!
//! This means that the type of a single flag is separate from a set of flags.
//!
//! ## Example
//! ```
//! use enumflags2::{bitflags, BitFlags};
//!
//! #[bitflags]
//! #[repr(u8)]
//! #[derive(Copy, Clone, Debug, PartialEq)]
//! enum Test {
//!     A = 0b0001,
//!     B = 0b0010,
//!     C = 0b0100,
//!     D = 0b1000,
//! }
//!
//! let a_b = Test::A | Test::B; // BitFlags<Test>
//! let a_c = Test::A | Test::C;
//! let b_c_d = Test::C | Test::B | Test::D;
//!
//! // BitFlags<Test>(0b11, [A, B])
//! println!("{:?}", a_b);
//!
//! // BitFlags<Test>(0b1, [A])
//! println!("{:?}", a_b & a_c);
//!
//! // Iterate over the flags like a normal set!
//! assert_eq!(a_b.iter().collect::<Vec<_>>(), &[Test::A, Test::B]);
//!
//! assert!(a_b.contains(Test::A));
//! assert!(b_c_d.contains(Test::B | Test::C));
//! assert!(!(b_c_d.contains(a_b)));
//!
//! assert!(a_b.intersects(a_c));
//! assert!(!(a_b.intersects(Test::C | Test::D)));
//! ```
//!
//! ## Note
//!
//! By default, the `BitFlags` are `usize`-sized. If you want them to be smaller,
//! specify a `repr` on your enum as in the example above.
//!
//! ## Optional Feature Flags
//!
//! - [`serde`](https://serde.rs/) implements `Serialize` and `Deserialize`
//!   for `BitFlags<T>`.
//! - `std` implements `std::error::Error` for `FromBitsError`.
//!
//! ### Migrating from 0.5
//!
//! The minimum rustc version has been bumped to 1.34.0, because of `syn 1.0`. The
//! version policy from now on will be "what's available on Debian stable", [because
//! Debian is famously slow with new software versions][debian-snailpace].
//!
//! You should no longer depend on `enumflags2_derive` directly.
//! Use the reexport from the `enumflags2` crate.
//! semver guarantees will be violated if you depend on the derive crate directly.
//!
//! The derive macro has been renamed to `BitFlags`, to make it clearer what the
//! derive does.
//!
//! The `nostd` feature flag has been removed. The crate now only depends on `libcore`
//! by default. Enable the `std` flag to get an implementation of `std::error::Error`
//! on error types.
//!
//! Flags more than one bit set have been found to have inconsistent semantics.
//! They are now rejected at compile-time. The same applies to flags without any
//! bit set. If you were relying on this in your code, please [open an issue][issue]
//! and explain your usecase.
//!
//! `BitFlags::from_bits` returns a `Result` instead of an `Option`. This might
//! necessitate some minor changes in your code.
//!
//! `BitFlags::not` has been removed. Use the `!` operator instead.
//!
//! [debian-snailpace]: https://www.jwz.org/blog/2016/04/i-would-like-debian-to-stop-shipping-xscreensaver/
//! [issue]: https://github.com/NieDzejkob/enumflags2/issues/new
#![warn(missing_docs)]
#![cfg_attr(all(not(test), not(feature = "std")), no_std)]

use core::{cmp, ops};
use core::iter::FromIterator;
use core::marker::PhantomData;

#[allow(unused_imports)]
#[macro_use]
extern crate enumflags2_derive;

#[doc(hidden)]
pub use enumflags2_derive::bitflags_internal as bitflags;

// Internal macro: expand into a separate copy for each supported numeric type.
macro_rules! for_each_uint {
    ( $tyvar:ident => $($input:tt)* ) => {
        // This wrapper macro is necessary to create a $ escape sequence
        // for use in macro repetitions in implement!
        // cf. https://github.com/rust-lang/rust/issues/35853
        macro_rules! with_dollar {
            ( $d:tt ) => {
                macro_rules! implement {
                    ( $d($d $tyvar:ty),* ) => {
                        $d(
                            $($input)*
                        )*
                    }
                }
            }
        }

        with_dollar!($);

        implement! { u8, u16, u32, u64, u128 }
    }
}

/// A trait automatically implemented by `#[bitflags]` to make the enum
/// a valid type parameter for `BitFlags<T>`.
pub trait BitFlag: Copy + Clone + 'static + _internal::RawBitFlags {
    /// Create a `BitFlags` with no flags set (in other words, with a value of 0).
    ///
    /// This is a convenience reexport of [`BitFlags::empty`]. It can be called with
    /// `MyFlag::empty()`, thus bypassing the need for type hints in some situations.
    ///
    /// [`BitFlags::empty`]: struct.BitFlags.html#method.empty
    ///
    /// ```
    /// # use enumflags2::{bitflags, BitFlags};
    /// #[bitflags]
    /// #[repr(u8)]
    /// #[derive(Clone, Copy, PartialEq, Eq)]
    /// enum MyFlag {
    ///     One = 1 << 0,
    ///     Two = 1 << 1,
    ///     Three = 1 << 2,
    /// }
    ///
    /// use enumflags2::BitFlag;
    ///
    /// let empty = MyFlag::empty();
    /// assert!(empty.is_empty());
    /// assert_eq!(empty.contains(MyFlag::One), false);
    /// assert_eq!(empty.contains(MyFlag::Two), false);
    /// assert_eq!(empty.contains(MyFlag::Three), false);
    /// ```
    fn empty() -> BitFlags<Self> {
        BitFlags::empty()
    }

    /// Create a `BitFlags` with all flags set.
    ///
    /// This is a convenience reexport of [`BitFlags::all`]. It can be called with
    /// `MyFlag::all()`, thus bypassing the need for type hints in some situations.
    ///
    /// [`BitFlags::all`]: struct.BitFlags.html#method.all
    ///
    /// ```
    /// # use enumflags2::{bitflags, BitFlags};
    /// #[bitflags]
    /// #[repr(u8)]
    /// #[derive(Clone, Copy, PartialEq, Eq)]
    /// enum MyFlag {
    ///     One = 1 << 0,
    ///     Two = 1 << 1,
    ///     Three = 1 << 2,
    /// }
    ///
    /// use enumflags2::BitFlag;
    ///
    /// let empty = MyFlag::all();
    /// assert!(empty.is_all());
    /// assert_eq!(empty.contains(MyFlag::One), true);
    /// assert_eq!(empty.contains(MyFlag::Two), true);
    /// assert_eq!(empty.contains(MyFlag::Three), true);
    /// ```
    fn all() -> BitFlags<Self> {
        BitFlags::all()
    }
}

/// While the module is public, this is only the case because it needs to be
/// accessed by the macro. Do not use this directly. Stability guarantees
/// don't apply.
#[doc(hidden)]
pub mod _internal {
    /// A trait automatically implemented by `#[bitflags]` to make the enum
    /// a valid type parameter for `BitFlags<T>`.
    pub trait RawBitFlags: Copy + Clone + 'static {
        /// The underlying integer type.
        type Numeric: BitFlagNum;

        /// A value with no bits set.
        const EMPTY: Self::Numeric;

        /// A value with all flag bits set.
        const ALL_BITS: Self::Numeric;

        /// A slice that contains each variant exactly one.
        const FLAG_LIST: &'static [Self];

        /// The name of the type for debug formatting purposes.
        ///
        /// This is typically `BitFlags<EnumName>`
        const BITFLAGS_TYPE_NAME: &'static str;

        /// Return the bits as a number type.
        fn bits(self) -> Self::Numeric;
    }

    use ::core::ops::{BitAnd, BitOr, BitXor, Not};
    use ::core::cmp::PartialOrd;
    use ::core::fmt;

    pub trait BitFlagNum
        : Default
        + BitOr<Self, Output = Self>
        + BitAnd<Self, Output = Self>
        + BitXor<Self, Output = Self>
        + Not<Output = Self>
        + PartialOrd<Self>
        + fmt::Debug
        + fmt::Binary
        + Copy
        + Clone {
    }

    for_each_uint! { ty =>
        impl BitFlagNum for $ty {}
    }

    // Re-export libcore so the macro doesn't inject "extern crate" downstream.
    pub mod core {
        pub use core::{convert, option, ops};
    }

    pub struct AssertionSucceeded;
    pub struct AssertionFailed;
    pub trait ExactlyOneBitSet {
        type X;
    }
    impl ExactlyOneBitSet for AssertionSucceeded {
        type X = ();
    }

    pub trait AssertionHelper {
        type Status;
    }

    impl AssertionHelper for [(); 1] {
        type Status = AssertionSucceeded;
    }

    impl AssertionHelper for [(); 0] {
        type Status = AssertionFailed;
    }
}

// Internal debug formatting implementations
mod formatting;

// impl TryFrom<T::Numeric> for BitFlags<T>
mod fallible;
pub use crate::fallible::FromBitsError;

/// Represents a set of flags of some type `T`.
/// `T` must have the `#[bitflags]` attribute applied.
///
/// A `BitFlags<T>` is as large as the `T` itself,
/// and stores one flag per bit.
///
/// ## Memory layout
///
/// `BitFlags<T>` is marked with the `#[repr(transparent)]` trait, meaning
/// it can be safely transmuted into the corresponding numeric type.
///
/// Usually, the same can be achieved by using [`BitFlags::from_bits`],
/// [`BitFlags::from_bits_truncated`] or [`BitFlags::from_bits_unchecked`],
/// but transmuting might still be useful if, for example, you're dealing with
/// an entire array of `BitFlags`.
///
/// Transmuting from a numeric type into `BitFlags` may also be done, but
/// care must be taken to make sure that each set bit in the value corresponds
/// to an existing flag (cf. [`from_bits_unchecked`]).
///
/// For example:
///
/// ```
/// # use enumflags2::{BitFlags, bitflags};
/// #[bitflags]
/// #[repr(u8)] // <-- the repr determines the numeric type
/// #[derive(Copy, Clone)]
/// enum TransmuteMe {
///     One = 1 << 0,
///     Two = 1 << 1,
/// }
///
/// # use std::slice;
/// // NOTE: we use a small, self-contained function to handle the slice
/// // conversion to make sure the lifetimes are right.
/// fn transmute_slice<'a>(input: &'a [BitFlags<TransmuteMe>]) -> &'a [u8] {
///     unsafe {
///         slice::from_raw_parts(input.as_ptr() as *const u8, input.len())
///     }
/// }
///
/// let many_flags = &[
///     TransmuteMe::One.into(),
///     TransmuteMe::One | TransmuteMe::Two,
/// ];
///
/// let as_nums = transmute_slice(many_flags);
/// assert_eq!(as_nums, &[0b01, 0b11]);
/// ```
///
/// ## Implementation notes
///
/// You might expect this struct to be defined as
///
/// ```ignore
/// struct BitFlags<T: BitFlag> {
///     value: T::Numeric
/// }
/// ```
///
/// Ideally, that would be the case. However, because `const fn`s cannot
/// have trait bounds in current Rust, this would prevent us from providing
/// most `const fn` APIs. As a workaround, we define `BitFlags` with two
/// type parameters, with a default for the second one:
///
/// ```ignore
/// struct BitFlags<T, N = <T as BitFlag>::Numeric> {
///     value: N,
///     marker: PhantomData<T>,
/// }
/// ```
///
/// The types substituted for `T` and `N` must always match, creating a
/// `BitFlags` value where that isn't the case is considered to be impossible
/// without unsafe code.
#[derive(Copy, Clone, Hash)]
#[repr(transparent)]
pub struct BitFlags<T, N = <T as _internal::RawBitFlags>::Numeric> {
    val: N,
    marker: PhantomData<T>,
}

/// The default value returned is one with all flags unset, i. e. [`empty`][Self::empty].
impl<T> Default for BitFlags<T>
where
    T: BitFlag,
{
    fn default() -> Self {
        Self::empty()
    }
}

impl<T: BitFlag> From<T> for BitFlags<T> {
    fn from(t: T) -> BitFlags<T> {
        Self::from_flag(t)
    }
}

impl<T> BitFlags<T>
where
    T: BitFlag,
{
    /// Create a new BitFlags unsafely, without checking if the bits form
    /// a valid bit pattern for the type.
    ///
    /// Consider using `from_bits` or `from_bits_truncate` instead.
    ///
    /// # Safety
    ///
    /// The argument must not have set bits at positions not corresponding to
    /// any flag.
    pub unsafe fn from_bits_unchecked(val: T::Numeric) -> Self {
        BitFlags { val, marker: PhantomData }
    }

    /// Create a `BitFlags` with no flags set (in other words, with a value of `0`).
    ///
    /// See also: [`BitFlag::empty`], a convenience reexport.
    ///
    /// [`BitFlag::empty`]: trait.BitFlag.html#method.empty
    ///
    /// ```
    /// # use enumflags2::{bitflags, BitFlags};
    /// #[bitflags]
    /// #[repr(u8)]
    /// #[derive(Clone, Copy, PartialEq, Eq)]
    /// enum MyFlag {
    ///     One = 1 << 0,
    ///     Two = 1 << 1,
    ///     Three = 1 << 2,
    /// }
    ///
    /// let empty: BitFlags<MyFlag> = BitFlags::empty();
    /// assert!(empty.is_empty());
    /// assert_eq!(empty.contains(MyFlag::One), false);
    /// assert_eq!(empty.contains(MyFlag::Two), false);
    /// assert_eq!(empty.contains(MyFlag::Three), false);
    /// ```
    pub fn empty() -> Self {
        unsafe { BitFlags::from_bits_unchecked(T::Numeric::default()) }
    }

    /// Create a `BitFlags` with all flags set.
    ///
    /// See also: [`BitFlag::all`], a convenience reexport.
    ///
    /// [`BitFlag::all`]: trait.BitFlag.html#method.all
    ///
    /// ```
    /// # use enumflags2::{bitflags, BitFlags};
    /// #[bitflags]
    /// #[repr(u8)]
    /// #[derive(Clone, Copy, PartialEq, Eq)]
    /// enum MyFlag {
    ///     One = 1 << 0,
    ///     Two = 1 << 1,
    ///     Three = 1 << 2,
    /// }
    ///
    /// let empty: BitFlags<MyFlag> = BitFlags::all();
    /// assert!(empty.is_all());
    /// assert_eq!(empty.contains(MyFlag::One), true);
    /// assert_eq!(empty.contains(MyFlag::Two), true);
    /// assert_eq!(empty.contains(MyFlag::Three), true);
    /// ```
    pub fn all() -> Self {
        unsafe { BitFlags::from_bits_unchecked(T::ALL_BITS) }
    }

    /// An empty `BitFlags`. Equivalent to [`empty()`],
    /// but works in a const context.
    ///
    /// [`empty()`]: #method.empty
    pub const EMPTY: Self = BitFlags { val: T::EMPTY, marker: PhantomData };

    /// A `BitFlags` with all flags set. Equivalent to [`all()`],
    /// but works in a const context.
    ///
    /// [`all()`]: #method.all
    pub const ALL: Self = BitFlags { val: T::ALL_BITS, marker: PhantomData };

    /// Returns true if all flags are set
    pub fn is_all(self) -> bool {
        self.val == T::ALL_BITS
    }

    /// Returns true if no flag is set
    pub fn is_empty(self) -> bool {
        self.val == Self::empty().bits()
    }

    /// Returns the underlying type value
    pub fn bits(self) -> T::Numeric {
        self.val
    }

    /// Returns true if at least one flag is shared.
    pub fn intersects<B: Into<BitFlags<T>>>(self, other: B) -> bool {
        (self.bits() & other.into().bits()) > Self::empty().bits()
    }

    /// Returns true if all flags are contained.
    pub fn contains<B: Into<BitFlags<T>>>(self, other: B) -> bool {
        let other = other.into();
        (self.bits() & other.bits()) == other.bits()
    }

    /// Returns a `BitFlags<T>` if the raw value provided does not contain
    /// any illegal flags.
    pub fn from_bits(bits: T::Numeric) -> Result<Self, FromBitsError<T>> {
        let flags = Self::from_bits_truncate(bits);
        if flags.bits() == bits {
            Ok(flags)
        } else {
            Err(FromBitsError {
                flags,
                invalid: bits & !flags.bits(),
            })
        }
    }

    /// Turn a `T` into a `BitFlags<T>`. Also available as `flag.into()`.
    pub fn from_flag(flag: T) -> Self {
        unsafe { Self::from_bits_unchecked(flag.bits()) }
    }

    /// Truncates flags that are illegal
    pub fn from_bits_truncate(bits: T::Numeric) -> Self {
        unsafe { BitFlags::from_bits_unchecked(bits & T::ALL_BITS) }
    }

    /// Toggles the matching bits
    pub fn toggle<B: Into<BitFlags<T>>>(&mut self, other: B) {
        *self ^= other.into();
    }

    /// Inserts the flags into the BitFlag
    pub fn insert<B: Into<BitFlags<T>>>(&mut self, other: B) {
        *self |= other.into();
    }

    /// Removes the matching flags
    pub fn remove<B: Into<BitFlags<T>>>(&mut self, other: B) {
        *self &= !other.into();
    }

    /// Returns an iterator that yields each set flag
    pub fn iter(self) -> impl Iterator<Item = T> {
        T::FLAG_LIST.iter().cloned().filter(move |&flag| self.contains(flag))
    }
}

impl<T, B> cmp::PartialEq<B> for BitFlags<T>
where
    T: BitFlag,
    B: Into<BitFlags<T>> + Copy,
{
    fn eq(&self, other: &B) -> bool {
        self.bits() == Into::<Self>::into(*other).bits()
    }
}

impl<T, B> ops::BitOr<B> for BitFlags<T>
where
    T: BitFlag,
    B: Into<BitFlags<T>>,
{
    type Output = BitFlags<T>;
    fn bitor(self, other: B) -> BitFlags<T> {
        unsafe { BitFlags::from_bits_unchecked(self.bits() | other.into().bits()) }
    }
}

impl<T, B> ops::BitAnd<B> for BitFlags<T>
where
    T: BitFlag,
    B: Into<BitFlags<T>>,
{
    type Output = BitFlags<T>;
    fn bitand(self, other: B) -> BitFlags<T> {
        unsafe { BitFlags::from_bits_unchecked(self.bits() & other.into().bits()) }
    }
}

impl<T, B> ops::BitXor<B> for BitFlags<T>
where
    T: BitFlag,
    B: Into<BitFlags<T>>,
{
    type Output = BitFlags<T>;
    fn bitxor(self, other: B) -> BitFlags<T> {
        unsafe { BitFlags::from_bits_unchecked(self.bits() ^ other.into().bits()) }
    }
}

impl<T, B> ops::BitOrAssign<B> for BitFlags<T>
where
    T: BitFlag,
    B: Into<BitFlags<T>>,
{
    fn bitor_assign(&mut self, other: B) {
        *self = *self | other;
    }
}

impl<T, B> ops::BitAndAssign<B> for BitFlags<T>
where
    T: BitFlag,
    B: Into<BitFlags<T>>,
{
    fn bitand_assign(&mut self, other: B) {
        *self = *self & other;
    }
}
impl<T, B> ops::BitXorAssign<B> for BitFlags<T>
where
    T: BitFlag,
    B: Into<BitFlags<T>>,
{
    fn bitxor_assign(&mut self, other: B) {
        *self = *self ^ other;
    }
}

impl<T> ops::Not for BitFlags<T>
where
    T: BitFlag,
{
    type Output = BitFlags<T>;
    fn not(self) -> BitFlags<T> {
        unsafe { BitFlags::from_bits_unchecked(!self.bits() & T::ALL_BITS) }
    }
}

impl<T, B> FromIterator<B> for BitFlags<T>
where
    T: BitFlag,
    B: Into<BitFlags<T>>
{
    fn from_iter<I>(it: I) -> BitFlags<T>
    where
        I: IntoIterator<Item = B>
    {
        it.into_iter().fold(BitFlags::empty(), |acc, flag| acc | flag)
    }
}

impl<T, B> Extend<B> for BitFlags<T>
where
    T: BitFlag,
    B: Into<BitFlags<T>>
{
    fn extend<I>(&mut self, it: I)
    where
        I: IntoIterator<Item = B>
    {
        *self = it.into_iter().fold(*self, |acc, flag| acc | flag)
    }
}

#[cfg(feature = "serde")]
mod impl_serde {
    use serde::{Serialize, Deserialize};
    use serde::de::{Error, Unexpected};
    use super::{BitFlags, BitFlag};

    impl<'a, T> Deserialize<'a> for BitFlags<T>
    where
        T: BitFlag,
        T::Numeric: Deserialize<'a> + Into<u64>,
    {
        fn deserialize<D: serde::Deserializer<'a>>(d: D) -> Result<Self, D::Error> {
            let val = T::Numeric::deserialize(d)?;
            Self::from_bits(val)
                .map_err(|_| D::Error::invalid_value(
                    Unexpected::Unsigned(val.into()),
                    &"valid bit representation"
                ))
        }
    }

    impl<T> Serialize for BitFlags<T>
    where
        T: BitFlag,
        T::Numeric: Serialize,
    {
        fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            T::Numeric::serialize(&self.val, s)
        }
    }
}
