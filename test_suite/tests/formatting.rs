use enumflags2::EnumFlags;

include!("../common.rs");

#[test]
fn debug_format() {
    use enumflags2::BitFlagExtConst;

    // Assert that our Debug output format meets expectations

    assert_eq!(
        format!("{:?}", Test::ALL),
        "BitFlags<Test>(0b1111, A | B | C | D)"
    );

    assert_eq!(
        format!("{:?}", Test::EMPTY),
        "BitFlags<Test>(0b0)"
    );

    assert_eq!(
        format!("{:04x?}", Test::ALL),
        "BitFlags<Test>(0x0f, A | B | C | D)"
    );

    assert_eq!(
        format!("{:04X?}", Test::ALL),
        "BitFlags<Test>(0x0F, A | B | C | D)"
    );

    // Also check alternate struct formatting

    assert_eq!(
        format!("{:#010?}", Test::ALL),
"BitFlags<Test> {
    bits: 0b00001111,
    flags: A | B | C | D,
}"
    );

    assert_eq!(
        format!("{:#?}", Test::EMPTY),
"BitFlags<Test> {
    bits: 0b0,
}"
    );
}

#[test]
fn format() {
    use enumflags2::BitFlagExtConst;

    // Assert BitFlags<T> impls fmt::{Binary, Octal, LowerHex, UpperHex}

    assert_eq!(
        format!("{:b}", Test::ALL),
        "1111"
    );

    assert_eq!(
        format!("{:o}", Test::ALL),
        "17"
    );

    assert_eq!(
        format!("{:x}", Test::ALL),
        "f"
    );

    assert_eq!(
        format!("{:#04X}", Test::ALL),
        "0x0F"
    );
}
