
use criterion::{criterion_group, criterion_main, Criterion};
use rusthour::bench_breadth_first;


pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("breadth first", |b| b.iter(bench_breadth_first));
}

// criterion_group!(benches, criterion_benchmark);
criterion_group!{
    name = benches;
    // This can be any expression that returns a `Criterion` object.
    config = Criterion::default().sample_size(10);
    targets = criterion_benchmark
}
criterion_main!(benches);