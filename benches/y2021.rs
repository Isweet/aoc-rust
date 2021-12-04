use criterion::{criterion_group, criterion_main};

#[path = "y2021/day1.rs"]
pub mod day1;
#[path = "y2021/day2.rs"]
pub mod day2;

criterion_group!(
    benches,
    day1::b_1star,
    day1::b_2star,
    day2::b_1star,
    day2::b_2star
);
criterion_main!(benches);
