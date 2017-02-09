extern crate num;

use num::{Unsigned, Zero};
use std::ops::BitOr;

pub trait EnumFlagSize {
    type Size: Unsigned;
}

#[derive(Eq, Copy, Clone)]
pub struct Bitflag<T: EnumFlagSize> {
    val: T::Size,
}

impl<T> Bitflag<T> where T: EnumFlagSize {
    pub unsafe fn new(val: T::Size) -> Self {
        Bitflag{
            val: val
        }
    }
}

impl<T> Bitflag<T>
    where T: EnumFlagSize,
          T::Size: Zero + Eq
{
    //pub fn empty() -> T::Size {
    //    T::Size::zero()
    //}

    pub fn is_empty(&self) -> bool {
        self.val == T::Size::zero()
    }
}

impl<T> Bitflag<T>
    where T: EnumFlagSize
{
    pub fn bits(self) -> T::Size {
        self.val
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
    where T: EnumFlagSize + Into<<T as EnumFlagSize>::Size>,
          T::Size: BitOr<T::Size, Output = T::Size>
{
    type Output = Bitflag<T>;
    fn bitor(self, other: Self) -> Self::Output {
        let l: T::Size = self.val.into();
        let r: T::Size = other.val.into();
        Bitflag { val: l | r }
    }
}
