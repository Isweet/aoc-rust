use aoc_rust::y2021::day2::{parse_input, solution_1star, solution_2star};
use criterion::{black_box, Criterion};

pub fn b_1star(c: &mut Criterion) {
    let input = parse_input("res/2021/day2-challenge.txt");
    c.bench_function("solution_1star", |b| {
        b.iter(|| solution_1star(black_box(&input)))
    });
}

pub fn b_2star(c: &mut Criterion) {
    let input = parse_input("res/2021/day2-challenge.txt");
    c.bench_function("solution_2star", |b| {
        b.iter(|| solution_2star(black_box(&input)))
    });
}
