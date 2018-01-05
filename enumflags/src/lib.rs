#[cfg(feature = "nostd")]
extern crate core as std;
extern crate num;

use std::ops::{BitAnd, BitOr, BitXor, Not};
use std::cmp;
use std::cmp::{Eq, PartialEq, PartialOrd};
use std::fmt::{self, Debug, Formatter};
use num::{Num, Zero};

pub trait BitFlagNum
    : Num
    + BitOr<Self, Output = Self>
    + BitAnd<Self, Output = Self>
    + BitXor<Self, Output = Self>
    + Not<Output = Self>
    + PartialOrd<Self>
    + Copy
    + Clone {
}

impl BitFlagNum for u8 {}
impl BitFlagNum for u16 {}
impl BitFlagNum for u32 {}
impl BitFlagNum for u64 {}
impl BitFlagNum for usize {}

pub trait BitFlagsFmt
where
    Self: RawBitFlags,
{
    fn fmt(flags: BitFlags<Self>, f: &mut Formatter) -> fmt::Result;
}

pub trait RawBitFlags: Copy + Clone {
    type Type: BitFlagNum;
    fn all() -> Self::Type;
    fn bits(self) -> Self::Type;
}

#[derive(Copy, Clone)]
pub struct BitFlags<T: RawBitFlags> {
    val: T::Type,
}

impl<T> ::std::fmt::Debug for BitFlags<T>
where
    T: RawBitFlags + BitFlagsFmt,
{
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        T::fmt(self.clone(), fmt)
    }
}

impl<T> BitFlags<T>
where
    T: RawBitFlags,
{
    /// Create a new BitFlags unsafely. Consider using `from_bits` or `from_bits_truncate`.
    pub unsafe fn new(val: T::Type) -> Self {
        BitFlags { val }
    }
}

impl<T: RawBitFlags> From<T> for BitFlags<T> {
    fn from(t: T) -> BitFlags<T> {
        BitFlags { val: t.bits() }
    }
}

impl<T> BitFlags<T>
where
    T: RawBitFlags,
{
    /// Create an empty BitFlags. Empty means `0`.
    pub fn empty() -> Self {
        unsafe { BitFlags::new(T::Type::zero()) }
    }

    /// Sets all flags.
    pub fn all() -> Self {
        unsafe { BitFlags::new(T::all()) }
    }

    /// Returns true if all flags are set
    pub fn is_all(self) -> bool {
        self.val == T::all()
    }

    /// Returns true if no flag is set
    pub fn is_empty(self) -> bool {
        self.val == Self::empty().bits()
    }

    /// Returns the underlying type value
    pub fn bits(self) -> T::Type {
        self.val
    }

    /// Returns true if at least one flag is shared.
    pub fn intersects<B: Into<BitFlags<T>>>(self, other: B) -> bool {
        (self.bits() & other.into().bits()) > Self::empty().bits()
    }

    /// Returns true iff all flags are contained.
    pub fn contains<B: Into<BitFlags<T>>>(self, other: B) -> bool {
        let other = other.into();
        (self.bits() & other.bits()) == other.bits()
    }

    /// Flips all flags
    pub fn not(self) -> Self {
        unsafe { BitFlags::new(!self.bits() & T::all()) }
    }

    /// Returns a BitFlags iff the bits value does not contain any illegal flags.
    pub fn from_bits(bits: T::Type) -> Option<Self> {
        if bits & !Self::all().bits() == Self::empty().bits() {
            unsafe { Some(BitFlags::new(bits)) }
        } else {
            None
        }
    }

    /// Truncates flags that are illegal
    pub fn from_bits_truncate(bits: T::Type) -> Self {
        unsafe { BitFlags::new(bits & T::all()) }
    }

    pub fn toggle<B: Into<BitFlags<T>>>(&mut self, other: B) {
        *self = *self ^ other.into();
    }

    pub fn insert<B: Into<BitFlags<T>>>(&mut self, other: B) {
        *self = *self | other.into();
    }

    pub fn remove<B: Into<BitFlags<T>>>(&mut self, other: B) {
        *self = *self & !other.into();
    }
}

impl<T, B> std::cmp::PartialEq<B> for BitFlags<T>
where
    T: RawBitFlags,
    B: Into<BitFlags<T>> + Copy,
{
    fn eq(&self, other: &B) -> bool {
        self.bits() == Into::<Self>::into(*other).bits()
    }
}

impl<T, B> std::ops::BitOr<B> for BitFlags<T>
where
    T: RawBitFlags,
    B: Into<BitFlags<T>>,
{
    type Output = BitFlags<T>;
    fn bitor(self, other: B) -> BitFlags<T> {
        unsafe { BitFlags::new(self.bits() | other.into().bits()) }
    }
}

impl<T, B> std::ops::BitAnd<B> for BitFlags<T>
where
    T: RawBitFlags,
    B: Into<BitFlags<T>>,
{
    type Output = BitFlags<T>;
    fn bitand(self, other: B) -> BitFlags<T> {
        unsafe { BitFlags::new(self.bits() & other.into().bits()) }
    }
}

impl<T, B> std::ops::BitXor<B> for BitFlags<T>
where
    T: RawBitFlags,
    B: Into<BitFlags<T>>,
{
    type Output = BitFlags<T>;
    fn bitxor(self, other: B) -> BitFlags<T> {
        unsafe { BitFlags::new((self.bits() ^ other.into().bits()) & T::all()) }
    }
}

impl<T> std::ops::Not for BitFlags<T>
where
    T: RawBitFlags,
{
    type Output = BitFlags<T>;
    fn not(self) -> BitFlags<T> {
        self.not()
    }
}
