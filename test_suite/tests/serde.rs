use enumflags2::{bitflags, BitFlags};
use serde::{Deserialize, Serialize};

#[test]
fn serde_compile() {
    #[bitflags]
    #[derive(Copy, Clone, Debug, Serialize, Deserialize)]
    #[repr(u8)]
    enum Test {
        A = 1 << 0,
        B = 1 << 1,
        C = 1 << 2,
        D = 1 << 3,
    }

    type TestBitFlags = BitFlags<Test>;

    #[derive(Clone, Debug, Serialize, Deserialize)]
    struct TestStructContainsFlags {
        flags: TestBitFlags,
    }
}
