use crate::{
    Negative, NegativeFinite, NonNaN, NonNaNFinite, NonZeroNonNaN, NonZeroNonNaNFinite, Positive,
    PositiveFinite, ExcludeInf, ExcludeNegative, ExcludePositive, ExcludeZero,
    StrictlyNegative, StrictlyNegativeFinite, StrictlyPositive, StrictlyPositiveFinite,
};

macro_rules! impl_exclude {
    ($trait:ident, $fn:ident, $($check:ident)? $($zero:literal)?, $type:ident, $output:ident) => {
        impl $trait for $type<f32> {
            type Output = $output<f32>;

            #[inline]
            fn $fn(self) -> Result<Self::Output, Self> {
                if
                $(self.$check())?
                $($crate::$type::accept_zero() && self.0 == $zero)?
                {
                    Err(self)
                } else {
                    Ok(unsafe { $output::<f32>::new_unchecked(self.get()) })
                }
            }
        }

        impl $trait for $type<f64> {
            type Output = $output<f64>;

            #[inline]
            fn $fn(self) -> Result<Self::Output, Self> {
                if
                $(self.$check())?
                $($crate::$type::accept_zero() && self.0 == $zero)?
                {
                    Err(self)
                } else {
                    Ok(unsafe { $output::<f64>::new_unchecked(self.get()) })
                }
            }
        }
    };
}

macro_rules! impl_exclude_inf {
    ($type:ident, $output:ident) => {
        impl_exclude!(ExcludeInf, exclude_inf, is_infinite, $type, $output);
    };
}

macro_rules! impl_exclude_zero {
    ($type:ident, $output:ident) => {
        impl_exclude!(ExcludeZero, exclude_zero, 0.0, $type, $output);
    };
}

macro_rules! impl_exclude_positive {
    ($type:ident, $output:ident) => {
        impl_exclude!(
            ExcludePositive,
            exclude_positive,
            is_sign_positive,
            $type,
            $output
        );
    };
}

macro_rules! impl_exclude_negative {
    ($type:ident, $output:ident) => {
        impl_exclude!(
            ExcludeNegative,
            exclude_negative,
            is_sign_negative,
            $type,
            $output
        );
    };
}

impl_exclude_inf!(NonNaN, NonNaNFinite);
impl_exclude_inf!(NonZeroNonNaN, NonZeroNonNaNFinite);
impl_exclude_inf!(StrictlyPositive, StrictlyPositiveFinite);
impl_exclude_inf!(StrictlyNegative, StrictlyNegativeFinite);
impl_exclude_inf!(Positive, PositiveFinite);
impl_exclude_inf!(Negative, NegativeFinite);

impl_exclude_zero!(NonNaN, NonZeroNonNaN);
impl_exclude_zero!(NonNaNFinite, NonZeroNonNaNFinite);
impl_exclude_zero!(PositiveFinite, StrictlyPositiveFinite);
impl_exclude_zero!(NegativeFinite, StrictlyNegativeFinite);
impl_exclude_zero!(Positive, StrictlyPositive);
impl_exclude_zero!(Negative, StrictlyNegative);

impl_exclude_positive!(NonNaN, Negative);
impl_exclude_positive!(NonNaNFinite, NegativeFinite);
impl_exclude_positive!(NonZeroNonNaN, StrictlyNegative);
impl_exclude_positive!(NonZeroNonNaNFinite, StrictlyNegativeFinite);

impl_exclude_negative!(NonNaN, Positive);
impl_exclude_negative!(NonNaNFinite, PositiveFinite);
impl_exclude_negative!(NonZeroNonNaN, StrictlyPositive);
impl_exclude_negative!(NonZeroNonNaNFinite, StrictlyPositiveFinite);
