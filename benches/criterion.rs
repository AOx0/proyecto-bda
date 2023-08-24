//! See:
//! - https://github.com/bheisler/criterion.rs
//! - https://bheisler.github.io/criterion.rs/book/index.html

#[allow(unused_imports)]
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use proyecto_bd::test;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("test", |b| b.iter(test));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
