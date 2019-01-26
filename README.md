[![LICENSE](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE-MIT)
[![LICENSE](https://img.shields.io/badge/license-apache-blue.svg)](LICENSE-APACHE)
[![Documentation](https://docs.rs/enumflags/badge.svg)](https://docs.rs/enumflags)
[![Crates.io Version](https://img.shields.io/crates/v/enumflags.svg)](https://crates.io/crates/enumflags)

# Enumflags

## Usage

In your `Cargo.toml`:
```Toml
[dependencies]
enumflags = "^0.4"
enumflags_derive = "^0.4"
```

If using the 2015 Rust edition, add this to your crate root:
```Rust
extern crate enumflags;
#[macro_use]
extern crate enumflags_derive;
```

## Features

- [x] Uses enums to represent individual flags.
- [x] Detects incorrect BitFlags at compile time.
  - Non-unique bits.
  - Missing values.
  - Flags larger than the chosen `repr`.
- [x] Has a similar API compared to the popular [bitflags](https://crates.io/crates/bitflags) crate.
- [x] Does not expose the generated types explicity. The user interacts exclusively with `struct BitFlags<Enum>;`.
- [x] A set of flags is a separate type from a single flag.
- [x] The debug formatter prints the binary flag value as well as the flag enums: `BitFlags { 0b1111, Flags::[A, B, C, D] }`.

### Example

```rust
extern crate enumflags;
#[macro_use]
extern crate enumflags_derive;

use enumflags::BitFlags;

#[derive(EnumFlags, Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
enum Test {
    A = 0b0001,
    B = 0b0010,
    C = 0b0100,
    D = 0b1000,
}

fn main() {
    let a_b = Test::A | Test::B; // BitFlags<Test>
    let a_c = Test::A | Test::C;
    let b_c_d = Test::C | Test::B | Test::D;

    // BitFlags<Test>(0b11, [A, B])
    println!("{:?}", a_b);

    // BitFlags<Test>(0b1, [A])
    println!("{:?}", a_b & a_c);

    // Iterate over the flags like a normal set!
    assert_eq!(a_b.iter().collect::<Vec<_>>(), &[Test::A, Test::B]);

    assert!(a_b.contains(Test::A));
    assert!(b_c_d.contains(Test::B | Test::C));
    assert!(!(b_c_d.contains(a_b)));

    assert!(a_b.intersects(a_c));
    assert!(!(a_b.intersects(Test::C | Test::D)));
}
```
