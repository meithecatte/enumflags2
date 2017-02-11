extern crate enumflags;
#[macro_use]
extern crate enumflags_derive;
use enumflags::*;

#[derive(EnumFlags, Copy, Clone, Debug)]
#[repr(u8)]
pub enum Test {
    A = 0b0001,
    B = 0b0010,
    C = 0b0100,
    D = 0b1000,
}

fn print_test<B: Into<BitFlags<Test>>>(bitflag: B) {
    println!("{:?}", bitflag.into());
}

fn main() {
    // BitFlags { 0b1111, Flags::[A, B, C, D] }
    print_test(BitFlags::<Test>::all());

    // BitFlags { 0b1111, Flags::[A, B, C, D] }
    print_test(BitFlags::all());

    // BitFlags { 0b0, Flags::[] }
    print_test(BitFlags::empty());

    // BitFlags { 0b101, Flags::[A, C] }
    print_test(BitFlags::from_bits_truncate(5));

    // BitFlags { 0b100, Flags::[C] }
    print_test(BitFlags::from_bits_truncate(4));

    // BitFlags { 0b0, Flags::[] }
    print_test(BitFlags::from_bits(16).unwrap_or(BitFlags::empty()));

    // BitFlags { 0b100, Flags::[C] }
    print_test(BitFlags::from_bits(4).unwrap());

    // BitFlags { 0b11111110, Flags::[B, C, D] }
    print_test(!Test::A);

    // BitFlags { 0b11, Flags::[A, B] }
    let flag1 = Test::A | Test::B;
    print_test(flag1);

    // BitFlags { 0b1100, Flags::[C, D] }
    let flag2 = Test::C | Test::D;
    print_test(flag2);

    // BitFlags { 0b1001, Flags::[A, D] }
    let flag3 = Test::A | Test::D;
    print_test(flag3);

    // BitFlags { 0b0, Flags::[] }
    print_test(flag1 & flag2);

    // BitFlags { 0b1111, Flags::[A, B, C, D] }
    print_test(flag1 | flag2);

    // false
    println!("{}", flag1.intersects(flag2));

    // true
    println!("{}", flag1.intersects(flag3));

    // true
    println!("{}", flag1.contains(Test::A | Test::B));

    // false
    println!("{}", flag1.contains(Test::A | Test::B | Test::C));

    // true
    println!("{}", flag1.contains(Test::A));
}
