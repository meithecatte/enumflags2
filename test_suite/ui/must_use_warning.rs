use enumflags2::{bitflags, BitFlags};

#[bitflags]
#[repr(u8)]
#[derive(Clone, Copy, Debug)]
enum Flag {
    Foo = 1 << 0,
    Bar = 1 << 1,
}

fn main() {
    let mut thing: BitFlags<Flag> = Flag::Foo.into();
    let _ = thing;
    // let's pretend the unused_mut warning doesn't trigger
    thing = Flag::Bar.into();
    thing.union_c(Flag::Foo.into());
}
