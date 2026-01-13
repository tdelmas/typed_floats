//! This module contains constants from [`core::f64`], casted to the corresponding type

use const_fn::const_fn;

/// Equivalent to `NonNaN<f64>`
pub type NonNaN = crate::NonNaN<f64>;

/// Equivalent to `NonNaNFinite<f64>`
pub type NonNaNFinite = crate::NonNaNFinite<f64>;

/// Equivalent to `NonZeroNonNaN<f64>`
pub type NonZeroNonNaN = crate::NonZeroNonNaN<f64>;

/// Equivalent to `NonZeroNonNaNFinite<f64>`
pub type NonZeroNonNaNFinite = crate::NonZeroNonNaNFinite<f64>;

/// Equivalent to `StrictlyPositive<f64>`
pub type StrictlyPositive = crate::StrictlyPositive<f64>;

/// Equivalent to `StrictlyNegative<f64>`
pub type StrictlyNegative = crate::StrictlyNegative<f64>;

/// Equivalent to `Positive<f64>`
pub type Positive = crate::Positive<f64>;

/// Equivalent to `Negative<f64>`
pub type Negative = crate::Negative<f64>;

/// Equivalent to `StrictlyPositiveFinite<f64>`
pub type StrictlyPositiveFinite = crate::StrictlyPositiveFinite<f64>;

/// Equivalent to `StrictlyNegativeFinite<f64>`
pub type StrictlyNegativeFinite = crate::StrictlyNegativeFinite<f64>;

/// Equivalent to `PositiveFinite<f64>`
pub type PositiveFinite = crate::PositiveFinite<f64>;

/// Equivalent to `NegativeFinite<f64>`
pub type NegativeFinite = crate::NegativeFinite<f64>;

/// Returns `true` if the number is positive zero.
///     
/// # Examples
///
/// ```
/// # use typed_floats::*;
///
/// assert_eq!(tf64::is_positive_zero(3.0), false);
/// assert_eq!(tf64::is_positive_zero(-0.0), false);
/// assert_eq!(tf64::is_positive_zero(0.0), true);
/// ```
#[inline]
#[must_use]
#[const_fn("1.83")]
pub const fn is_positive_zero(x: f64) -> bool {
    x == 0.0 && x.is_sign_positive()
}

/// Returns `true` if the number is negative zero.
///    
/// # Examples
///
/// ```
/// # use typed_floats::*;
///
/// assert_eq!(tf64::is_negative_zero(3.0), false);
/// assert_eq!(tf64::is_negative_zero(-0.0), true);
/// assert_eq!(tf64::is_negative_zero(0.0), false);
/// ```
#[inline]
#[must_use]
#[const_fn("1.83")]
pub const fn is_negative_zero(x: f64) -> bool {
    x == 0.0 && x.is_sign_negative()
}

const fn from_bits(bits: u64) -> f64 {
    // SAFETY: it is a plain old datatype so we can always transmute from it.
    // `f64::from_bits` is not const for `1.70` MSRV
    unsafe {
        #[allow(unnecessary_transmutes)]
        core::mem::transmute::<u64, f64>(bits)
    }
}

crate::generate_const!(
    INFINITY,
    StrictlyPositive,
    f64,
    f64::INFINITY,
    "Infinity (∞)."
);

crate::generate_const!(
    NEG_INFINITY,
    StrictlyNegative,
    f64,
    f64::NEG_INFINITY,
    "Negative infinity (−∞)."
);

crate::generate_const!(ZERO, PositiveFinite, f64, 0.0f64, "Positive zero (+0.0).");

crate::generate_const!(
    NEG_ZERO,
    NegativeFinite,
    f64,
    -0.0f64,
    "Negative zero (-0.0)."
);

crate::generate_const!(
    MAX,
    StrictlyPositiveFinite,
    f64,
    f64::MAX,
    "Largest finite `f64` value."
);

crate::generate_const!(
    MIN,
    StrictlyNegativeFinite,
    f64,
    f64::MIN,
    "Smallest finite `f64` value."
);

crate::generate_const!(
    MIN_POSITIVE,
    StrictlyPositiveFinite,
    f64,
    f64::MIN_POSITIVE,
    "Smallest positive normal `f64` value."
);

crate::generate_const!(
    MIN_SUBNORMAL_POSITIVE,
    StrictlyPositiveFinite,
    f64,
    from_bits(0x0000_0000_0000_0001),
    "Smallest subnormal positive `f64` value."
);

crate::generate_const!(
    MAX_SUBNORMAL_POSITIVE,
    StrictlyPositiveFinite,
    f64,
    from_bits(0x000F_FFFF_FFFF_FFFF),
    "Largest subnormal positive `f64` value."
);

crate::generate_const!(
    MIN_SUBNORMAL_NEGATIVE,
    StrictlyNegativeFinite,
    f64,
    from_bits(0x8000_0000_0000_0001),
    "Smallest subnormal negative `f64` value."
);

crate::generate_const!(
    MAX_SUBNORMAL_NEGATIVE,
    StrictlyNegativeFinite,
    f64,
    from_bits(0x800F_FFFF_FFFF_FFFF),
    "Largest subnormal negative `f64` value."
);

/// This module contains constants from [`core::f64::consts`], casted to the corresponding type
pub mod consts {
    crate::generate_const!(
        PI,
        StrictlyPositiveFinite,
        f64,
        core::f64::consts::PI,
        "Archimedes' constant (π)"
    );
    crate::generate_const!(
        TAU,
        StrictlyPositiveFinite,
        f64,
        core::f64::consts::TAU,
        "The full circle constant (τ). Equal to 2π."
    );
    crate::generate_const!(
        FRAC_PI_2,
        StrictlyPositiveFinite,
        f64,
        core::f64::consts::FRAC_PI_2,
        "π/2"
    );
    crate::generate_const!(
        FRAC_PI_3,
        StrictlyPositiveFinite,
        f64,
        core::f64::consts::FRAC_PI_3,
        "π/3"
    );
    crate::generate_const!(
        FRAC_PI_4,
        StrictlyPositiveFinite,
        f64,
        core::f64::consts::FRAC_PI_4,
        "π/4"
    );
    crate::generate_const!(
        FRAC_PI_6,
        StrictlyPositiveFinite,
        f64,
        core::f64::consts::FRAC_PI_6,
        "π/6"
    );
    crate::generate_const!(
        FRAC_PI_8,
        StrictlyPositiveFinite,
        f64,
        core::f64::consts::FRAC_PI_8,
        "π/8"
    );
    crate::generate_const!(
        FRAC_1_PI,
        StrictlyPositiveFinite,
        f64,
        core::f64::consts::FRAC_1_PI,
        "1/π"
    );
    crate::generate_const!(
        FRAC_2_PI,
        StrictlyPositiveFinite,
        f64,
        core::f64::consts::FRAC_2_PI,
        "2/π"
    );
    crate::generate_const!(
        FRAC_2_SQRT_PI,
        StrictlyPositiveFinite,
        f64,
        core::f64::consts::FRAC_2_SQRT_PI,
        "2/sqrt(π)"
    );
    crate::generate_const!(
        SQRT_2,
        StrictlyPositiveFinite,
        f64,
        core::f64::consts::SQRT_2,
        "sqrt(2)"
    );
    crate::generate_const!(
        FRAC_1_SQRT_2,
        StrictlyPositiveFinite,
        f64,
        core::f64::consts::FRAC_1_SQRT_2,
        "1/sqrt(2)"
    );
    crate::generate_const!(
        E,
        StrictlyPositiveFinite,
        f64,
        core::f64::consts::E,
        "Euler's number (e)"
    );
    crate::generate_const!(
        LOG2_10,
        StrictlyPositiveFinite,
        f64,
        core::f64::consts::LOG2_10,
        "log<sub>2</sub>(10)"
    );
    crate::generate_const!(
        LOG2_E,
        StrictlyPositiveFinite,
        f64,
        core::f64::consts::LOG2_E,
        "log<sub>2</sub>(e)"
    );
    crate::generate_const!(
        LOG10_2,
        StrictlyPositiveFinite,
        f64,
        core::f64::consts::LOG10_2,
        "log<sub>10</sub>(2)"
    );
    crate::generate_const!(
        LOG10_E,
        StrictlyPositiveFinite,
        f64,
        core::f64::consts::LOG10_E,
        "log<sub>10</sub>(e)"
    );
    crate::generate_const!(
        LN_2,
        StrictlyPositiveFinite,
        f64,
        core::f64::consts::LN_2,
        "ln(2)"
    );
    crate::generate_const!(
        LN_10,
        StrictlyPositiveFinite,
        f64,
        core::f64::consts::LN_10,
        "ln(10)"
    );
}

/// Return an array of interesting test values
#[doc(hidden)]
#[must_use]
pub fn get_test_values() -> [f64; 25] {
    [
        f64::NAN,
        f64::NEG_INFINITY,
        f64::MIN,
        -core::f64::consts::PI,
        -core::f64::consts::E,
        -2.0,
        -core::f64::consts::FRAC_PI_2,
        -1.0,
        -f64::MIN_POSITIVE,
        crate::tf64::MAX_SUBNORMAL_NEGATIVE.get(),
        -1.0e-308,
        crate::tf64::MIN_SUBNORMAL_NEGATIVE.get(),
        -0.0,
        0.0,
        crate::tf64::MIN_SUBNORMAL_POSITIVE.get(),
        1.0e-308,
        crate::tf64::MAX_SUBNORMAL_POSITIVE.get(),
        f64::MIN_POSITIVE,
        1.0,
        core::f64::consts::FRAC_PI_2,
        2.0,
        core::f64::consts::E,
        core::f64::consts::PI,
        f64::MAX,
        f64::INFINITY,
    ]
}
