use enumflags2::BitFlags;
use serde::{Serialize, Deserialize};

include!("../common.rs");

#[test]
fn serde_compile() {
    #[derive(enumflags2::BitFlags, Copy, Clone, Debug, Serialize, Deserialize)]
    #[repr(u8)]
    enum Test {
        A = 1 << 0,
        B = 1 << 1,
        C = 1 << 2,
        D = 1 << 3,
    }

    type TestBitFlags = BitFlags<Test>;

    #[derive(Clone, Debug, Serialize, Deserialize)]
    struct TestStructContainsFlags{
        flags: TestBitFlags,
    }
}
