// "an inner attribute is not permitted in this context" :/
#[deny(clippy::all, clippy::pedantic, clippy::nursery)]
mod everything {

    use enumflags2::bitflags;

    include!("../common.rs");
}
