#![no_implicit_prelude]

#[macro_use]
extern crate enumflags2;
use enumflags2::BitFlags;

#[bitflags]
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum Test {
    A = 1 << 0,
    B = 1 << 1,
    C = 1 << 2,
    D = 1 << 3,
}

#[test]
fn test_foo() {
    // assert!() doesn't even work in no_implicit_prelude!
    let _ = BitFlags::<Test>::all();
}
