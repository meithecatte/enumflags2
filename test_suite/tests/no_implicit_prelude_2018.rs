#![no_implicit_prelude]

#[derive(::enumflags2::EnumFlags, Copy, Clone, Debug, PartialEq)]
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
    use ::enumflags2::BitFlagExtConst;
    let _ = Test::ALL;
}
