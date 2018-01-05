extern crate enumflags;
#[macro_use]
extern crate enumflags_derive;

//use enumflags::BitFlags;

#[derive(EnumFlags, Copy, Clone, Debug)]
#[repr(u8)]
enum Test {
    A = 0b0001,
    B = 0b0010,
    C = 0b0100,
    D = 0b1000,
}

#[test]
fn test_foo() {
    panic!()
}
