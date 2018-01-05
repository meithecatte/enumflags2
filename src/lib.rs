#[cfg(feature = "nostd")]
extern crate core as std;
extern crate num;

use std::ops::{BitOr, BitAnd, BitXor, Not};
use std::cmp;
use std::cmp::{PartialEq, Eq,PartialOrd};
use std::fmt::{self, Formatter, Debug};
use num::{Num, Zero};

pub trait BitFlagNum: Num + BitOr<Self, Output=Self>
    + BitAnd<Self, Output=Self>
    + BitXor<Self, Output=Self>
    + Not<Output=Self>
    + PartialOrd<Self>
    + Copy + Clone {}

impl BitFlagNum for u8 {}
impl BitFlagNum for u16 {}
impl BitFlagNum for u32 {}
impl BitFlagNum for u64 {}
impl BitFlagNum for usize {}

pub trait BitFlagsFmt
where Self: RawBitFlags {
    fn fmt(flags: BitFlags<Self>, f: &mut Formatter) -> fmt::Result;
}

pub trait RawBitFlags: Copy + Clone {
    type Type: BitFlagNum;

    fn all() -> Self::Type;
    fn bits(self) -> Self::Type;

    fn empty() -> Self::Type {
        Self::Type::zero()
    }

    fn intersects(l: Self::Type, r: Self::Type) -> bool {
        (l & r) > Self::Type::zero()
    }

    fn contains(l: Self::Type, r: Self::Type) -> bool {
        (l & r) == r
    }

    fn xor(l: Self::Type, r: Self::Type) -> Self::Type {
        (l ^ r) & Self::all()
    }

    fn not(val: Self::Type) -> Self::Type {
        !val & Self::all()
    }

    fn from_bits(bits: Self::Type) -> Option<Self::Type> {
        if bits & !Self::all() == Self::empty() {
            Some(bits)
        }
        else{
            None
        }
    }

    fn from_bits_truncate(bits: Self::Type) -> Self::Type{
        bits & Self::all()
    }
}

#[derive(Copy, Clone)]
pub struct BitFlags<T: RawBitFlags> {
    val: T::Type
}

impl<T> ::std::fmt::Debug for BitFlags<T>
    where T: RawBitFlags + BitFlagsFmt
{
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        T::fmt(self.clone(), fmt)
    }
}

impl<T> BitFlags<T>
    where T: RawBitFlags
{
    /// Create a new BitFlags unsafely. Consider using `from_bits` or `from_bits_truncate`.
    pub unsafe fn new(val: T::Type) -> Self {
        BitFlags { val }
    }
}

impl<T: RawBitFlags> From<T> for BitFlags<T>{
    fn from(t: T) -> BitFlags<T>{
        BitFlags{
            val: t.bits()
        }
    }
}

impl<T> BitFlags<T>
    where 
          T: RawBitFlags + Copy,
T::Type: Copy
{
    /// Create an empty BitFlags. Empty means `0`.
    pub fn empty() -> Self {
        unsafe{BitFlags::new(T::empty())}
    }

    /// Sets all flags.
    pub fn all() -> Self {
        unsafe{BitFlags::new(T::all())}
    }

    /// Returns true if all flags are set
    pub fn is_all(self) -> bool {
        self.val == T::all()
    }

    /// Returns true if no flag is set
    pub fn is_empty(self) -> bool {
        self.val == T::empty()
    }

    /// Returns the underlying type value
    pub fn bits(self) -> T::Type {
        self.val
    }

    /// Returns true if at least one flag is shared.
    pub fn intersects<B: Into<BitFlags<T>>>(self, other: B) -> bool {
        T::intersects(self.val, other.into().bits())
    }

    /// Returns true iff all flags are contained.
    pub fn contains<B: Into<BitFlags<T>>>(self, other: B) -> bool {
        T::contains(self.bits(), other.into().bits())
    }

    /// Flips all flags
    pub fn not(self) -> Self {
        unsafe{BitFlags::new(T::not(self.bits()))}
    }

    /// Returns a BitFlags iff the bits value does not contain any illegal flags.
    pub fn from_bits(bits: T::Type) -> Option<Self> {
        T::from_bits(bits).map(|val| unsafe {BitFlags::new(val) })
    }

    /// Truncates flags that are illegal
    pub fn from_bits_truncate(bits: T::Type) -> Self {
        unsafe { BitFlags::new(T::from_bits_truncate(bits)) }
    }

    pub fn toggle(&mut self, other: Self) {
        *self = *self ^ other;
    }

    pub fn insert(&mut self, other: Self) {
        *self = *self | other;
    }

    pub fn remove(&mut self, other: Self) {
        *self = *self & !other;
    }
}

impl<T, B> std::cmp::PartialEq<B> for BitFlags<T>
    where T: RawBitFlags,
          B: Into<BitFlags<T>> + Copy
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
        unsafe{BitFlags::new(self.bits() | other.into().bits())}
    }
}

impl<T, B> std::ops::BitAnd<B> for BitFlags<T>
    where
    T: RawBitFlags,
    B: Into<BitFlags<T>>,
{
    type Output = BitFlags<T>;
    fn bitand(self, other: B) -> BitFlags<T> {
        unsafe{BitFlags::new(self.bits() & other.into().bits())}
    }
}

impl<T, B> std::ops::BitXor<B> for BitFlags<T>
    where
    T: RawBitFlags,
    B: Into<BitFlags<T>>,
{
    type Output = BitFlags<T>;
    fn bitxor(self, other: B) -> BitFlags<T> {
        unsafe{BitFlags::new(T::xor(self.bits(), other.into().bits()))}
    }
}

impl<T> std::ops::Not for BitFlags<T>
    where T: RawBitFlags
{
    type Output = BitFlags<T>;
    fn not(self) -> BitFlags<T> {
        self.not()
    }
}
