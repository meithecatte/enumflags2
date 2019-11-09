/// ```compile_fail
/// #[derive(Copy, Clone, enumflags2::BitFlags)]
/// struct Foo(u16);
/// ```
///
/// ```compile_fail
/// #[derive(Copy, Clone, enumflags2::BitFlags)]
/// enum Foo {
///     OhNoTheresNoDiscriminant,
///     WhatWillTheMacroDo,
/// }
/// ```
///
/// ```compile_fail
/// #[derive(Copy, Clone, enumflags2::BitFlags)]
/// enum Foo {
///     BigNumber = 0xdeadbeefcafebabe1337,
/// }
/// ```
///
/// ```compile_fail
/// #[derive(Copy, Clone, enumflags2::BitFlags)]
/// enum Foo {
///     SingleBit = 1,
///     MultipleBits = 6,
/// }
/// ```
///
/// ```compile_fail
/// #[derive(Copy, Clone, enumflags2::BitFlags)]
/// enum Foo {
///     SomeFlag = 1 << 0,
///     OverlappingFlag = 1 << 0,
/// }
/// ```
///
/// ```compile_fail
/// const THREE: u8 = 3;
/// #[derive(Copy, Clone, enumflags2::BitFlags)]
/// #[repr(u8)]
/// enum Foo {
///     Three = THREE,
/// }
/// ```
#[allow(dead_code)]
fn compile_fail() {}
