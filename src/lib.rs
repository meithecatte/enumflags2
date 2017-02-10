extern crate num;

use num::{Unsigned, Zero};
use std::ops::{BitOr, BitAnd, BitXor, Not};
use std::cmp;

pub trait EnumFlagSize {
    type Size: InnerBitflag;
}

pub trait InnerBitflag: BitOr<Self> + cmp::PartialEq + cmp::Eq
    where Self: Sized{
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
pub struct Bitflag<T: EnumFlagSize> {
    val: T::Size,
}

impl<T> ::std::fmt::Debug for Bitflag<T>
    where T: EnumFlagSize,
          T::Size: ::std::fmt::Debug
{
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        fmt.write_str(&format!("Bitflag {o} {inner:?} {c} ",
                               o = "{",
                               inner = self.val,
                               c = "}"))
    }
}

impl<T> Bitflag<T>
    where T: EnumFlagSize
{
    /// Create a new Bitflag unsafely. Consider using `from_bits` or `from_bits_truncate`.
    pub unsafe fn new(val: T::Size) -> Self {
        Bitflag { val: val }
    }
}

impl<T> Bitflag<T>
    where T: EnumFlagSize,
          T::Size: InnerBitflag + Into<Bitflag<T>>
{
    /// Create an empty Bitflag. Empty means `0`.
    pub fn empty() -> Self{
        T::Size::empty().into()
    }

    /// Sets all flags.
    pub fn all() -> Self{
        T::Size::all().into()
    }

    /// Returns true if all flags are set
    pub fn is_all(self) -> bool{
        self.val.is_all()
    }

    /// Returns true if no flag is set
    pub fn is_empty(self) -> bool{
        self.val.is_empty()
    }

    /// Returns the underlying type value
    pub fn bits(self) -> <T::Size as InnerBitflag>::Type {
        self.val.bits()
    }

    /// Returns true if at least one flag is shared.
    pub fn intersects<B: Into<Bitflag<T>>>(self, other: B) -> bool{
        T::Size::intersects(self.val, other.into().val)
    }

    /// Returns true iff all flags are contained.
    pub fn contains<B: Into<Bitflag<T>>>(self, other: B) -> bool{
        T::Size::contains(self.val, other.into().val)
    }

    /// Flips all flags
    pub fn not(self) -> Self{
        self.val.not().into()
    }

    /// Returns a Bitflag iff the bits value does not contain any illegal flags.
    pub fn from_bits(bits: <T::Size as InnerBitflag>::Type) -> Option<Self>{
        T::Size::from_bits(bits).map(|v| v.into())
    }

    /// Truncates flags that are illegal
    pub fn from_bits_truncate(bits: <T::Size as InnerBitflag>::Type) -> Self{
        T::Size::from_bits_truncate(bits).into()
    }

    pub fn toggle(&mut self, other: Self){
        T::Size::toggle(&mut self.val, other.val);
    }

    pub fn insert(&mut self, other: Self){
        T::Size::insert(&mut self.val, other.val);
    }

    pub fn remove(&mut self, other: Self){
        T::Size::remove(&mut self.val, other.val);
    }
}

impl<T> std::cmp::PartialEq for Bitflag<T>
    where T: EnumFlagSize
{
    fn eq(&self, other: &Self) -> bool {
        self.val == other.val
    }
}

impl<T> std::ops::BitOr for Bitflag<T>
    where T: EnumFlagSize ,
          T::Size: BitOr<T::Size, Output = T::Size> + Into<Bitflag<T>>
{
    type Output = Bitflag<T>;
    fn bitor(self, other: Self) -> Bitflag<T> {
        (self.val | other.val).into()
    }
}

impl<T> std::ops::BitAnd for Bitflag<T>
    where T: EnumFlagSize,
          T::Size: BitAnd<T::Size, Output = T::Size> + Into<Bitflag<T>>
{
    type Output = Bitflag<T>;
    fn bitand(self, other: Self) -> Bitflag<T> {
        (self.val & other.val).into()
    }
}

impl<T> std::ops::BitXor for Bitflag<T>
    where T: EnumFlagSize,
          T::Size: BitXor<T::Size, Output = T::Size> + Into<Bitflag<T>>
{
    type Output = Bitflag<T>;
    fn bitxor(self, other: Self) -> Bitflag<T> {
        (self.val ^ other.val).into()
    }
}

impl<T> std::ops::Not for Bitflag<T>
    where T: EnumFlagSize,
          T::Size: Not<Output = T::Size> + Into<Bitflag<T>>
{
    type Output = Bitflag<T>;
    fn not(self) -> Bitflag<T> {
        (!self.val).into()
    }
}
