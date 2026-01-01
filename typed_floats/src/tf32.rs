//! This module contains constants from [`core::f32`], casted to the corresponding type

use const_fn::const_fn;

/// Equivalent to `NonNaN<f32>`
pub type NonNaN = crate::NonNaN<f32>;

/// Equivalent to `NonNaNFinite<f32>`
pub type NonNaNFinite = crate::NonNaNFinite<f32>;

/// Equivalent to `NonZeroNonNaN<f32>`
pub type NonZeroNonNaN = crate::NonZeroNonNaN<f32>;

/// Equivalent to `NonZeroNonNaNFinite<f32>`
pub type NonZeroNonNaNFinite = crate::NonZeroNonNaNFinite<f32>;

/// Equivalent to `StrictlyPositive<f32>`
pub type StrictlyPositive = crate::StrictlyPositive<f32>;

/// Equivalent to `StrictlyNegative<f32>`
pub type StrictlyNegative = crate::StrictlyNegative<f32>;

/// Equivalent to `Positive<f32>`
pub type Positive = crate::Positive<f32>;

/// Equivalent to `Negative<f32>`
pub type Negative = crate::Negative<f32>;

/// Equivalent to `StrictlyPositiveFinite<f32>`
pub type StrictlyPositiveFinite = crate::StrictlyPositiveFinite<f32>;

/// Equivalent to `StrictlyNegativeFinite<f32>`
pub type StrictlyNegativeFinite = crate::StrictlyNegativeFinite<f32>;

/// Equivalent to `PositiveFinite<f32>`
pub type PositiveFinite = crate::PositiveFinite<f32>;

/// Equivalent to `NegativeFinite<f32>`
pub type NegativeFinite = crate::NegativeFinite<f32>;

/// Returns `true` if the number is positive zero.
///     
/// # Examples
///
/// ```
/// # use typed_floats::*;
///
/// assert_eq!(tf32::is_positive_zero(3.0), false);
/// assert_eq!(tf32::is_positive_zero(-0.0), false);
/// assert_eq!(tf32::is_positive_zero(0.0), true);
/// ```
#[inline]
#[must_use]
#[const_fn("1.83")]
pub const fn is_positive_zero(x: f32) -> bool {
    x == 0.0 && x.is_sign_positive()
}

const fn from_bits(bits: u32) -> f32 {
    // SAFETY: it is a plain old datatype so we can always transmute from it.
    // `f32::from_bits` is not const for `1.70` MSRV
    unsafe {
        #[allow(unnecessary_transmutes)]
        core::mem::transmute::<u32, f32>(bits)
    }
}

/// Returns `true` if the number is negative zero.
///    
/// # Examples
///
/// ```
/// # use typed_floats::*;
///
/// assert_eq!(tf32::is_negative_zero(3.0), false);
/// assert_eq!(tf32::is_negative_zero(-0.0), true);
/// assert_eq!(tf32::is_negative_zero(0.0), false);
/// ```
#[inline]
#[must_use]
#[const_fn("1.83")]
pub const fn is_negative_zero(x: f32) -> bool {
    x == 0.0 && x.is_sign_negative()
}

crate::generate_const!(
    INFINITY,
    StrictlyPositive,
    f32,
    f32::INFINITY,
    "Infinity (∞)."
);

crate::generate_const!(
    NEG_INFINITY,
    StrictlyNegative,
    f32,
    f32::NEG_INFINITY,
    "Negative infinity (−∞)."
);

crate::generate_const!(ZERO, PositiveFinite, f32, 0.0f32, "Positive zero (+0.0).");

crate::generate_const!(
    NEG_ZERO,
    NegativeFinite,
    f32,
    -0.0f32,
    "Negative zero (-0.0)."
);

crate::generate_const!(
    MAX,
    StrictlyPositiveFinite,
    f32,
    f32::MAX,
    "Largest finite `f32` value."
);

crate::generate_const!(
    MIN,
    StrictlyNegativeFinite,
    f32,
    f32::MIN,
    "Smallest finite `f32` value."
);

crate::generate_const!(
    MIN_POSITIVE,
    StrictlyPositiveFinite,
    f32,
    f32::MIN_POSITIVE,
    "Smallest positive normal `f32` value."
);

crate::generate_const!(
    MIN_SUBNORMAL_POSITIVE,
    StrictlyPositiveFinite,
    f32,
    from_bits(0b0_00000000_00000000000000000000001),
    "Smallest subnormal positive `f32` value."
);

crate::generate_const!(
    MAX_SUBNORMAL_POSITIVE,
    StrictlyPositiveFinite,
    f32,
    from_bits(0b0_00000000_11111111111111111111111),
    "Largest subnormal positive `f32` value."
);

crate::generate_const!(
    MIN_SUBNORMAL_NEGATIVE,
    StrictlyNegativeFinite,
    f32,
    from_bits(0b1_00000000_00000000000000000000001),
    "Smallest subnormal negative `f32` value."
);

crate::generate_const!(
    MAX_SUBNORMAL_NEGATIVE,
    StrictlyNegativeFinite,
    f32,
    from_bits(0b1_00000000_11111111111111111111111),
    "Largest subnormal negative `f32` value."
);

/// This module contains constants from [`core::f32::consts`], casted to the corresponding type
pub mod consts {
    crate::generate_const!(
        PI,
        StrictlyPositiveFinite,
        f32,
        core::f32::consts::PI,
        "Archimedes' constant (π)"
    );
    crate::generate_const!(
        TAU,
        StrictlyPositiveFinite,
        f32,
        core::f32::consts::TAU,
        "The full circle constant (τ). Equal to 2π."
    );
    crate::generate_const!(
        FRAC_PI_2,
        StrictlyPositiveFinite,
        f32,
        core::f32::consts::FRAC_PI_2,
        "π/2"
    );
    crate::generate_const!(
        FRAC_PI_3,
        StrictlyPositiveFinite,
        f32,
        core::f32::consts::FRAC_PI_3,
        "π/3"
    );
    crate::generate_const!(
        FRAC_PI_4,
        StrictlyPositiveFinite,
        f32,
        core::f32::consts::FRAC_PI_4,
        "π/4"
    );
    crate::generate_const!(
        FRAC_PI_6,
        StrictlyPositiveFinite,
        f32,
        core::f32::consts::FRAC_PI_6,
        "π/6"
    );
    crate::generate_const!(
        FRAC_PI_8,
        StrictlyPositiveFinite,
        f32,
        core::f32::consts::FRAC_PI_8,
        "π/8"
    );
    crate::generate_const!(
        FRAC_1_PI,
        StrictlyPositiveFinite,
        f32,
        core::f32::consts::FRAC_1_PI,
        "1/π"
    );
    crate::generate_const!(
        FRAC_2_PI,
        StrictlyPositiveFinite,
        f32,
        core::f32::consts::FRAC_2_PI,
        "2/π"
    );
    crate::generate_const!(
        FRAC_2_SQRT_PI,
        StrictlyPositiveFinite,
        f32,
        core::f32::consts::FRAC_2_SQRT_PI,
        "2/sqrt(π)"
    );
    crate::generate_const!(
        SQRT_2,
        StrictlyPositiveFinite,
        f32,
        core::f32::consts::SQRT_2,
        "sqrt(2)"
    );
    crate::generate_const!(
        FRAC_1_SQRT_2,
        StrictlyPositiveFinite,
        f32,
        core::f32::consts::FRAC_1_SQRT_2,
        "1/sqrt(2)"
    );
    crate::generate_const!(
        E,
        StrictlyPositiveFinite,
        f32,
        core::f32::consts::E,
        "Euler's number (e)"
    );
    crate::generate_const!(
        LOG2_10,
        StrictlyPositiveFinite,
        f32,
        core::f32::consts::LOG2_10,
        "log<sub>2</sub>(10)"
    );
    crate::generate_const!(
        LOG2_E,
        StrictlyPositiveFinite,
        f32,
        core::f32::consts::LOG2_E,
        "log<sub>2</sub>(e)"
    );
    crate::generate_const!(
        LOG10_2,
        StrictlyPositiveFinite,
        f32,
        core::f32::consts::LOG10_2,
        "log<sub>10</sub>(2)"
    );
    crate::generate_const!(
        LOG10_E,
        StrictlyPositiveFinite,
        f32,
        core::f32::consts::LOG10_E,
        "log<sub>10</sub>(e)"
    );
    crate::generate_const!(
        LN_2,
        StrictlyPositiveFinite,
        f32,
        core::f32::consts::LN_2,
        "ln(2)"
    );
    crate::generate_const!(
        LN_10,
        StrictlyPositiveFinite,
        f32,
        core::f32::consts::LN_10,
        "ln(10)"
    );
}

/// Return an array of interesting test values
#[doc(hidden)]
#[must_use]
pub fn get_test_values() -> [f32; 25] {
    [
        f32::NAN,
        f32::NEG_INFINITY,
        f32::MIN,
        -core::f32::consts::PI,
        -core::f32::consts::E,
        -2.0,
        -core::f32::consts::FRAC_PI_2,
        -1.0,
        -f32::MIN_POSITIVE,
        crate::tf32::MAX_SUBNORMAL_NEGATIVE.get(),
        -1.0e-40,
        crate::tf32::MIN_SUBNORMAL_NEGATIVE.get(),
        -0.0,
        0.0,
        crate::tf32::MIN_SUBNORMAL_POSITIVE.get(),
        1.0e-40,
        crate::tf32::MAX_SUBNORMAL_POSITIVE.get(),
        f32::MIN_POSITIVE,
        1.0,
        core::f32::consts::FRAC_PI_2,
        2.0,
        core::f32::consts::E,
        core::f32::consts::PI,
        f32::MAX,
        f32::INFINITY,
    ]
}
