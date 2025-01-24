use enumflags2::{bitflags, BitFlags};
use zerocopy::{Immutable, IntoBytes, KnownLayout, TryFromBytes};

#[test]
fn zerocopy_compile() {
    #[bitflags]
    #[derive(Copy, Clone, Debug, KnownLayout)]
    #[repr(u8)]
    enum TestU8 {
        A,
        B,
        C,
        D,
    }

    #[bitflags]
    #[derive(Copy, Clone, Debug, KnownLayout)]
    #[repr(u16)]
    enum TestU16 {
        A,
        B,
        C,
        D,
    }

    #[derive(Clone, Debug, Immutable, TryFromBytes, IntoBytes, KnownLayout)]
    #[repr(packed)]
    struct Other {
        flags2: BitFlags<TestU8>,
        flags: BitFlags<TestU16>,
    }
}
