use core::convert::TryFrom;
use super::BitFlags;
use super::_internal::RawBitFlags;

macro_rules! impl_try_from {
    ($($ty:ty),*) => {
        $(
            impl<T> TryFrom<$ty> for BitFlags<T>
            where
                T: RawBitFlags<Type=$ty>,
            {
                type Error = FromBitsError<T>;

                fn try_from(bits: T::Type) -> Result<Self, Self::Error> {
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
            }
        )*
    };
}

impl_try_from! {
    u8, u16, u32, u64, usize
}

#[derive(Debug, Copy, Clone)]
pub struct FromBitsError<T: RawBitFlags> {
    flags: BitFlags<T>,
    invalid: T::Type,
}

impl<T: RawBitFlags> FromBitsError<T> {
    pub fn truncate(self) -> BitFlags<T> {
        self.flags
    }

    pub fn invalid_bits(self) -> T::Type {
        self.invalid
    }
}

#[cfg(feature = "std")]
impl<T: RawBitFlags> std::error::Error for FromBitsError<T> {
    fn description(&self) -> &str {
        "invalid bit representation"
    }
}
