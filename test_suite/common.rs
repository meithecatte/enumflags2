#[bitflags]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
enum Test {
    A = 1 << 0,
    B = 1 << 1,
    C = 1 << 2,
    D = 1 << 3,
}

#[bitflags]
#[derive(Copy, Clone, Debug)]
#[repr(u64)]
enum Test1 {
    A = 1 << 0,
    B = 1 << 1,
    C = 1 << 2,
    D = 1 << 3,
    E = 1 << 34,
}

#[bitflags(default = B | C)]
#[derive(Copy, Clone, Debug)]
#[repr(u8)]
enum Default6 {
    A = 1 << 0,
    B = 1 << 1,
    C = 1 << 2,
    D = 1 << 3,
}

#[test]
fn test_ctors() {
    use enumflags2::BitFlags;
    assert_eq!(
        BitFlags::<Test>::all(),
        Test::A | Test::B | Test::C | Test::D
    );
    assert_eq!(BitFlags::<Test>::all() & Test::A, Test::A);
    assert_eq!(BitFlags::<Test>::from_bits_truncate(4), Test::C);
    assert_eq!(BitFlags::<Test>::from_bits_truncate(5), Test::A | Test::C);
    assert_eq!(
        BitFlags::<Test>::from_bits_truncate(16),
        BitFlags::<Test>::empty()
    );
    assert_eq!(BitFlags::<Test>::from_bits_truncate(17), Test::A);
    assert!(BitFlags::<Test>::from_bits(17).is_err());
    assert_eq!(
        BitFlags::<Test>::from_bits(15).unwrap(),
        BitFlags::<Test>::all()
    );
    assert_eq!((Test::A | Test::B).bits(), 3);
    assert_eq!((!(Test::A | Test::B)).bits(), 12);
    assert_eq!(BitFlags::<Test>::all().bits(), 15);
    assert_eq!(BitFlags::<Default6>::default(), Default6::B | Default6::C);
}

#[test]
fn test_ops() {
    assert_eq!(!Test::A, Test::B | Test::C | Test::D);
    assert_eq!((Test::A | Test::C) ^ (Test::C | Test::B), Test::A | Test::B);
    assert!((Test::A | Test::B).intersects(Test::B));
    assert!(!(Test::A | Test::B).intersects(Test::C));
    assert!((Test::A | Test::B | Test::C).contains(Test::A | Test::B));
    assert!(!(Test::A | Test::B | Test::C).contains(Test::A | Test::D));
    assert_eq!(!(Test::A | Test::B), Test::C | Test::D);
    assert_eq!((Test::A ^ Test::B), Test::A | Test::B);
}

#[test]
fn test_mutation() {
    {
        let mut b = Test::A | Test::B;
        b.insert(Test::C);
        assert_eq!(b, Test::A | Test::B | Test::C);
    }
    {
        let mut b = Test::A | Test::B | Test::C;
        b.remove(Test::B);
        assert_eq!(b, Test::A | Test::C);
    }
}

#[test]
fn test_exactly_one() {
    use enumflags2::BitFlags;
    assert_eq!(BitFlags::<Test>::empty().exactly_one(), None);
    assert_eq!(BitFlags::<Test>::from(Test::B).exactly_one(), Some(Test::B));
    assert_eq!((Test::A | Test::C).exactly_one(), None);
}

#[test]
fn test_len() {
    use enumflags2::BitFlags;
    assert_eq!(BitFlags::<Test>::empty().len(), 0);
    assert_eq!(BitFlags::<Test>::all().len(), 4);
    assert_eq!((Test::A | Test::D).len(), 2);
}

#[test]
fn iterator() {
    use enumflags2::BitFlags;

    // it's a separate statement because type ascription is nightly
    let tests: &[(BitFlags<Test>, &[Test])] = &[
        (BitFlags::empty(), &[]),
        (Test::A.into(), &[Test::A]),
        (Test::A | Test::B, &[Test::A, Test::B]),
    ];

    for &(bitflag, expected) in tests {
        assert!(bitflag.iter().zip(expected.iter().copied()).all(|(a, b)| a == b));
        // If cloned, the iterator will yield the same elements.
        let it = bitflag.iter();
        assert!(it.clone().zip(it).all(|(a, b)| a == b));
        // The ExactLenIterator implementation should always yield the
        // correct remaining length.
        let mut it = bitflag.iter();
        for rest in (0..=expected.len()).rev() {
            assert_eq!(it.len(), rest);
            assert_eq!(it.size_hint(), (rest, Some(rest)));
            it.next();
        }
    }
}

#[test]
fn assign_ops() {
    let mut x = Test::A | Test::B;
    x |= Test::C;
    assert_eq!(x, Test::A | Test::B | Test::C);

    let mut x = Test::A | Test::B;
    x &= Test::B | Test::C;
    assert_eq!(x, Test::B);

    let mut x = Test::A | Test::B;
    x ^= Test::B | Test::C;
    assert_eq!(x, Test::A | Test::C);
}

#[test]
const fn fn_derive() {
    #[bitflags]
    #[derive(Copy, Clone, Debug)]
    #[repr(u8)]
    enum TestFn {
        A = 1 << 0,
    }
}

#[test]
const fn module() {
    mod some_modules {
        #[enumflags2::bitflags]
        #[derive(Copy, Clone, Debug)]
        #[repr(u8)]
        enum Test2 {
            A = 1 << 0,
            B = 1 << 1,
            C = 1 << 2,
            D = 1 << 3,
        }
    }
}

#[test]
fn inferred_values() {
    #[bitflags]
    #[derive(Copy, Clone, Debug)]
    #[repr(u8)]
    enum Inferred {
        Infer2,
        SpecifiedA = 1,
        Infer8,
        SpecifiedB = 4,
    }

    #[bitflags]
    #[derive(Copy, Clone, Debug)]
    #[repr(u8)]
    enum OnlyInferred {
        Infer1,
        Infer2,
        Infer4,
        Infer8,
    }

    assert_eq!(Inferred::Infer2 as u8, 2);
    assert_eq!(Inferred::Infer8 as u8, 8);

    assert_eq!(OnlyInferred::Infer1 as u8, 1);
    assert_eq!(OnlyInferred::Infer2 as u8, 2);
    assert_eq!(OnlyInferred::Infer4 as u8, 4);
    assert_eq!(OnlyInferred::Infer8 as u8, 8);
}
