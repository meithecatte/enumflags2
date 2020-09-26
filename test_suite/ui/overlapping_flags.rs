#[enumflags2::bitflags]
#[repr(u8)]
#[derive(Copy, Clone)]
enum Foo {
    SomeFlag = 1 << 0,
    OverlappingFlag = 1 << 0,
}

fn main() {}
