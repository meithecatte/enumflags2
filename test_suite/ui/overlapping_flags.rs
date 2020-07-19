#[enumflags2::bitflags]
#[derive(Copy, Clone)]
enum Foo {
    SomeFlag = 1 << 0,
    OverlappingFlag = 1 << 0,
}

fn main() {}
