#[derive(Copy, Clone, enumflags2::BitFlags)]
enum Foo {
    SomeFlag = 1 << 0,
    OverlappingFlag = 1 << 0,
}

fn main() {}
