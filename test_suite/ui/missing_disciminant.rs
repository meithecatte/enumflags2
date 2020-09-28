#[enumflags2::bitflags]
#[derive(Copy, Clone)]
enum Foo {
    OhNoTheresNoDiscriminant,
    WhatWillTheMacroDo,
}

fn main() {}
