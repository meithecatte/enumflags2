extern crate enumflags;
#[macro_use]
extern crate enumflags_derive;
use enumflags::*;

#[derive(EnumFlags, Copy, Clone, Debug)]
#[EnumFlags(ty="u8")]
#[repr(u8)]
pub enum Test {
    A = 0b001,
    B = 0b010,
    C = 0b10000000,
}

fn main() {
    let b = Test::A | Test::B;
    println!("{:?}", Test::from_bitflag(Test::max_bitflag()));
    println!("{:?}", Test::from_bitflag(Test::empty_bitflag()));
}
