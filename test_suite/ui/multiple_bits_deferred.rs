const THREE: u8 = 3;

#[derive(Copy, Clone, enumflags2::BitFlags)]
#[repr(u8)]
enum Foo {
    Three = THREE,
}

fn main() {}
