[![LICENSE](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE-MIT)
[![LICENSE](https://img.shields.io/badge/license-apache-blue.svg)](LICENSE-APACHE)
[![Documentation](https://docs.rs/enumflags2/badge.svg)](https://docs.rs/enumflags2)
[![Crates.io Version](https://img.shields.io/crates/v/enumflags2.svg)](https://crates.io/crates/enumflags2)

# Enumflags

`enumflags2` defines a `BitFlags<T>` type, which is a `Set<T>`
for enums without associated data.

## Usage

In your `Cargo.toml`:
```Toml
[dependencies]
enumflags2 = "^0.6"
```

## Features

- [x] Uses enums to represent individual flags&mdash;a set of flags is a separate type from a single flag.
- [x] Detects incorrect BitFlags at compile time.
  - Non-unique bits.
  - Missing values.
  - Flags larger than the chosen `repr`.
- [x] Has a similar API compared to the popular [bitflags](https://crates.io/crates/bitflags) crate.
- [x] Does not expose the generated types explicity. The user interacts exclusively with `struct BitFlags<Enum>;`.
- [x] The debug formatter prints the binary flag value as well as the flag enums: `BitFlags(0b1111, [A, B, C, D])`.
- [x] Optional support for serialization with the [`serde`](https://serde.rs/) feature flag.

### Example

```rust
use enumflags2::BitFlags;

#[derive(BitFlags, Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
enum Test {
    A = 0b0001,
    B = 0b0010,
    C = 0b0100,
    D = 0b1000,
}

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
```

### Note

By default, the `BitFlags` are `usize`-sized. If you want them to be smaller,
specify a `repr` on your enum as in the example above.

### Migrating from 0.5

The minimum rustc version has been bumped to 1.34.0, because of `syn 1.0`. The
version policy from now on will be "what's available on Debian stable", [because
Debian is famously slow with new software versions][debian-snailpace].

You should no longer depend on `enumflags2_derive` directly.
Use the reexport from the `enumflags2` crate.
semver guarantees will be violated if you depend on the derive crate directly.

The derive macro has been renamed to `BitFlags`, to make it clearer what the
derive does.

The `nostd` feature flag has been removed. The crate now only depends on `libcore`
by default. Enable the `std` flag to get an implementation of `std::error::Error`
on error types.

Flags more than one bit set have been found to have inconsistent semantics.
They are now rejected at compile-time. The same applies to flags without any
bit set. If you were relying on this in your code, please [open an issue][issue]
and explain your usecase.

`BitFlags::from_bits` returns a `Result` instead of an `Option`. This might
necessitate some minor changes in your code.

`BitFlags::not` has been removed. Use the `!` operator instead.

[debian-snailpace]: https://www.jwz.org/blog/2016/04/i-would-like-debian-to-stop-shipping-xscreensaver/
[issue]: https://github.com/NieDzejkob/enumflags2/issues/new
