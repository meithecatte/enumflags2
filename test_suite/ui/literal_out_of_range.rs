//#[enumflags2::bitflags]
#[repr(u64)]
#[derive(Copy, Clone)]
enum Foo {
    BigNumber = 0xdeadbeefcafebabe1337,
}

fn main() {}
