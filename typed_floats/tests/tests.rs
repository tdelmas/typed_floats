use typed_floats::*;

typed_floats_macros::generate_tests!();

#[test]
fn test_others() {
    let values = [
        (f64::NAN, false),
        (f64::INFINITY, true),
        (f64::NEG_INFINITY, true),
        (0.0f64, true),
        (-0.0f64, true),
        (1.0f64, true),
        (-1.0f64, true),
    ];

    for (value, expected) in &values {
        let num = NonNaN::try_from(*value);
        let result = num.is_ok();
        assert_eq!(result, *expected);

        match num {
            Ok(num) => {
                let v: f64 = num.into();
                assert_eq!(v, *value);
            }
            Err(_) => {}
        }
    }
}
