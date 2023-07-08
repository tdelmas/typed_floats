#![feature(test)]

use typed_floats::*;

extern crate test;

use test::Bencher;

fn do_some_math_f64(a: f64, b: f64) -> f64 {
    let c = a + b;
    let d = c / 0.5;

    d
}

fn some_math_f64() {
    let a = 5.0;
    let mut b = 2.0;

    let one = 1.0;

    let mut c = 0.0;

    while b < 10000.0 {
        b += one;
        c += do_some_math_f64(a, b);
    }

    assert!(c >= 0.0);
}

fn do_some_math(a: StrictlyPositiveFinite<f64>, b: StrictlyPositive<f64>) -> Positive<f64> {
    let c = a + b;
    let d = c / unsafe { StrictlyPositiveFinite::<f64>::new_unchecked(0.5) };

    d
}

fn some_math() {
    let a = unsafe { StrictlyPositiveFinite::<f64>::new_unchecked(5.0) };
    let mut b = unsafe { StrictlyPositive::<f64>::new_unchecked(2.0) };

    let one = unsafe { StrictlyPositiveFinite::<f64>::new_unchecked(1.0) };

    let mut c = 0.0;
    while b.get() < 10000.0 {
        b += one;
        c += do_some_math(a, b).get();
    }

    assert!(c >= 0.0);
}

#[bench]
fn bench_f64(b: &mut Bencher) {
    b.iter(|| {
        some_math_f64();
    });
}

#[bench]
fn bench_type(b: &mut Bencher) {
    b.iter(|| {
        some_math();
    });
}
