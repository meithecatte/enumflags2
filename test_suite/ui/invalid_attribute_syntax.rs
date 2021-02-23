use enumflags2::bitflags;

#[bitflags(default = A + B)]
enum Test {
    A = 1,
    B = 2,
}

#[bitflags(default = A |)]
enum Test {
    A = 1,
    B = 2,
}

#[bitflags(default =)]
enum Test {
    A = 1,
    B = 2,
}

#[bitflags(default)]
enum Test {
    A = 1,
    B = 2,
}

#[bitflags(yes)]
enum Test {
    A = 1,
    B = 2,
}

fn main() {}
