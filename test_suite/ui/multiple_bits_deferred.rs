const THREE: u8 = 3;

#[enumflags2::bitflags]
#[derive(Copy, Clone)]
#[repr(u8)]
enum Foo {
    Three = THREE,
}

fn main() {}
