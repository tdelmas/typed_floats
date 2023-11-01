#[cfg(test)]
use typed_floats::tf64::NonNaN;

#[test]
fn test_others_f64() {
    let values_f64 = [
        (f64::NAN, false),
        (f64::INFINITY, true),
        (f64::NEG_INFINITY, true),
        (0.0f64, true),
        (-0.0f64, true),
        (1.0f64, true),
        (-1.0f64, true),
    ];

    for (value, expected) in &values_f64 {
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

#[test]
fn test_others_i64() {
    let values_i64 = [
        (0_i64, true),
        (1_i64, true),
        (-1_i64, true),
        (i64::MAX, true),
    ];

    for (value, expected) in &values_i64 {
        let num = NonNaN::try_from(*value);
        let result = num.is_ok();
        assert_eq!(result, *expected);
    }
}
