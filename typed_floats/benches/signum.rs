use criterion::{black_box, criterion_group, criterion_main, Criterion};

use typed_floats::NonNaN;

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

pub fn criterion_benchmark(c: &mut Criterion) {
    let values_f32 = typed_floats::tf32::get_test_values();
    let valid_values_f32: Vec<_> = values_f32
        .iter()
        .filter_map(|&value| NonNaN::<f32>::new(value).ok())
        .collect();

    c.bench_function("signum", |b| {
        b.iter(|| {
            let mut sum = 0.0;
            for i in &valid_values_f32 {
                sum = sum + i.signum();
            }
            black_box(sum);
        });
    });
}
