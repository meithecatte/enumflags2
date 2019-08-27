extern crate enumflags2;
use enumflags2::EnumFlags;

include!("../common.rs");

#[test]
fn module() {
    mod some_modules {
        #[derive(enumflags2::EnumFlags, Copy, Clone, Debug)]
        #[repr(u8)]
        enum Test2 {
            A = 1 << 0,
            B = 1 << 1,
            C = 1 << 2,
            D = 1 << 3,
        }
    }
}
