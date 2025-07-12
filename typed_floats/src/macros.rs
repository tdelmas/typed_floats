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

/// Macro to create a constant value.
/// Will panic at compile time if the value is not a valid.
///
/// This macro is only useful is the rust version older than 1.83,
/// as floats methods are not yet const.
///
/// # Examples
///
/// ```
/// # use typed_floats::*;
/// const ONE: StrictlyPositiveFinite = as_const!(StrictlyPositiveFinite, 1.0); // f64
/// const TWO: StrictlyPositiveFinite<f64> = as_const!(StrictlyPositiveFinite, f64, 2.0);
/// const THREE: StrictlyPositiveFinite<f32> = as_const!(StrictlyPositiveFinite, f32, 3.0);
/// const FOUR: NonNaN = as_const!(NonNaN, 4.0); // f64
/// const FIVE: NonNaN<f64> = as_const!(NonNaN, f64, 5.0);
/// const SIX: NonNaN<f32> = as_const!(NonNaN, f32, 6.0);
/// ```
///
/// Thoses examples will panic at compile time:
///
/// ```ignore
/// # use typed_floats::*;
/// // Will panic at compile time
/// const X: StrictlyPositiveFinite = as_const!(StrictlyPositiveFinite, 0.0);
/// ```
///
/// ```ignore
/// # use typed_floats::*;
/// // Will panic at compile time
/// const X: StrictlyPositiveFinite = as_const!(StrictlyPositiveFinite, 1.0/0.0);
/// ```
///
/// ```ignore
/// # use typed_floats::*;
/// // Will panic at compile time
/// const X: StrictlyPositiveFinite = as_const!(StrictlyPositiveFinite, -1.0/0.0);
/// ```
///
/// ```ignore
/// # use typed_floats::*;
/// // Will panic at compile time
/// const X: StrictlyPositiveFinite = as_const!(StrictlyPositiveFinite, 0.0/0.0);
/// ```
///
#[macro_export]
macro_rules! as_const {
    ($type:ident, $float:ident, $x:expr) => {{
        const TMP: $float = $x;

        #[allow(clippy::float_cmp_const)]
        const RESULT: $crate::$type<$float> = if TMP != TMP {
            panic!("NaN is not valid")
        } else if TMP == $float::INFINITY && !$crate::$type::accept_infinity() {
            panic!("Infinity is not valid")
        } else if TMP == $float::NEG_INFINITY && !$crate::$type::accept_infinity() {
            panic!("Infinity is not valid")
        } else if TMP == 0.0 && !$crate::$type::accept_zero() {
            panic!("Zero is not valid")
        } else if TMP < 0.0 && !$crate::$type::accept_negative() {
            panic!("Negative value is not valid")
        } else if TMP > 0.0 && !$crate::$type::accept_positive() {
            panic!("Negative zero is not valid")
        } else {
            // Safety: The value has been checked
            unsafe { $crate::$type::<$float>::internal_only_new_unchecked(TMP) }
        };
        RESULT
    }};
    ($type:ident, $x:expr) => {{
        $crate::as_const!($type, f64, $x)
    }};
    ($x:expr) => {{
        $crate::as_const!(f64, $x)
    }};
}

/// Macro to create a constant.
/// It will generate the full declaration of the constant:
/// ```ignore
/// pub const NAME: TYPE = TYPE(VALUE)
/// ```
///
/// Will panic at compile time if the value is not a valid.
///
/// # Examples
///
/// ```
/// # use typed_floats::*;
/// generate_const!(ZERO, NonNaN, f32, 0.0);
/// generate_const!(ONE, StrictlyPositiveFinite, 1.0); // f64
/// generate_const!(TWO, StrictlyPositiveFinite, f64, 2.0);
/// generate_const!(THREE, StrictlyPositiveFinite, f32, 3.0);
/// generate_const!(PI, StrictlyPositiveFinite, f32, 3.14, "The number π");
/// generate_const!(FOUR, NonNaN, 4.0); // f64
/// generate_const!(FIVE, NonNaN, f64, 5.0);
/// generate_const!(INF, StrictlyPositive, f32, 1.0/0.0);
/// ```
///
/// Thoses examples will panic at compile time:
///
/// ```ignore
/// # use typed_floats::*;
/// // Will panic at compile time
/// generate_const!(X, StrictlyPositiveFinite, 0.0);
/// ```
///
/// ```ignore
/// # use typed_floats::*;
/// // Will panic at compile time
/// generate_const!(X, StrictlyPositiveFinite, 1.0/0.0);
/// ```
///
/// ```ignore
/// # use typed_floats::*;
/// // Will panic at compile time
/// generate_const!(X, StrictlyPositiveFinite, -1.0/0.0);
/// ```
///
/// ```ignore
/// # use typed_floats::*;
/// // Will panic at compile time
/// generate_const!(X, StrictlyPositiveFinite, 0.0/0.0);
/// ```
///
#[macro_export]
macro_rules! generate_const {
    ($name:ident, $type:ident, $float:ident, $x:expr, $doc:expr) => {
        #[doc = $doc]
        pub const $name: $crate::$type<$float> = $crate::as_const!($type, $float, $x);
    };
    ($name:ident, $type:ident, f32, $x:expr) => {
        pub const $name: $crate::$type<f32> = $crate::as_const!($type, f32, $x);
    };
    ($name:ident, $type:ident, f64, $x:expr) => {
        pub const $name: $crate::$type<f64> = $crate::as_const!($type, f64, $x);
    };
    ($name:ident, $type:ident, $x:expr, $doc:expr) => {
        #[doc = $doc]
        pub const $name: $crate::$type = $crate::as_const!($type, f64, $x);
    };
    ($name:ident, $type:ident, $x:expr) => {
        pub const $name: $crate::$type = $crate::as_const!($type, f64, $x);
    };
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

        Self($value)
    }};
}

pub(crate) use new_unchecked;
