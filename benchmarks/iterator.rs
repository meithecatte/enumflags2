use criterion::{black_box, criterion_group, criterion_main, Criterion};
use enumflags2::{bitflags, BitFlags};

#[bitflags]
#[repr(u16)]
#[derive(Clone, Copy, Debug)]
pub enum Test {
    Flag1 = 1 << 0,
    Flag2 = 1 << 1,
    Flag3 = 1 << 2,
    Flag4 = 1 << 3,
    Flag5 = 1 << 4,
    Flag6 = 1 << 5,
    Flag7 = 1 << 6,
    Flag8 = 1 << 7,
    Flag9 = 1 << 8,
    Flag10 = 1 << 9,
    Flag11 = 1 << 10,
    Flag12 = 1 << 11,
}

pub fn iterators(c: &mut Criterion) {
    let v1 = BitFlags::<Test>::from_bits(0x003).unwrap();
    let v2 = BitFlags::<Test>::from_bits(0x691).unwrap();
    let v3 = BitFlags::<Test>::from_bits(0xfed).unwrap();

    c.bench_function("iterate (2/12)", |b| {
        b.iter(|| black_box(&v1).iter().collect::<Vec<_>>())
    });

    c.bench_function("iterate (5/12)", |b| {
        b.iter(|| black_box(&v2).iter().collect::<Vec<_>>())
    });

    c.bench_function("iterate (10/12)", |b| {
        b.iter(|| black_box(&v3).iter().collect::<Vec<_>>())
    });
}

criterion_group!(benches, iterators);
criterion_main!(benches);
