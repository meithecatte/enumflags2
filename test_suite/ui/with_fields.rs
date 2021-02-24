#[enumflags2::bitflags]
#[repr(u8)]
#[derive(Copy, Clone)]
enum Foo {
    Bar(u32),
}

fn main() {}
