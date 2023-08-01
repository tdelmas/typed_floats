use quote::quote;

use crate::types::*;

pub(crate) fn get_impl_self() -> Vec<Op> {
    vec![
        OpBuilder::new("neg")
            .trait_name("core::ops::Neg")
            .display("-")
            .op_fn(Box::new(|_| {
                quote! { -self.get() }
            }))
            .op_test(Box::new(|var| {
                quote! { -#var }
            }))
            .result(Box::new(|float| {
                let mut output_spec = float.s.clone();

                if !output_spec.accept_positive {
                    output_spec.accept_positive = true;
                    output_spec.accept_negative = false;
                } else if !output_spec.accept_negative {
                    output_spec.accept_positive = false;
                    output_spec.accept_negative = true;
                }

                Some(output_spec)
            }))
            .build(),
        OpBuilder::new("abs")
            .description(quote! {
                /// Computes the absolute value of `self`.
                ///
                /// See [`f64::abs()`] for more details.
            })
            .op_fn(Box::new(|float| {
                if !float.s.accept_negative {
                    // no-op
                    quote! { self.get() }
                } else if !float.s.accept_positive {
                    // inv
                    quote! { -self.get() }
                } else {
                    quote! { self.get().abs() }
                }
            }))
            .result(Box::new(|float| {
                let mut output_spec = float.s.clone();

                output_spec.accept_positive = true;
                output_spec.accept_negative = false;

                Some(output_spec)
            }))
            .build(),
        OpBuilder::new("ceil")
            .description(quote! {
                /// Returns the smallest integer greater than or equal to `self`.
                ///
                /// See [`f64::ceil()`] for more details.
            })
            .result(Box::new(|float| {
                let mut output_spec = float.s.clone();

                if float.s.accept_negative {
                    output_spec.accept_zero = true;
                }

                Some(output_spec)
            }))
            .build(),
        OpBuilder::new("floor")
            .description(quote! {
                /// Returns the largest integer less than or equal to `self`.
                ///
                /// See [`f64::floor()`] for more details.
            })
            .result(Box::new(|float| {
                let mut output_spec = float.s.clone();

                if float.s.accept_positive {
                    output_spec.accept_zero = true;
                }

                Some(output_spec)
            }))
            .build(),
        OpBuilder::new("round")
            .description(quote! {
                /// Returns the nearest integer to `self`. If a value is half-way between two
                /// integers, round away from `0.0`.
                ///
                /// See [`f64::round()`] for more details.
            })
            .result(Box::new(|float| {
                let mut output_spec = float.s.clone();

                output_spec.accept_zero = true;

                Some(output_spec)
            }))
            .build(),
        OpBuilder::new("trunc")
            .description(quote! {
                /// Returns the integer part of `self`.
                /// This means that non-integer numbers are always truncated towards zero.
                ///
                /// See [`f64::trunc()`] for more details.
            })
            .result(Box::new(|float| {
                let mut output_spec = float.s.clone();

                output_spec.accept_zero = true;

                Some(output_spec)
            }))
            .build(),
        OpBuilder::new("fract")
            .description(quote! {
                /// Returns the fractional part of `self`.
                /// For negative numbers, the result is negative except when the fractional part is zero.
                ///
                /// See [`f64::fract()`] for more details.
            })
            .comment(
                "`fract` returns `+0.0` if the factional part is zero, even for negative numbers.",
            )
            .result(Box::new(|float| {
                if float.s.accept_inf {
                    return None;
                }

                let mut output_spec = float.s.clone();
                output_spec.accept_zero = true;
                // Returns POSITIVE zero if the factional part is zero
                output_spec.accept_positive = true;

                Some(output_spec)
            }))
            .build(),
        OpBuilder::new("signum")
            .description(quote! {
                /// Returns a number that represents the sign of `self`.
                ///
                /// - `1.0` if the number is positive, `+0.0` or `INFINITY`
                /// - `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`
            })
            .op_fn(Box::new(|float| {
                if !float.s.accept_negative {
                    quote! { 1.0 }
                } else if !float.s.accept_positive {
                    quote! { -1.0 }
                } else {
                    quote! { self.get().signum() }
                }
            }))
            .result(Box::new(|float| {
                let spec = if !float.s.accept_negative {
                    FloatSpecifications {
                        accept_negative: false,
                        accept_positive: true,
                        accept_zero: false,
                        accept_inf: false,
                    }
                } else if !float.s.accept_positive {
                    FloatSpecifications {
                        accept_negative: true,
                        accept_positive: false,
                        accept_zero: false,
                        accept_inf: false,
                    }
                } else {
                    FloatSpecifications {
                        accept_negative: true,
                        accept_positive: true,
                        accept_zero: false,
                        accept_inf: false,
                    }
                };

                Some(spec)
            }))
            .build(),
        OpBuilder::new("sqrt")
            .description(quote! {
                /// Returns the square root of a number.
                ///
                /// Returns NaN if `self` is a negative number other than `-0.0`.
                /// (It returns `-0.0` if `self` is `-0.0`.)
                ///
                /// See [`f64::sqrt()`] for more details.
            })
            .comment("`sqrt(-0.0) = -0.0`")
            .op_fn(Box::new(|float| {
                // sqrt(-0.0) = -0.0
                if !float.s.accept_positive && !float.s.accept_zero {
                    let float_type = float.float_type_ident();

                    quote! { core::#float_type::NAN }
                } else {
                    quote! { self.get().sqrt() }
                }
            }))
            .result(Box::new(|float| {
                if float.s.accept_negative {
                    return None;
                }

                Some(float.s.clone())
            }))
            .build(),
        OpBuilder::new("exp")
            .description(quote! {
                /// Returns `e^(self)`, (the exponential function).
                ///
                /// See [`f64::exp()`] for more details.
            })
            .result(Box::new(|float| {
                Some(FloatSpecifications {
                    accept_negative: false,
                    accept_positive: true,
                    accept_zero: float.s.accept_negative,
                    accept_inf: float.s.accept_positive,
                })
            }))
            .build(),
        OpBuilder::new("exp2")
            .description(quote! {
                /// Returns `2^(self)`.
                ///
                /// See [`f64::exp2()`] for more details.
            })
            .result(Box::new(|float| {
                Some(FloatSpecifications {
                    accept_negative: false,
                    accept_positive: true,
                    accept_zero: float.s.accept_negative,
                    accept_inf: float.s.accept_positive,
                })
            }))
            .build(),
        OpBuilder::new("ln")
            .description(quote! {
                /// Returns the natural logarithm of the number.
                ///
                /// See [`f64::ln()`] for more details.
            })
            .op_fn(Box::new(|float| {
                let is_strictly_negative = !float.s.accept_positive && !float.s.accept_zero;

                if is_strictly_negative {
                    let float_type = float.float_type_ident();

                    quote! { core::#float_type::NAN }
                } else {
                    quote! { self.get().ln() }
                }
            }))
            .result(Box::new(|float| {
                if float.s.accept_negative {
                    return None;
                }

                Some(FloatSpecifications {
                    accept_negative: true,
                    accept_positive: true,
                    accept_zero: true,
                    accept_inf: float.s.accept_inf || float.s.accept_zero,
                })
            }))
            .build(),
        OpBuilder::new("log2")
            .description(quote! {
                /// Returns the base 2 logarithm of the number.
                ///
                /// See [`f64::log2()`] for more details.
            })
            .result(Box::new(|float| {
                if float.s.accept_negative {
                    return None;
                }

                Some(FloatSpecifications {
                    accept_negative: true,
                    accept_positive: true,
                    accept_zero: true,
                    accept_inf: float.s.accept_inf || float.s.accept_zero,
                })
            }))
            .build(),
        OpBuilder::new("log10")
            .description(quote! {
                /// Returns the base 10 logarithm of the number.
                ///
                /// See [`f64::log10()`] for more details.
            })
            .result(Box::new(|float| {
                if float.s.accept_negative {
                    return None;
                }

                Some(FloatSpecifications {
                    accept_negative: true,
                    accept_positive: true,
                    accept_zero: true,
                    accept_inf: float.s.accept_inf || float.s.accept_zero,
                })
            }))
            .build(),
        OpBuilder::new("to_degrees")
            .description(quote! {
                /// Converts degrees to radians.
                ///
                /// See [`f64::to_degrees()`] for more details.
            })
            .result(Box::new(|float| {
                let mut output_spec = float.s.clone();

                output_spec.accept_inf = true;

                Some(output_spec)
            }))
            .build(),
        OpBuilder::new("to_radians")
            .description(quote! {
                /// Converts degrees to radians.
                ///
                /// See [`f64::to_radians()`] for more details.
            })
            .result(Box::new(|float| Some(float.s.clone())))
            .build(),
    ]
}
