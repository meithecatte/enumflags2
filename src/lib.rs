#![no_std]

use core::ops::{BitOr, BitAnd, BitXor, Not};
use core::cmp;

pub trait EnumFlagSize {
    type Size: InnerBitFlags;
}

pub trait InnerBitFlags: BitOr<Self> + cmp::PartialEq + cmp::Eq
    where Self: Sized
{
    type Type;
    fn all() -> Self;
    fn empty() -> Self;
    fn is_empty(self) -> bool;
    fn is_all(self) -> bool;
    fn bits(self) -> Self::Type;
    fn intersects(self, other: Self) -> bool;
    fn contains(self, other: Self) -> bool;
    fn not(self) -> Self;
    fn from_bits(bits: Self::Type) -> Option<Self>;
    fn from_bits_truncate(bits: Self::Type) -> Self;
    fn insert(&mut self, other: Self);
    fn remove(&mut self, other: Self);
    fn toggle(&mut self, other: Self);
}

#[derive(Eq, Copy, Clone)]
pub struct BitFlags<T: EnumFlagSize> {
    val: T::Size,
}

impl<T> ::core::fmt::Debug for BitFlags<T>
    where T: EnumFlagSize,
          T::Size: ::core::fmt::Debug
{
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(fmt,
               "BitFlags {o} {inner:?} {c} ",
               o = "{",
               inner = self.val,
               c = "}")
    }
}

impl<T> BitFlags<T>
    where T: EnumFlagSize
{
    /// Create a new BitFlags unsafely. Consider using `from_bits` or `from_bits_truncate`.
    pub unsafe fn new(val: T::Size) -> Self {
        BitFlags { val: val }
    }
}

impl<T> BitFlags<T>
    where T: EnumFlagSize,
          T::Size: InnerBitFlags + Into<BitFlags<T>>
{
    /// Create an empty BitFlags. Empty means `0`.
    pub fn empty() -> Self {
        T::Size::empty().into()
    }

    /// Sets all flags.
    pub fn all() -> Self {
        T::Size::all().into()
    }

    /// Returns true if all flags are set
    pub fn is_all(self) -> bool {
        self.val.is_all()
    }

    /// Returns true if no flag is set
    pub fn is_empty(self) -> bool {
        self.val.is_empty()
    }

    /// Returns the underlying type value
    pub fn bits(self) -> <T::Size as InnerBitFlags>::Type {
        self.val.bits()
    }

    /// Returns true if at least one flag is shared.
    pub fn intersects<B: Into<BitFlags<T>>>(self, other: B) -> bool {
        T::Size::intersects(self.val, other.into().val)
    }

    /// Returns true iff all flags are contained.
    pub fn contains<B: Into<BitFlags<T>>>(self, other: B) -> bool {
        T::Size::contains(self.val, other.into().val)
    }

    /// Flips all flags
    pub fn not(self) -> Self {
        self.val.not().into()
    }

    /// Returns a BitFlags iff the bits value does not contain any illegal flags.
    pub fn from_bits(bits: <T::Size as InnerBitFlags>::Type) -> Option<Self> {
        T::Size::from_bits(bits).map(|v| v.into())
    }

    /// Truncates flags that are illegal
    pub fn from_bits_truncate(bits: <T::Size as InnerBitFlags>::Type) -> Self {
        T::Size::from_bits_truncate(bits).into()
    }

    pub fn toggle(&mut self, other: Self) {
        T::Size::toggle(&mut self.val, other.val);
    }

    pub fn insert(&mut self, other: Self) {
        T::Size::insert(&mut self.val, other.val);
    }

    pub fn remove(&mut self, other: Self) {
        T::Size::remove(&mut self.val, other.val);
    }
}

impl<T> core::cmp::PartialEq for BitFlags<T>
    where T: EnumFlagSize
{
    fn eq(&self, other: &Self) -> bool {
        self.val == other.val
    }
}

// impl<T> core::ops::BitOr for BitFlags<T>
//    where T: EnumFlagSize ,
//          T::Size: BitOr<T::Size, Output = T::Size> + Into<BitFlags<T>>
// {
//    type Output = BitFlags<T>;
//    fn bitor(self, other: Self) -> BitFlags<T> {
//        (self.val | other.val).into()
//    }
// }

impl<T, B> core::ops::BitOr<B> for BitFlags<T>
    where T: EnumFlagSize,
          B: Into<BitFlags<T>>,
          T::Size: BitOr<T::Size, Output = T::Size> + Into<BitFlags<T>>
{
    type Output = BitFlags<T>;
    fn bitor(self, other: B) -> BitFlags<T> {
        (self.val | other.into().val).into()
    }
}

impl<T, B> core::ops::BitAnd<B> for BitFlags<T>
    where T: EnumFlagSize,
          B: Into<BitFlags<T>>,
          T::Size: BitAnd<T::Size, Output = T::Size> + Into<BitFlags<T>>
{
    type Output = BitFlags<T>;
    fn bitand(self, other: B) -> BitFlags<T> {
        (self.val & other.into().val).into()
    }
}

impl<T, B> core::ops::BitXor<B> for BitFlags<T>
    where T: EnumFlagSize,
          B: Into<BitFlags<T>>,
          T::Size: BitXor<T::Size, Output = T::Size> + Into<BitFlags<T>>
{
    type Output = BitFlags<T>;
    fn bitxor(self, other: B) -> BitFlags<T> {
        (self.val ^ other.into().val).into()
    }
}

impl<T> core::ops::Not for BitFlags<T>
    where T: EnumFlagSize,
          T::Size: Not<Output = T::Size> + Into<BitFlags<T>>
{
    type Output = BitFlags<T>;
    fn not(self) -> BitFlags<T> {
        (!self.val).into()
    }
}
