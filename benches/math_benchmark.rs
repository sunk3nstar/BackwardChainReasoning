use criterion::{Criterion, criterion_group, criterion_main};
use reasoning::bench::bench_bc_math;

pub fn benchmark(c: &mut Criterion) {
    c.bench_function("math_bench", |b| b.iter(|| bench_bc_math()));
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
