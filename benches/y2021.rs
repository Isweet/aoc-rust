use criterion::{criterion_group, criterion_main};

#[path = "2021/day1.rs"]
pub mod day1;

criterion_group!(benches, day1::b_1star, day1::b_2star);
criterion_main!(benches);
