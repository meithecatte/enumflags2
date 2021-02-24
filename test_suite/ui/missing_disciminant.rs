#[enumflags2::bitflags]
#[repr(u8)]
#[derive(Copy, Clone)]
enum Foo {
    OhNoTheresNoDiscriminant,
    WhatWillTheMacroDo,
}

fn main() {}
