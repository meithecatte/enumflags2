extern crate enumflags2;
extern crate core;

const ZERO: u8 = 0;

#[enumflags2::bitflags]
#[derive(Copy, Clone)]
#[repr(u8)]
enum Foo {
    Zero = ZERO,
}

#[enumflags2::bitflags]
#[derive(Copy, Clone)]
#[repr(u8)]
enum Bar {
    Overflown = (ZERO + 2) << 7,
}

fn main() {}
