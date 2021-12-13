#[enumflags2::bitflags]
#[derive(Copy, Clone)]
struct Foo(u16);

#[enumflags2::bitflags]
const WTF: u8 = 42;

fn main() {}
