//! # Enum Flags
//! `enumflags2` defines a `BitFlags<T>` type for representing a set of named boolean values
//! efficiently, where `T` is an enum with explicitly defined values. Semantically similar to a
//! `HashSet<EnumWithoutAssociatedData>`, but much more efficient.
//!
//! ## Example
//! ```
//! extern crate enumflags2;
//!
//! use enumflags2::{BitFlags, EnumFlags};
//!
//! #[derive(EnumFlags, Copy, Clone, Debug, PartialEq)]
//! #[repr(u8)]
//! enum Test {
//!     A = 0b0001,
//!     B = 0b0010,
//!     C = 0b0100,
//!     D = 0b1000,
//! }
//!
//! fn main() {
//!     let a_b = Test::A | Test::B; // BitFlags<Test>
//!     let a_c = Test::A | Test::C;
//!     let b_c_d = Test::C | Test::B | Test::D;
//!
//!     // BitFlags<Test>(0b11, [A, B])
//!     println!("{:?}", a_b);
//!
//!     // BitFlags<Test>(0b1, [A])
//!     println!("{:?}", a_b & a_c);
//!
//!     // Iterate over the flags like a normal set!
//!     assert_eq!(a_b.iter().collect::<Vec<_>>(), &[Test::A, Test::B]);
//!
//!     assert!(a_b.contains(Test::A));
//!     assert!(b_c_d.contains(Test::B | Test::C));
//!     assert!(!(b_c_d.contains(a_b)));
//!
//!     assert!(a_b.intersects(a_c));
//!     assert!(!(a_b.intersects(Test::C | Test::D)));
//! }
//! ```
//!
//! ## Optional Feature Flags
//!
//! - [`serde`](https://serde.rs/) implements `Serialize` and `Deserialize` for `BitFlags<T>`.
//#![warn(missing_docs)]
#![cfg_attr(not(test), no_std)]

#[cfg(test)]
extern crate core;
use core::iter::FromIterator;
use core::ops::{Not,
    BitAnd, BitOr, BitXor,
    BitAndAssign, BitOrAssign, BitXorAssign,
};
use core::cmp;
use repr::{BitFlagRepr, BitFlagNum};

#[allow(unused_imports)]
#[macro_use]
extern crate enumflags2_derive;

#[doc(hidden)]
pub use enumflags2_derive::EnumFlags_internal as EnumFlags;

pub mod repr;

/// While the module is public, this is only the case because it needs to be
/// accessed by the derive macro. Do not use this directly. Stability guarantees
/// don't apply.
#[doc(hidden)]
pub mod _internal {
    // Re-export libcore so the macro doesn't inject "extern crate" downstream.
    pub mod core {
        pub use core::{convert, mem, option, ops, result};
    }
}

// Internal debug formatting implementations
mod formatting;

mod ext;
pub use ext::*;

/// A single flag in a type-safe set of flags.
///
/// Automatically implemented by `derive(EnumFlags)` to make the enum
/// a valid type parameter for `BitFlags<T>`.
pub trait BitFlag
    : Copy + Clone
    + BitFlagRepr<Self>
    + BitOr<Self, Output = <Self as BitFlag>::Flags>
    + BitAnd<Self, Output = <Self as BitFlag>::Flags>
    + BitXor<Self, Output = <Self as BitFlag>::Flags>
    + Not<Output = <Self as BitFlag>::Flags>
    + 'static
{
    /// The underlying integer type.
    type Type: BitFlagNum;

    /// A type-safe set of flags.
    type Flags: EnumFlags<Flag=Self> + BitFlagRepr<Self>;

    /// Return a value with all flag bits set.
    const ALL_BITS: Self::Type;

    /// Return a slice that contains each variant exactly one.
    fn flag_list() -> &'static [Self];

    #[inline]
    /// Return the name of the type for debug formatting purposes.
    ///
    /// This is typically `BitFlags<EnumName>`
    fn bitflags_type_name() -> &'static str {
        "BitFlags"
    }
}

/// A set of bit flags.
///
/// This trait is unsafe to impl because it assumes that the underlying `BitFlagRepr` is
/// capable of representing a set of flags (and any combination rather than a single enum flag).
pub unsafe trait EnumFlags
    : Sized
    + BitFlagRepr<<Self as EnumFlags>::Flag>
{
    type Flag: BitFlag;

    #[inline]
    /// Create a new BitFlags unsafely. Consider using `from_bits` or `from_bits_truncate`.
    unsafe fn new(val: <Self::Flag as BitFlag>::Type) -> Self {
        <Self as BitFlagRepr<Self::Flag>>::from_repr_unchecked(val)
    }

    #[inline]
    /// Create an empty BitFlags. Empty means `0`.
    fn empty() -> Self {
        unsafe { Self::new(<Self::Flag as BitFlag>::Type::EMPTY) }
    }

    #[inline]
    /// Create a BitFlags with all flags set.
    fn all() -> Self {
        unsafe { Self::new(Self::Flag::ALL_BITS) }
    }

    #[inline]
    /// Returns true if all flags are set
    fn is_all(self) -> bool {
        self.bits() == <Self::Flag as BitFlag>::ALL_BITS
    }

    #[inline]
    /// Returns true if no flag is set
    fn is_empty(self) -> bool {
        self.bits() == <Self::Flag as BitFlag>::Type::EMPTY
    }

    #[inline]
    /// Returns the underlying type value
    fn bits(self) -> <Self::Flag as BitFlag>::Type {
        self.into_repr()
    }

    #[inline]
    /// Returns a BitFlags iff the bits value does not contain any illegal flags.
    fn from_bits(bits: <Self::Flag as BitFlag>::Type) -> Option<Self> {
        Self::from_repr(bits).ok()
    }

    #[inline]
    /// Truncates flags that are illegal
    fn from_bits_truncate(bits: <Self::Flag as BitFlag>::Type) -> Self {
        unsafe { Self::new(bits & Self::Flag::ALL_BITS) }
    }

    #[inline]
    fn from_flag(flag: Self::Flag) -> Self {
        unsafe { Self::new(flag.into_bits()) }
    }

    #[inline]
    fn from_flags<B: BitFlagRepr<Self::Flag>>(flags: B) -> Self {
        unsafe { Self::new(flags.into_repr()) }
    }

    /// Returns true if at least one flag is shared.
    fn intersects<B: BitFlagRepr<Self::Flag>>(self, other: B) -> bool {
        (self.bits() & other.into_repr()) != <Self::Flag as BitFlag>::Type::EMPTY
    }

    #[inline]
    /// Returns true iff all flags are contained.
    fn contains<B: BitFlagRepr<Self::Flag>>(self, other: B) -> bool {
        self.contains_repr(other.into_repr())
    }

    #[inline]
    fn bits_or<B: BitFlagRepr<Self::Flag>>(self, other: B) -> Self {
        unsafe {
            Self::from_repr_unchecked(self.into_repr() | other.into_repr())
        }
    }

    #[inline]
    fn bits_and<B: BitFlagRepr<Self::Flag>>(self, other: B) -> Self {
        unsafe {
            Self::from_repr_unchecked(self.into_repr() & other.into_repr())
        }
    }

    #[inline]
    fn bits_xor<B: BitFlagRepr<Self::Flag>>(self, other: B) -> Self {
        unsafe {
            Self::from_repr_unchecked(self.into_repr() ^ other.into_repr())
        }
    }

    #[inline]
    fn bits_not(self) -> Self {
        unsafe {
            Self::from_repr_unchecked((!self.into_repr()) & Self::Flag::ALL_BITS)
        }
    }

    #[inline]
    /// Toggles the matching bits
    fn toggle<B: BitFlagRepr<Self::Flag>>(&mut self, other: B) {
        unsafe {
            self.set_repr_unchecked(self.get_repr() ^ other.into_repr())
        }
    }

    #[inline]
    /// Inserts the flags into the BitFlag
    fn insert<B: BitFlagRepr<Self::Flag>>(&mut self, other: B) {
        unsafe {
            self.set_repr_unchecked(self.get_repr() | other.into_repr())
        }
    }

    #[inline]
    fn mask<B: BitFlagRepr<Self::Flag>>(&mut self, other: B) {
        unsafe {
            self.set_repr_unchecked(self.get_repr() & other.into_repr())
        }
    }

    #[inline]
    /// Removes the matching flags
    fn remove<B: BitFlagRepr<Self::Flag>>(&mut self, other: B) {
        unsafe {
            self.set_repr_unchecked(self.get_repr() & !other.into_repr())
        }
    }

    #[inline]
    /// Returns an iterator that yields each set flag
    fn iter(self) -> Self::IntoIter
    where
        Self: IntoIterator<Item=<Self as EnumFlags>::Flag>,
    {
        self.into_iter()
    }
}

pub trait EnumFlagsConst
    : EnumFlags
{
    const ALL: Self;

    const EMPTY: Self;
}

/// Represents a set of flags of some type `T`.
/// The type must have the `#[derive(EnumFlags)]` attribute applied.
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct BitFlags<T: BitFlag> {
    val: T::Type,
}

impl<T: BitFlag> EnumFlagsConst for BitFlags<T> {
    const ALL: Self = Self { val: T::ALL_BITS };
    const EMPTY: Self = Self { val: T::Type::EMPTY };
}

unsafe impl<T: BitFlag> BitFlagRepr<T> for BitFlags<T> {
    #[inline]
    fn into_repr(self) -> T::Type {
        self.val
    }

    #[inline]
    fn get_repr(&self) -> T::Type {
        self.val
    }

    #[inline]
    unsafe fn from_repr_unchecked(val: T::Type) -> Self {
        Self { val }
    }

    unsafe fn set_repr_unchecked(&mut self, val: T::Type) {
        self.val = val;
    }

    fn from_repr(repr: T::Type) -> Result<Self, T::Type> {
        let left = repr & !T::ALL_BITS;
        if left == T::Type::EMPTY {
            unsafe { Ok(Self::from_repr_unchecked(repr)) }
        } else {
            Err(left)
        }
    }
}

/// The default value returned is one with all flags unset, i. e. [`empty`][Self::empty].
impl<T> Default for BitFlags<T>
where
    T: BitFlag,
{
    fn default() -> Self {
        Self::EMPTY
    }
}

impl<T: BitFlag> From<T> for BitFlags<T> {
    #[inline]
    fn from(t: T) -> BitFlags<T> {
        Self::from_flag(t)
    }
}

unsafe impl<T> EnumFlags for BitFlags<T>
where
    T: BitFlag,
{
    type Flag = T;
}

pub struct BitFlagsIter<T: BitFlag> {
    flags: BitFlags<T>,
    iter: core::slice::Iter<'static, T>,
}

impl<T> Iterator for BitFlagsIter<T>
where
    T: BitFlag,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(&flag) = self.iter.next() {
            if self.flags.contains(flag) {
                return Some(flag)
            }
        }

        None
    }
}

impl<T> IntoIterator for BitFlags<T>
where
    T: BitFlag,
{
    type Item = T;
    type IntoIter = BitFlagsIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        BitFlagsIter {
            flags: self,
            iter: T::flag_list().iter(),
        }
    }
}

impl<T, B> cmp::PartialEq<B> for BitFlags<T>
where
    T: BitFlag,
    B: BitFlagRepr<T>,
{
    #[inline]
    fn eq(&self, other: &B) -> bool {
        self.get_repr() == other.get_repr()
    }
}

impl<T, B> BitOr<B> for BitFlags<T>
where
    T: BitFlag,
    B: BitFlagRepr<T>,
{
    type Output = BitFlags<T>;

    #[inline]
    fn bitor(self, other: B) -> BitFlags<T> {
        self.bits_or(other)
    }
}

impl<T, B> BitAnd<B> for BitFlags<T>
where
    T: BitFlag,
    B: BitFlagRepr<T>,
{
    type Output = BitFlags<T>;

    #[inline]
    fn bitand(self, other: B) -> BitFlags<T> {
        self.bits_and(other)
    }
}

impl<T, B> BitXor<B> for BitFlags<T>
where
    T: BitFlag,
    B: BitFlagRepr<T>,
{
    type Output = BitFlags<T>;

    #[inline]
    fn bitxor(self, other: B) -> BitFlags<T> {
        self.bits_xor(other)
    }
}

impl<T, B> BitOrAssign<B> for BitFlags<T>
where
    T: BitFlag,
    B: BitFlagRepr<T>,
{
    #[inline]
    fn bitor_assign(&mut self, other: B) {
        self.insert(other)
    }
}

impl<T, B> BitAndAssign<B> for BitFlags<T>
where
    T: BitFlag,
    B: BitFlagRepr<T>,
{
    #[inline]
    fn bitand_assign(&mut self, other: B) {
        self.mask(other)
    }
}
impl<T, B> BitXorAssign<B> for BitFlags<T>
where
    T: BitFlag,
    B: BitFlagRepr<T>,
{
    #[inline]
    fn bitxor_assign(&mut self, other: B) {
        self.toggle(other)
    }
}

impl<T> Not for BitFlags<T>
where
    T: BitFlag,
{
    type Output = BitFlags<T>;

    #[inline]
    fn not(self) -> BitFlags<T> {
        self.bits_not()
    }
}

impl<T, B> FromIterator<B> for BitFlags<T>
where
    T: BitFlag,
    B: BitFlagRepr<T>
{
    fn from_iter<I>(it: I) -> BitFlags<T>
    where 
        I: IntoIterator<Item = B>
    {
        let mut flags = BitFlags::EMPTY;
        for flag in it {
            flags |= flag;
        }
        flags
    }
}

#[cfg(feature = "serde")]
mod impl_serde {
    extern crate serde;
    use self::serde::{Serialize, Deserialize};
    use self::serde::de::{Error, Unexpected};
    use super::{BitFlags, _internal::RawBitFlags};

    impl<'a, T> Deserialize<'a> for BitFlags<T>
    where
        T: RawBitFlags,
        T::Type: Deserialize<'a> + Into<u64>,
    {
        fn deserialize<D: serde::Deserializer<'a>>(d: D) -> Result<Self, D::Error> {
            let val = T::Type::deserialize(d)?;
            Self::from_bits(val)
                .ok_or_else(|| D::Error::invalid_value(
                    Unexpected::Unsigned(val.into()),
                    &"valid bit representation"
                ))
        }
    }

    impl<T> Serialize for BitFlags<T>
    where
        T: RawBitFlags,
        T::Type: Serialize,
    {
        fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            T::Type::serialize(&self.val, s)
        }
    }
}
