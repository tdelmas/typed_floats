#![cfg(any(feature = "std", feature = "libm"))]

use typed_floats::*;

typed_floats_macros::generate_tests_self_rhs!(hypot);

#[cfg(feature = "f32")]
#[test]
fn test_hypot_f32() {
    let a = 3.0f32;
    let b = 4.0f32;
    let c = 5.0f32;

    let a = NonNaN::try_from(a).unwrap();
    let b = NonNaN::try_from(b).unwrap();
    let c = Positive::try_from(c).unwrap();

    let result = a.hypot(b);
    assert_eq!(result, c);
}

#[cfg(feature = "f64")]
#[test]
fn test_hypot_f64() {
    let a = 3.0f64;
    let b = 4.0f64;
    let c = 5.0f64;

    let a = NonNaN::try_from(a).unwrap();
    let b = NonNaN::try_from(b).unwrap();
    let c = Positive::try_from(c).unwrap();

    let result = a.hypot(b);
    assert_eq!(result, c);
}
