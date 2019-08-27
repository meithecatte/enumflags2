use core::convert::TryFrom;
use super::{BitFlags, FromBitsError};
use super::_internal::RawBitFlags;

macro_rules! impl_try_from {
    () => { };
    ($ty:ty, $($tt:tt)*) => {
        impl_try_from! { $ty }
        impl_try_from! { $($tt)* }
    };
    ($ty:ty) => {
        impl<T> TryFrom<$ty> for BitFlags<T>
        where
            T: RawBitFlags<Type=$ty>,
        {
            type Error = FromBitsError<T>;

            fn try_from(bits: T::Type) -> Result<Self, Self::Error> {
                Self::try_from_bits(bits)
            }
        }
    };
}

impl_try_from! {
    u8, u16, u32, u64, usize
}
