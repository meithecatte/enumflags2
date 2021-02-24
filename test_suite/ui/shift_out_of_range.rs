#[enumflags2::bitflags]
#[repr(u64)]
#[derive(Copy, Clone)]
enum Foo {
    BigNumber = 1 << 69,
}

#[enumflags2::bitflags]
#[repr(u16)]
#[derive(Copy, Clone)]
enum Bar {
    BigNumber = 1 << 20,
}

#[enumflags2::bitflags]
#[repr(u16)]
#[derive(Copy, Clone)]
enum Baz {
    BigNumber = (1 << 10) << 10,
}

fn main() {}
