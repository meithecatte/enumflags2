use enumflags2::{bitflags, make_bitflags};

#[bitflags]
#[repr(u8)]
#[derive(Copy, Clone, Debug)]
enum Test {
    A = 1,
    B = 2,
}

impl Test {
    const C: u8 = 69;
}

fn main() {
    let x = make_bitflags!(Test::{C});
    dbg!(x);
}
