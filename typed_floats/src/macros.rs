/// This macros assert that two values are close to each other.
///
/// # Examples
///
/// ```
/// # use typed_floats::*;
/// assert_relative_eq!(1.0_f64, 1.0);
/// assert_relative_eq!(1.0_f64, 1.000000001, 1e-7);
/// ```
///
/// ```should_panic
/// # use typed_floats::*;
/// assert_relative_eq!(2.0_f64, 1.0);
/// ```
///
/// ```should_panic
/// # use typed_floats::*;
/// assert_relative_eq!(1.0_f64, 1.000001, 1e-7);
/// ```
#[macro_export]
macro_rules! assert_relative_eq {
    ($left:expr, $right:expr) => {{
        let epsilon = 1e-7;

        assert_relative_eq!($left, $right, epsilon);
    }};
    ($left:expr, $right:expr, $epsilon:expr) => {{
        let left_val: f64 = $left.into();
        let right_val: f64 = $right.into();

        assert!(
            (left_val == right_val) || (left_val - right_val).abs() <= $epsilon,
            "assertion failed: `(left ~= right)` \
             (left: `{:?}`, right: `{:?}`, (left - right): `{:?}` > epsilon: `{:?}`)",
            left_val,
            right_val,
            left_val - right_val,
            $epsilon
        );
    }};
}

/// This macros assert that the value is NaN.
///
/// # Examples
///
/// ```
/// # use typed_floats::*;
/// assert_is_nan!(0.0_f64 / 0.0);
/// ```
///
/// ```should_panic
/// # use typed_floats::*;
/// assert_is_nan!(2.0_f64);
/// ```
///
/// ```should_panic
/// # use typed_floats::*;
/// assert_is_nan!(core::f64::INFINITY);
/// ```
#[macro_export]
macro_rules! assert_is_nan {
    ($left:expr) => {{
        let left_val = $left;

        assert!(
            left_val.is_nan(),
            "assertion failed: `(value is NaN)` \
             (value: `{:?}`)",
            left_val,
        );
    }};
}

/// This macros assert that the value is positive zero.
///
/// # Examples
///
/// ```
/// # use typed_floats::*;
/// assert_is_positive_zero!(0.0_f64);
/// ```
///
/// ```should_panic
/// # use typed_floats::*;
/// assert_is_positive_zero!(-0.0_f64);
/// ```
///
/// ```should_panic
/// # use typed_floats::*;
/// assert_is_positive_zero!(core::f64::INFINITY);
/// ```
#[macro_export]
macro_rules! assert_is_positive_zero {
    ($left:expr) => {{
        let val = $left;

        assert!(
            val == 0.0 && val.is_sign_positive(),
            "assertion failed: `(value is positive zero)` \
             (value: `{:?}`)",
            val,
        );
    }};
}

/// This macros assert that the value is negative zero.
///
/// # Examples
///
/// ```
/// # use typed_floats::*;
/// assert_is_negative_zero!(-0.0_f64);
/// ```
///
/// ```should_panic
/// # use typed_floats::*;
/// assert_is_negative_zero!(0.0_f64);
/// ```
///
/// ```should_panic
/// # use typed_floats::*;
/// assert_is_negative_zero!(core::f64::NEG_INFINITY);
/// ```
#[macro_export]
macro_rules! assert_is_negative_zero {
    ($left:expr) => {{
        let val = $left;

        assert!(
            val == 0.0 && val.is_sign_negative(),
            "assertion failed: `(value is negative zero)` \
             (value: `{:?}`)",
            val,
        );
    }};
}
