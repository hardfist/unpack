use codspeed_criterion_compat::{criterion_group, criterion_main, Criterion};

fn module_graph_benchmark(c: &mut Criterion) {
    c.bench_function("example_benchmark", |b| {
        b.iter(|| {
            // Add your benchmark code here
            // Example: simple computation
            (0..100).fold(0, |a, b| a + b)
        });
    });
}

criterion_group!(benches, module_graph_benchmark);
criterion_main!(benches);
