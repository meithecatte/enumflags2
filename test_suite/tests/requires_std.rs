#![allow(dead_code)]
use enumflags2::{bitflags, BitFlag, BitFlags};

include!("../common.rs");

#[test]
fn debug_format() {
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
        format!("{:?}", BitFlags::from_flag(Test::B)),
        "BitFlags<Test>(0b10, B)"
    );

    assert_eq!(
        format!("{:04x?}", BitFlags::<Test>::all()),
        "BitFlags<Test>(0x0f, A | B | C | D)"
    );

    assert_eq!(
        format!("{:04X?}", BitFlags::<Test>::all()),
        "BitFlags<Test>(0x0F, A | B | C | D)"
    );
}

#[test]
fn debug_format_alternate() {
    /// Handle the slight difference in alternate debug output on rustc 1.34.2.
    fn compare(mut actual: String, expected: &str) {
        if actual.ends_with("\n}") && !actual.ends_with(",\n}") {
            actual.replace_range(actual.len() - 2.., ",\n}");
        }

        assert_eq!(actual, expected);
    }

    compare(
        format!("{:#010?}", BitFlags::<Test>::all()),
        "BitFlags<Test> {
    bits: 0b00001111,
    flags: A | B | C | D,
}",
    );

    compare(
        format!("{:#?}", BitFlags::<Test>::empty()),
        "BitFlags<Test> {
    bits: 0b0,
}",
    );
}

#[test]
fn display_format() {
    // Assert that our Debug output format meets expectations

    assert_eq!(format!("{}", BitFlags::<Test>::all()), "A | B | C | D");

    assert_eq!(format!("{}", BitFlags::<Test>::empty()), "<empty>");

    assert_eq!(format!("{}", BitFlags::from_flag(Test::B)), "B");
}

#[test]
fn format() {
    // Assert BitFlags<T> impls fmt::{Binary, Octal, LowerHex, UpperHex}

    assert_eq!(format!("{:b}", BitFlags::<Test>::all()), "1111");

    assert_eq!(format!("{:o}", BitFlags::<Test>::all()), "17");

    assert_eq!(format!("{:x}", BitFlags::<Test>::all()), "f");

    assert_eq!(format!("{:#04X}", BitFlags::<Test>::all()), "0x0F");
}

#[test]
fn debug_generic() {
    #[derive(Debug)]
    struct Debug<T: BitFlag>(BitFlags<T>);

    let _ = format!("{:?}", Debug(BitFlags::<Test>::all()));
}

fn works_in_maps() {
    // Assert that BitFlags<T> implements Hash and Ord.

    use std::collections::{BTreeSet, HashSet};
    let mut map: BTreeSet<BitFlags<Test>> = BTreeSet::new();
    map.insert(BitFlags::empty());
    let mut map: HashSet<BitFlags<Test>> = HashSet::new();
    map.insert(BitFlags::empty());
}

fn works_in_maps_generic<T: BitFlag>() {
    // Assert that BitFlags<T> implements Hash and Ord.

    use std::collections::{BTreeSet, HashSet};
    let mut map: BTreeSet<BitFlags<T>> = BTreeSet::new();
    map.insert(BitFlags::empty());
    let mut map: HashSet<BitFlags<T>> = HashSet::new();
    map.insert(BitFlags::empty());
}
