use super::{BitFlag, EnumFlags, EnumFlagsConst};
use super::repr::BitFlagRepr;

pub trait BitFlagExt
    : BitFlag
{
    #[inline]
    fn into_flags(self) -> Self::Flags {
        Self::Flags::from_flag(self)
    }

    #[inline]
    /// Return the bits as a number type.
    fn into_bits(self) -> Self::Type {
        self.into_repr()
    }

    #[inline]
    fn from_bit(bit: Self::Type) -> Option<Self> {
        Self::from_repr(bit).ok()
    }

    #[inline]
    fn try_from_bit(bit: Self::Type) -> Result<Self, Self::Type> {
        Self::from_repr(bit)
    }

    #[inline]
    unsafe fn from_bit_unchecked(bit: Self::Type) -> Self {
        Self::from_repr_unchecked(bit)
    }

    #[inline]
    fn from_bits(bits: Self::Type) -> Option<Self::Flags> {
        Self::Flags::from_bits(bits)
    }

    #[inline]
    fn try_from_bits(bits: Self::Type) -> Result<Self::Flags, Self::Type> {
        Self::Flags::from_repr(bits)
    }

    #[inline]
    fn from_bits_truncate(bits: Self::Type) -> Self::Flags {
        Self::Flags::from_bits_truncate(bits)
    }

    #[inline]
    unsafe fn from_bits_unchecked(bits: Self::Type) -> Self::Flags {
        Self::Flags::from_repr_unchecked(bits)
    }
}

impl<T: BitFlag> BitFlagExt for T { }

pub trait BitFlagExtConst
    : BitFlag
{
    const ALL: Self::Flags;

    const EMPTY: Self::Flags;
}

impl<T: BitFlag> BitFlagExtConst for T
where
    T::Flags: EnumFlagsConst,
{
    const ALL: Self::Flags = Self::Flags::ALL;
    const EMPTY: Self::Flags = Self::Flags::EMPTY;
}
