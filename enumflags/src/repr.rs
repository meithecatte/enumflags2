use super::BitFlag;
use core::cmp;
use core::ops::{Not,
    BitAnd, BitOr, BitXor,
    BitAndAssign, BitOrAssign, BitXorAssign,
};

/// A raw representation of a set of `BitFlag`s
pub trait BitFlagNum
    : Default
    + BitOr<Self, Output = Self>
    + BitOrAssign<Self>
    + BitAnd<Self, Output = Self>
    + BitAndAssign<Self>
    + BitXor<Self, Output = Self>
    + BitXorAssign<Self>
    + Not<Output = Self>
    + cmp::PartialOrd<Self>
    + Copy
    + Clone
{
    const EMPTY: Self;
}

impl BitFlagNum for u8 { const EMPTY: Self = 0; }
impl BitFlagNum for u16 { const EMPTY: Self = 0; }
impl BitFlagNum for u32 { const EMPTY: Self = 0; }
impl BitFlagNum for u64 { const EMPTY: Self = 0; }
impl BitFlagNum for usize { const EMPTY: Self = 0; }

/// A representation of a flag or set of `BitFlag`s
///
/// Implementing this trait is unsafe because it must only return
/// bit patterns that represent a valid set of `T` flags.
pub unsafe trait BitFlagRepr<T: BitFlag>
    : Sized
{
    #[inline]
    fn into_repr(self) -> T::Type { self.get_repr() }
    fn from_repr(repr: T::Type) -> Result<Self, T::Type>;
    unsafe fn from_repr_unchecked(repr: T::Type) -> Self;

    fn get_repr(&self) -> T::Type;

    unsafe fn set_repr(&mut self, val: T::Type) -> bool {
        if let Ok(s) = Self::from_repr(val) {
            *self = s;
            true
        } else {
            false
        }
    }

    #[inline]
    unsafe fn set_repr_unchecked(&mut self, val: T::Type) {
        *self = Self::from_repr_unchecked(val)
    }

    #[inline]
    fn contains_repr(&self, r: T::Type) -> bool {
        self.get_repr() & r == r
    }
}
