#[enumflags2::bitflags]
#[repr(C)]
#[derive(Clone, Copy)]
enum NotAType {
    Bar = 1,
    Baz = 2,
}

#[enumflags2::bitflags]
#[repr(i32)]
#[derive(Clone, Copy)]
enum SignedType {
    Bar = 1,
    Baz = 2,
}

#[enumflags2::bitflags]
#[repr(usize)]
#[derive(Clone, Copy)]
enum Usize {
    Bar = 1,
    Baz = 2,
}

fn main() {}
