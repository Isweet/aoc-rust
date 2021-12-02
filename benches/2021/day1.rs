use aoc_rust::y2021::day1::{challenge_input, solution_1star};
use criterion::{black_box, Criterion};

pub fn b_1star(c: &mut Criterion) {
    let input = challenge_input();
    c.bench_function("solution_1star", |b| {
        b.iter(|| solution_1star(black_box(&input)))
    });
}

pub fn b_2star(c: &mut Criterion) {
    let input = challenge_input();
    c.bench_function("solution_2star", |b| {
        b.iter(|| solution_1star(black_box(&input)))
    });
}
