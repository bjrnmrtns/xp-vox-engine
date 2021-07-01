use criterion::{black_box, criterion_group, criterion_main, Criterion};
use xp_vox_engine::chunker::greedy_meshing::greedy_mesh;

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| greedy_mesh()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
