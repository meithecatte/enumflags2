#[enumflags2::bitflags]
#[repr(u8)]
#[derive(Copy, Clone)]
enum Foo {
    SingleBit = 1,
    MultipleBits = 6,
}

fn main() {}
