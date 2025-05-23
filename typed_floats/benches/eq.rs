use criterion::{black_box, criterion_group, criterion_main, Criterion};

use typed_floats::NonZeroNonNaN;

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

pub fn criterion_benchmark(c: &mut Criterion) {
    let values_f32 = typed_floats::tf32::get_test_values();
    let valid_values_f32: Vec<_> = values_f32
        .iter()
        .filter_map(|&value| NonZeroNonNaN::<f32>::new(value).ok())
        .collect();

    c.bench_function("eq_non_zero_non_nan_f32", |b| {
        b.iter(|| {
            let mut equals = 0;
            for i in 0..valid_values_f32.len() {
                for j in 0..valid_values_f32.len() {
                    if valid_values_f32[i] == valid_values_f32[j] {
                        equals += 1;
                    }
                }
            }
            black_box(equals);
        });
    });
}
