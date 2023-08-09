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
                /// # Examples
                ///
                /// ```
                /// # use typed_floats::*;
                /// let a: NonNaN = 3.0.try_into().unwrap();
                /// let b: NonNaN = (-4.0).try_into().unwrap();
                ///
                /// assert_eq!(a.abs(), 3.0);
                /// assert_eq!(b.abs(), 4.0);
                ///
                /// assert!(tf64::NEG_ZERO.abs().is_sign_positive());
                ///
                /// assert_eq!(tf64::NEG_INFINITY.abs(), tf64::INFINITY);
                /// ```
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
                /// # Examples
                ///
                /// ```
                /// # use typed_floats::*;
                /// let a: NonNaN = 3.5.try_into().unwrap();
                /// let b: NonNaN = (-3.5).try_into().unwrap();
                ///
                /// assert_eq!(a.ceil(), 4.0);
                /// assert_eq!(b.ceil(), -3.0);
                ///
                /// assert_eq!(tf64::INFINITY.ceil(), tf64::INFINITY);
                /// assert_eq!(tf64::NEG_INFINITY.ceil(), tf64::NEG_INFINITY);
                /// ```
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
                /// # Examples
                ///
                /// ```
                /// # use typed_floats::*;
                /// let a: NonNaN = 3.5.try_into().unwrap();
                /// let b: NonNaN = (-3.5).try_into().unwrap();
                ///
                /// assert_eq!(a.floor(), 3.0);
                /// assert_eq!(b.floor(), -4.0);
                ///
                /// assert_eq!(tf64::INFINITY.floor(), tf64::INFINITY);
                /// assert_eq!(tf64::NEG_INFINITY.floor(), tf64::NEG_INFINITY);
                /// ```
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
                /// # Examples
                ///
                /// ```
                /// # use typed_floats::*;
                /// let a: NonNaN = 3.5.try_into().unwrap();
                /// let b: NonNaN = (-3.5).try_into().unwrap();
                ///
                /// assert_eq!(a.round(), 4.0);
                /// assert_eq!(b.round(), -4.0);
                ///
                /// assert_eq!(tf64::INFINITY.round(), tf64::INFINITY);
                /// assert_eq!(tf64::NEG_INFINITY.round(), tf64::NEG_INFINITY);
                /// ```
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
                /// # Examples
                ///
                /// ```
                /// # use typed_floats::*;
                /// let a: NonNaN = 3.5.try_into().unwrap();
                /// let b: NonNaN = (-3.5).try_into().unwrap();
                ///
                /// assert_eq!(a.trunc(), 3.0);
                /// assert_eq!(b.trunc(), -3.0);
                ///
                /// assert_eq!(tf64::INFINITY.trunc(), tf64::INFINITY);
                /// assert_eq!(tf64::NEG_INFINITY.trunc(), tf64::NEG_INFINITY);
                /// ```
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
                /// For INIFINITY and NEG_INFINITY, the result is NaN.
                ///
                /// # Examples
                ///
                /// ```
                /// # use typed_floats::*;
                /// let a: NonNaNFinite = 3.5.try_into().unwrap();
                /// let b: NonNaNFinite = (-3.5).try_into().unwrap();
                /// let c: NonNaNFinite = (-3.0).try_into().unwrap();
                ///
                /// assert_eq!(a.fract(), 0.5);
                /// assert_eq!(b.fract(), -0.5);
                /// assert_eq!(c.fract().is_positive_zero(), true);
                ///
                /// assert_eq!(tf64::INFINITY.fract().is_nan(), true);
                /// ```
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
                ///
                /// # Examples
                ///
                /// ```
                /// # use typed_floats::*;
                /// let a: NonNaN = 3.5.try_into().unwrap();
                /// let b: NonNaN = (-3.5).try_into().unwrap();
                ///
                /// assert_eq!(a.signum(), 1.0);
                /// assert_eq!(b.signum(), -1.0);
                ///
                /// assert_eq!(tf64::ZERO.signum(), 1.0);
                /// assert_eq!(tf64::NEG_ZERO.signum(), -1.0);
                ///
                /// assert_eq!(tf64::INFINITY.signum(), 1.0);
                /// assert_eq!(tf64::NEG_INFINITY.signum(), -1.0);
                /// ```
                ///
                /// See [`f64::signum()`] for more details.
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
                /// Returns `-0.0` if `self` is `-0.0`.
                ///
                /// # Examples
                ///
                /// ```
                /// # use typed_floats::*;
                /// let a: NonNaN = 25.0.try_into().unwrap();
                /// let b: NonNaN = (-1).try_into().unwrap();
                ///
                /// assert_eq!(a.sqrt(), 5.0);
                /// assert!(b.sqrt().is_nan());
                ///
                /// assert!(tf64::is_positive_zero(tf64::ZERO.sqrt().into()));
                /// assert!(tf64::is_negative_zero(tf64::NEG_ZERO.sqrt()));
                ///
                /// assert_eq!(tf64::INFINITY.sqrt(), tf64::INFINITY);
                /// assert!(tf64::NEG_INFINITY.sqrt().is_nan());
                /// ```
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
                /// # Examples
                ///
                /// ```
                /// # use typed_floats::*;
                /// let a: NonNaN = 1.0.try_into().unwrap();
                /// let b: NonNaN = (-1.0).try_into().unwrap();
                ///
                /// assert_eq!(a.exp(), core::f64::consts::E);
                /// assert_eq!(b.exp(), 1.0 / core::f64::consts::E);
                ///
                /// assert_eq!(tf64::ZERO.exp(), 1.0);
                /// assert_eq!(tf64::NEG_ZERO.exp(), 1.0);
                ///
                /// assert_eq!(tf64::INFINITY.exp(), tf64::INFINITY);
                /// assert_eq!(tf64::NEG_INFINITY.exp(), tf64::ZERO);
                /// ```
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
                /// # Examples
                ///
                /// ```
                /// # use typed_floats::*;
                /// let a: NonNaN = 2.0.try_into().unwrap();
                /// let b: NonNaN = (-2.0).try_into().unwrap();
                ///
                /// assert_eq!(a.exp2(), 4.0);
                /// assert_eq!(b.exp2(), 0.25);
                ///
                /// assert_eq!(tf64::ZERO.exp2(), 1.0);
                /// assert_eq!(tf64::NEG_ZERO.exp2(), 1.0);
                ///
                /// assert_eq!(tf64::INFINITY.exp2(), tf64::INFINITY);
                /// assert_eq!(tf64::NEG_INFINITY.exp2(), tf64::ZERO);
                /// ```
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
                /// Returns NaN if `self` is a negative number other than `-0.0`.
                ///
                /// # Examples
                ///
                /// ```
                /// # use typed_floats::*;
                /// let a: NonNaN = 1.0.try_into().unwrap();
                /// let b: NonNaN = (-1.0).try_into().unwrap();
                ///
                /// assert_eq!(a.ln(), 0.0);
                /// assert!(b.ln().is_nan());
                ///
                /// assert_eq!(tf64::ZERO.ln(), f64::NEG_INFINITY);
                /// assert_eq!(tf64::NEG_ZERO.ln(), f64::NEG_INFINITY);
                ///
                /// assert_eq!(tf64::INFINITY.ln(), tf64::INFINITY);
                /// assert!(tf64::NEG_INFINITY.ln().is_nan());
                /// ```
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
                /// Returns NaN if `self` is a negative number other than `-0.0`.
                ///
                /// # Examples
                ///
                /// ```
                /// # use typed_floats::*;
                /// let a: NonNaN = 2.0.try_into().unwrap();
                /// let b: NonNaN = 1.0.try_into().unwrap();
                /// let c: NonNaN = (-1.0).try_into().unwrap();
                ///
                /// assert_eq!(a.log2(), 1.0);
                /// assert!(tf64::is_positive_zero(b.log2()));
                /// assert!(c.log2().is_nan());
                ///
                /// assert_eq!(tf64::ZERO.log2(), f64::NEG_INFINITY);
                /// assert_eq!(tf64::NEG_ZERO.log2(), f64::NEG_INFINITY);
                ///
                /// assert_eq!(tf64::INFINITY.log2(), tf64::INFINITY);
                /// assert!(tf64::NEG_INFINITY.log2().is_nan());
                /// ```
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
                /// Returns NaN if `self` is a negative number other than `-0.0`.
                ///
                /// # Examples
                ///
                /// ```
                /// # use typed_floats::*;
                /// let a: NonNaN = 100.0.try_into().unwrap();
                /// let b: NonNaN = 1.0.try_into().unwrap();
                /// let c: NonNaN = (-1.0).try_into().unwrap();
                ///
                /// assert_eq!(a.log10(), 2.0);
                /// assert_eq!(b.log10(), 0.0);
                /// assert!(c.log10().is_nan());
                ///
                /// assert_eq!(tf64::ZERO.log10(), f64::NEG_INFINITY);
                /// assert_eq!(tf64::NEG_ZERO.log10(), f64::NEG_INFINITY);
                ///
                /// assert_eq!(tf64::INFINITY.log10(), tf64::INFINITY);
                /// assert!(tf64::NEG_INFINITY.log10().is_nan());
                /// ```
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
                /// # Examples
                ///
                /// ```
                /// # use typed_floats::*;
                /// let a: NonNaN = core::f64::consts::PI.try_into().unwrap();
                /// let b: NonNaN = (-core::f64::consts::PI).try_into().unwrap();
                /// let c: NonNaN = (2.0 * core::f64::consts::PI).try_into().unwrap();
                ///
                /// assert_eq!(a.to_degrees(), 180.0);
                /// assert_eq!(b.to_degrees(), -180.0);
                /// assert_eq!(c.to_degrees(), 360.0);
                ///
                /// assert!(tf64::ZERO.to_degrees().is_positive_zero());
                /// assert!(tf64::NEG_ZERO.to_degrees().is_negative_zero());
                ///
                /// assert_eq!(tf64::INFINITY.to_degrees(), tf64::INFINITY);
                /// assert_eq!(tf64::NEG_INFINITY.to_degrees(), tf64::NEG_INFINITY);
                /// ```
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
                /// # Examples
                ///
                /// ```
                /// # use typed_floats::*;
                /// let a: NonNaN = 180.0.try_into().unwrap();
                /// let b: NonNaN = 0.0.try_into().unwrap();
                /// let c: NonNaN = (-180.0).try_into().unwrap();
                /// let d: NonNaN = 360.0.try_into().unwrap();
                ///
                /// assert_eq!(a.to_radians(), core::f64::consts::PI);
                /// assert_eq!(b.to_radians(), 0.0);
                /// assert_eq!(c.to_radians(), -core::f64::consts::PI);
                /// assert_eq!(d.to_radians(), 2.0 * core::f64::consts::PI);
                ///
                /// assert_eq!(tf64::INFINITY.to_radians(), tf64::INFINITY);
                /// assert_eq!(tf64::NEG_INFINITY.to_radians(), tf64::NEG_INFINITY);
                /// ```
                ///
                /// See [`f64::to_radians()`] for more details.
            })
            .result(Box::new(|float| Some(float.s.clone())))
            .build(),
        OpBuilder::new("cbrt")
            .description(quote! {
                /// Returns the cube root of a number.
                ///
                /// The result will be finite unless the argument is infinite.
                ///
                /// # Examples
                ///
                /// ```
                /// # use typed_floats::*;
                /// let a: NonNaN = 8.0.try_into().unwrap();
                /// let b: NonNaN = (-1.0).try_into().unwrap();
                ///
                /// assert_eq!(a.cbrt(), 2.0);
                /// assert_eq!(b.cbrt(), -1.0);
                ///
                /// assert_eq!(tf64::ZERO.cbrt(), tf64::ZERO);
                /// assert_eq!(tf64::NEG_ZERO.cbrt(), tf64::NEG_ZERO);
                ///
                /// assert_eq!(tf64::INFINITY.cbrt(), tf64::INFINITY);
                /// assert_eq!(tf64::NEG_INFINITY.cbrt(), tf64::NEG_INFINITY);
                /// ```
                ///
                /// See [`f64::cbrt()`] for more details.
            })
            .result(Box::new(|float| Some(float.s.clone())))
            .build(),
        OpBuilder::new("sin")
            // For Non-zero, `sin` still produces zeros but the tests can't check it
            .skip_check_return_type_strictness()
            .description(quote! {
                /// Computes the sine of a number (in radians).
                ///
                /// The result will be in the range [-1, 1] if the input is finite,
                /// and NaN if the input is infinite.
                ///
                /// # Examples
                ///
                /// ```
                /// # use typed_floats::*;
                /// let a: NonNaNFinite = core::f64::consts::PI.try_into().unwrap();
                /// let b: NonNaNFinite = 0.0.try_into().unwrap();
                /// let c: NonNaNFinite = (-0.0).try_into().unwrap();
                /// let d: NonNaNFinite = (-core::f64::consts::PI).try_into().unwrap();
                /// let e: NonNaNFinite = core::f64::consts::FRAC_PI_2.try_into().unwrap();
                /// let f: NonNaNFinite = (-core::f64::consts::FRAC_PI_2).try_into().unwrap();
                ///
                /// assert_relative_eq!(a.sin().get(), 0.0);
                /// assert_relative_eq!(b.sin().get(), 0.0);
                /// assert_relative_eq!(c.sin().get(), 0.0);
                /// assert_relative_eq!(d.sin().get(), 0.0);
                /// assert_relative_eq!(e.sin().get(), 1.0);
                /// assert_relative_eq!(f.sin().get(), -1.0);
                ///
                /// assert!(tf64::INFINITY.sin().is_nan());
                /// assert!(tf64::NEG_INFINITY.sin().is_nan());
                ///
                /// ```
                ///
                /// See [`f64::sin()`] for more details.
            })
            .result(Box::new(|float| {
                if float.s.accept_inf {
                    return None;
                }

                Some(FloatSpecifications {
                    accept_negative: true,
                    accept_positive: true,
                    accept_zero: true,
                    accept_inf: false,
                })
            }))
            .build(),
        OpBuilder::new("cos")
            // For Non-zero, `cos` still produces zeros but the tests can't check it
            .skip_check_return_type_strictness()
            .description(quote! {
                /// Computes the cosine of a number (in radians).
                ///
                /// The result will be in the range [-1, 1] if the input is finite,
                /// and NaN if the input is infinite.
                ///
                /// # Examples
                ///
                /// ```
                /// # use typed_floats::*;
                /// let a: NonNaNFinite = core::f64::consts::PI.try_into().unwrap();
                /// let b: NonNaNFinite = 0.0.try_into().unwrap();
                /// let c: NonNaNFinite = (-0.0).try_into().unwrap();
                /// let d: NonNaNFinite = (-core::f64::consts::PI).try_into().unwrap();
                /// let e: NonNaNFinite = core::f64::consts::FRAC_PI_2.try_into().unwrap();
                /// let f: NonNaNFinite = (-core::f64::consts::FRAC_PI_2).try_into().unwrap();
                ///
                /// assert_relative_eq!(a.cos().get(), -1.0);
                /// assert_relative_eq!(b.cos().get(), 1.0);
                /// assert_relative_eq!(c.cos().get(), 1.0);
                /// assert_relative_eq!(d.cos().get(), -1.0);
                /// assert_relative_eq!(e.cos().get(), 0.0);
                /// assert_relative_eq!(f.cos().get(), 0.0);
                ///
                /// assert!(tf64::INFINITY.cos().is_nan());
                /// assert!(tf64::NEG_INFINITY.cos().is_nan());
                /// ```
                ///
                /// See [`f64::cos()`] for more details.
            })
            .result(Box::new(|float| {
                if float.s.accept_inf {
                    return None;
                }

                Some(FloatSpecifications {
                    accept_negative: true,
                    accept_positive: true,
                    accept_zero: true,
                    accept_inf: false,
                })
            }))
            .build(),
        OpBuilder::new("tan")
            // For Non-zero, `tan` still produces zeros but the tests can't check it
            .skip_check_return_type_strictness()
            .description(quote! {
                /// Computes the tangent of a number (in radians).
                ///
                /// The result NaN if the input is infinite.
                ///
                /// # Examples
                ///
                /// ```
                /// # use typed_floats::*;
                ///
                /// assert_eq!(tf64::ZERO.tan(), 0.0);
                /// assert_relative_eq!(tf64::consts::FRAC_PI_4.tan().get(), 1.0);
                ///
                /// assert!(tf64::INFINITY.tan().is_nan());
                /// assert!(tf64::NEG_INFINITY.tan().is_nan());
                /// ```
                ///
                /// See [`f64::tan()`] for more details.
            })
            .result(Box::new(|float| {
                if float.s.accept_inf {
                    return None;
                }

                Some(FloatSpecifications {
                    accept_negative: true,
                    accept_positive: true,
                    accept_zero: true,
                    accept_inf: true,
                })
            }))
            .build(),
        OpBuilder::new("asin")
            .description(quote! {
                /// Computes the arcsine of a number.
                ///
                /// Return value is in radians in the range [-pi/2, pi/2] or NaN if the number is outside the range [-1, 1].
                ///
                /// See [`f64::asin()`] for more details.
            })
            .result(Box::new(|_| None))
            .build(),
        OpBuilder::new("acos")
            .description(quote! {
                /// Computes the arccosine of a number.
                ///
                /// Return value is in radians in the range [0, pi] or NaN if the number is outside the range [-1, 1].
                ///
                /// See [`f64::acos()`] for more details.
            })
            .result(Box::new(|_| None))
            .build(),
        OpBuilder::new("atan")
            .description(quote! {
                /// Computes the arctangent of a number.
                ///
                /// Return value is in radians in the range [-pi/2, pi/2];
                ///
                /// See [`f64::atan()`] for more details.
            })
            .result(Box::new(|float| {
                Some(FloatSpecifications {
                    accept_negative: float.s.accept_negative,
                    accept_positive: float.s.accept_positive,
                    accept_zero: float.s.accept_zero,
                    accept_inf: false,
                })
            }))
            .build(),
        OpBuilder::new("exp_m1")
            .description(quote! {
                /// Returns `e^(self) - 1` in a way that is accurate even if the number is close to zero.
                ///
                /// See [`f64::exp_m1()`] for more details.
            })
            .result(Box::new(|float| {
                Some(FloatSpecifications {
                    accept_negative: float.s.accept_negative,
                    accept_positive: float.s.accept_positive,
                    accept_zero: float.s.accept_zero,
                    accept_inf: float.s.accept_positive,
                })
            }))
            .build(),
        OpBuilder::new("ln_1p")
            .description(quote! {
                /// Returns `ln(1+n)` (natural logarithm) more accurately than if the operations were performed separately.
                ///
                /// See [`f64::ln_1p()`] for more details.
            })
            .result(Box::new(|float| {
                if float.s.accept_negative {
                    return None;
                }

                Some(FloatSpecifications {
                    accept_negative: false,
                    accept_positive: true,
                    accept_zero: float.s.accept_zero,
                    accept_inf: float.s.accept_inf,
                })
            }))
            .build(),
        OpBuilder::new("sinh")
            .description(quote! {
                /// Hyperbolic sine function.
                ///
                /// See [`f64::sinh()`] for more details.
            })
            .result(Box::new(|float| {
                Some(FloatSpecifications {
                    accept_negative: float.s.accept_negative,
                    accept_positive: float.s.accept_positive,
                    accept_zero: float.s.accept_zero,
                    accept_inf: true,
                })
            }))
            .build(),
        OpBuilder::new("cosh")
            .description(quote! {
                /// Hyperbolic cosine  function.
                ///
                /// See [`f64::cosh()`] for more details.
            })
            .result(Box::new(|_| {
                Some(FloatSpecifications {
                    accept_negative: false,
                    accept_positive: true,
                    accept_zero: false,
                    accept_inf: true,
                })
            }))
            .build(),
    ]
}
