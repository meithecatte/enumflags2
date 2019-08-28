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
#![warn(missing_docs)]
#![cfg_attr(not(test), no_std)]

#[cfg(test)]
extern crate core;
use core::iter::FromIterator;
use core::ops::{self, BitAnd, BitOr, BitXor, Not};
use core::cmp;

#[allow(unused_imports)]
#[macro_use]
extern crate enumflags2_derive;

#[doc(hidden)]
pub use enumflags2_derive::EnumFlags_internal as EnumFlags;

/// While the module is public, this is only the case because it needs to be
/// accessed by the derive macro. Do not use this directly. Stability guarantees
/// don't apply.
#[doc(hidden)]
pub mod _internal {
    // Re-export libcore so the macro doesn't inject "extern crate" downstream.
    pub mod core {
        pub use core::{convert, option, ops};
    }
}

// Internal debug formatting implementations
mod formatting;

pub trait BitFlagNum
    : Default
    + BitOr<Self, Output = Self>
    + BitAnd<Self, Output = Self>
    + BitXor<Self, Output = Self>
    + Not<Output = Self>
    + cmp::PartialOrd<Self>
    + Copy
    + Clone {
}

impl BitFlagNum for u8 {}
impl BitFlagNum for u16 {}
impl BitFlagNum for u32 {}
impl BitFlagNum for u64 {}
impl BitFlagNum for usize {}

/// A trait automatically implemented by `derive(EnumFlags)` to make the enum
/// a valid type parameter for `BitFlags<T>`.
pub trait BitFlag
    : Copy + Clone
    + BitOr<Self, Output = <Self as BitFlag>::Flags>
    + BitAnd<Self, Output = <Self as BitFlag>::Flags>
    + BitXor<Self, Output = <Self as BitFlag>::Flags>
    + Not<Output = <Self as BitFlag>::Flags>
    //+ Into<<Self as BitFlag>::Flags>
    + 'static
{
    /// The underlying integer type.
    type Type: BitFlagNum;

    /// A type-safe representation of multiple flags.
    type Flags: EnumFlags;

    /// Return a value with all flag bits set.
    fn all() -> Self::Type;

    /// Return the bits as a number type.
    fn bits(self) -> Self::Type;

    /// Return a slice that contains each variant exactly one.
    fn flag_list() -> &'static [Self];

    fn into_flags(self) -> Self::Flags;

    /// Return the name of the type for debug formatting purposes.
    ///
    /// This is typically `BitFlags<EnumName>`
    fn bitflags_type_name() -> &'static str {
        "BitFlags"
    }
}

/*
/// Constraints that apply to all types using `derive(EnumFlags)`
pub trait BitFlagBlanket
    : BitFlag
    + BitOr<Self, Output = <Self as BitFlag>::Flags>
    + BitAnd<Self, Output = <Self as BitFlag>::Flags>
    + BitXor<Self, Output = <Self as BitFlag>::Flags>
    + Not<Output = <Self as BitFlag>::Flags>
{ }

impl<T> BitFlagBlanket for T
where T:
    BitFlag
    + BitOr<Self, Output = Self::Flags>
    + BitAnd<Self, Output = Self::Flags>
    + BitXor<Self, Output = Self::Flags>
    + Not<Output = Self::Flags>
{ }*/

/*impl<T: BitFlag> BitFlag for BitFlags<T>
{
    type Type = T::Type;
    type Flags = Self;

    fn all() -> Self::Type { Self::Type::all() }
    fn bits(self) -> Self::Type { self.val }
    fn flag_list() -> &'static [Self] { Self::Type::flag_list() }
    fn into_flags(self) -> Self::Flags { self }
    fn bitflags_type_name() -> &'static str { Self::Type::bitflags_type_name() }
}*/

pub trait EnumFlags
    : Sized
{
    type Flag: BitFlag;

    /// Create a new BitFlags unsafely. Consider using `from_bits` or `from_bits_truncate`.
    unsafe fn new(val: <Self::Flag as BitFlag>::Type) -> Self;

    #[inline]
    /// Create an empty BitFlags. Empty means `0`.
    fn empty() -> Self {
        unsafe { Self::new(Default::default()) }
    }

    #[inline]
    /// Create a BitFlags with all flags set.
    fn all() -> Self {
        unsafe { Self::new(Self::Flag::all()) }
    }

    #[inline]
    /// Returns true if all flags are set
    fn is_all(self) -> bool {
        self.bits() == <Self::Flag as BitFlag>::all()
    }

    #[inline]
    /// Returns true if no flag is set
    fn is_empty(self) -> bool {
        self.bits() == Self::empty().bits()
    }

    /// Returns the underlying type value
    fn bits(self) -> <Self::Flag as BitFlag>::Type;

    /// Returns a BitFlags iff the bits value does not contain any illegal flags.
    fn from_bits(bits: <Self::Flag as BitFlag>::Type) -> Option<Self> {
        if bits & !Self::all().bits() == Self::empty().bits() {
            unsafe { Some(Self::new(bits)) }
        } else {
            None
        }
    }

    #[inline]
    /// Truncates flags that are illegal
    fn from_bits_truncate(bits: <Self::Flag as BitFlag>::Type) -> Self {
        unsafe { Self::new(bits & Self::Flag::all()) }
    }

    #[inline]
    fn from_flag(flag: Self::Flag) -> Self {
        unsafe { Self::new(flag.bits()) }
    }

    /*
    /// Returns true if at least one flag is shared.
    fn intersects<B: Into<Self>>(self, other: B) -> bool {
        (self.bits() & other.into().bits()) != Self::empty().bits()
    }

    /// Returns true iff all flags are contained.
    fn contains<B: Into<Self>>(self, other: B) -> bool {
        let other = other.into();
        (self.bits() & other.bits()) == other.bits()
    }

    /// Toggles the matching bits
    fn toggle<B: Into<Self>>(&mut self, other: B) {
        *self ^= other.into();
    }

    /// Inserts the flags into the BitFlag
    fn insert<B: Into<Self>>(&mut self, other: B) {
        *self |= other.into();
    }

    /// Removes the matching flags
    fn remove<B: Into<Self>>(&mut self, other: B) {
        *self &= !other.into();
    }
    */

    /// Returns an iterator that yields each set flag
    fn iter(self) -> <Self as IntoIterator>::IntoIter
    where
        Self: IntoIterator,
    {
        self.into_iter()
    }
}

/// Represents a set of flags of some type `T`.
/// The type must have the `#[derive(EnumFlags)]` attribute applied.
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct BitFlags<T: BitFlag> {
    val: T::Type,
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
    #[inline]
    fn from(t: T) -> BitFlags<T> {
        Self::from_flag(t)
    }
}

impl<T> EnumFlags for BitFlags<T>
where
    T: BitFlag,
{
    type Flag = T;

    #[inline]
    unsafe fn new(val: <Self::Flag as BitFlag>::Type) -> Self {
        Self { val }
    }

    #[inline]
    fn bits(self) -> <Self::Flag as BitFlag>::Type {
        self.val
    }
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

impl<T> BitFlags<T>
where
    T: BitFlag,
{
    /// Returns true if at least one flag is shared.
    pub fn intersects<B: Into<BitFlags<T>>>(self, other: B) -> bool {
        (self.bits() & other.into().bits()) != Self::empty().bits()
    }

    /// Returns true iff all flags are contained.
    pub fn contains<B: Into<BitFlags<T>>>(self, other: B) -> bool {
        let other = other.into();
        (self.bits() & other.bits()) == other.bits()
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

impl<T, B> BitOr<B> for BitFlags<T>
where
    T: BitFlag,
    B: Into<BitFlags<T>>,
{
    type Output = BitFlags<T>;
    fn bitor(self, other: B) -> BitFlags<T> {
        unsafe { BitFlags::new(self.bits() | other.into().bits()) }
    }
}

impl<T, B> BitAnd<B> for BitFlags<T>
where
    T: BitFlag,
    B: Into<BitFlags<T>>,
{
    type Output = BitFlags<T>;
    fn bitand(self, other: B) -> BitFlags<T> {
        unsafe { BitFlags::new(self.bits() & other.into().bits()) }
    }
}

impl<T, B> BitXor<B> for BitFlags<T>
where
    T: BitFlag,
    B: Into<BitFlags<T>>,
{
    type Output = BitFlags<T>;
    fn bitxor(self, other: B) -> BitFlags<T> {
        unsafe { BitFlags::new((self.bits() ^ other.into().bits()) & T::all()) }
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

impl<T> Not for BitFlags<T>
where
    T: BitFlag,
{
    type Output = BitFlags<T>;
    fn not(self) -> BitFlags<T> {
        unsafe { BitFlags::new(!self.bits() & T::all()) }
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
        let mut flags = BitFlags::empty();
        for flag in it {
            flags |= flag.into();
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
