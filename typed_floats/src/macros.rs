#![allow(clippy::float_cmp)]

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
/// assert_is_nan!(f64::INFINITY);
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
/// assert_is_positive_zero!(f64::INFINITY);
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
/// assert_is_negative_zero!(f64::NEG_INFINITY);
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

/// This macros assert that the two value are equal:
/// - If they are both NaN, they are considered equal;
/// - If they are zero, they are considered equal if they have the same sign;
/// - All other cases are tested with `assert_eq!`.
///
/// # Examples
///
/// ```
/// # use typed_floats::*;
/// assert_float_eq!(f64::NAN, f64::NAN);
/// assert_float_eq!(1.0_f64, 1.0_f64);
/// assert_float_eq!(-1.0_f64, -1.0_f64);
/// assert_float_eq!(0.0_f64, 0.0_f64);
/// assert_float_eq!(-0.0_f64, -0.0_f64);
/// assert_float_eq!(f64::INFINITY, f64::INFINITY);
/// assert_float_eq!(f64::NEG_INFINITY, f64::NEG_INFINITY);
/// ```
///
/// ```should_panic
/// # use typed_floats::*;
/// assert_float_eq!(1.0_f64, 2.0_f64);
/// ```
///
/// ```should_panic
/// # use typed_floats::*;
/// assert_float_eq!(1.0_f64, f64::NAN);
/// ```
///
/// ```should_panic
/// # use typed_floats::*;
/// assert_float_eq!(0.0_f64, -0.0_f64);
/// ```
#[macro_export]
macro_rules! assert_float_eq {
    ($left:expr, $right:expr) => {{
        if (!$left.is_nan() || !$right.is_nan()) {
            assert_eq!($left, $right);
            assert_eq!($left.is_sign_positive(), $right.is_sign_positive());
        }
    }};
}

macro_rules! new_unchecked {
    ($value:ident, $name:ident) => {{
        if cfg!(any(
            debug_assertions,
            feature = "ensure_no_undefined_behavior"
        )) {
            if Self::new($value).is_err() {
                panic!(concat!("This value is not a valid ", stringify!($name)));
            }
        } else if cfg!(feature = "compiler_hints") {
            if Self::new($value).is_err() {
                unsafe { core::hint::unreachable_unchecked() }
            }
        }

        Self(crate::container::Container::new($value))
    }};
}

macro_rules! generate_const {
    ($name:ident, $type:ident, $float:ident, $x:expr, $doc:expr) => {
        #[doc = $doc]
        pub const $name: $crate::$type<$float> = match $crate::$type::<$float>::new($x) {
            Ok(v) => v,
            Err(_) => panic!("Invalid value"),
        };
    };
}

pub(crate) use generate_const;
pub(crate) use new_unchecked;
