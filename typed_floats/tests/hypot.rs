#![cfg(any(feature = "std", feature = "libm"))]

use typed_floats::*;

typed_floats_macros::generate_tests_self_rhs!(hypot);

#[test]
fn test_hypot() {
    let a = 3.0f64;
    let b = 4.0f64;
    let c = 5.0f64;

    let a = NonNaN::try_from(a).unwrap();
    let b = NonNaN::try_from(b).unwrap();
    let c = Positive::try_from(c).unwrap();

    let result = a.hypot(b);
    assert_eq!(result, c);
}
