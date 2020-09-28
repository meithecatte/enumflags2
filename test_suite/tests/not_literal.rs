#![forbid(trivial_numeric_casts)]

const FOO_BAR: u8 = 1;
const FOO_BAZ: u8 = 2;

#[enumflags2::bitflags]
#[derive(Clone, Copy)]
#[repr(u8)]
enum Foo {
    Bar = FOO_BAR,
    Baz = FOO_BAZ,
}

#[enumflags2::bitflags]
#[derive(Clone, Copy)]
#[repr(u8)]
enum SingleTest {
    Hello = FOO_BAR,
}
