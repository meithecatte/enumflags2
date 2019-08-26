use enumflags2_derive::EnumFlags;

include!("../common.rs");

#[test]
fn debug_format() {
    use enumflags2::BitFlags;

    // Assert that our Debug output format meets expectations

    assert_eq!(
        format!("{:?}", BitFlags::<Test>::all()),
        "BitFlags<Test>(0b1111, A | B | C | D)"
    );

    assert_eq!(
        format!("{:?}", BitFlags::<Test>::empty()),
        "BitFlags<Test>(0b0)"
    );

    assert_eq!(
        format!("{:04x?}", BitFlags::<Test>::all()),
        "BitFlags<Test>(0x0f, A | B | C | D)"
    );

    assert_eq!(
        format!("{:04X?}", BitFlags::<Test>::all()),
        "BitFlags<Test>(0x0F, A | B | C | D)"
    );

    // Also check alternate struct formatting

    assert_eq!(
        format!("{:#010?}", BitFlags::<Test>::all()),
"BitFlags<Test> {
    bits: 0b00001111,
    flags: A | B | C | D,
}"
    );

    assert_eq!(
        format!("{:#?}", BitFlags::<Test>::empty()),
"BitFlags<Test> {
    bits: 0b0,
}"
    );
}

#[test]
fn format() {
    use enumflags2::BitFlags;

    // Assert BitFlags<T> impls fmt::{Binary, Octal, LowerHex, UpperHex}

    assert_eq!(
        format!("{:b}", BitFlags::<Test>::all()),
        "1111"
    );

    assert_eq!(
        format!("{:o}", BitFlags::<Test>::all()),
        "17"
    );

    assert_eq!(
        format!("{:x}", BitFlags::<Test>::all()),
        "f"
    );

    assert_eq!(
        format!("{:#04X}", BitFlags::<Test>::all()),
        "0x0F"
    );
}
