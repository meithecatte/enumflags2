#[enumflags2::bitflags]
#[derive(Copy, Clone)]
enum Foo {
    SingleBit = 1,
    MultipleBits = 6,
}

fn main() {}
