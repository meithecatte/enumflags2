use enumflags2::bitflags;

#[bitflags(default = A | C)]
#[repr(u8)]
#[derive(Clone, Copy)]
enum Test {
    A = 1,
    B = 2,
}

fn main() {}
