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

fn print_test<B: Into<Bitflag<Test>>>(bitflag: B) {
    println!("{:?}", bitflag.into());
}

fn main() {
    let b = Test::A | Test::B;
    let b1 = Test::A | Test::A;
    let c = b1 | b;
    print_test(Bitflag::all());


    // let t: InnerTest = Test::A.into();
    println!("{:?}", c.contains(Test::A | Test::B));
    println!("{:?}", c);
    println!("{:?}", c.not());
    print_test(Bitflag::from_bits_truncate(5));
    print_test(b & b1);
    print_test(!Test::A);
    // println!("{:?}", Test::from_bitflag(Test::max_bitflag()));
    // println!("{:?}", Test::from_bitflag(Test::empty_bitflag()));
}
