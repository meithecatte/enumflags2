extern crate enumflags2;
extern crate core;

#[enumflags2::bitflags]
#[repr(u8)]
#[derive(Copy, Clone)]
enum Foo {
    Zero = 0,
}

fn main() {}
