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
                /// let x: NonNaN = 3.0.try_into().unwrap();
                /// let y: NonNaN = (-4.0).try_into().unwrap();
                /// let z: NonNaN = (-0.0).try_into().unwrap();
                ///
                /// assert_eq!(x.abs().get(), 3.0);
                /// assert_eq!(y.abs().get(), 4.0);
                /// assert!(z.abs().is_sign_positive());
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
                /// let x: NonNaN = 3.5.try_into().unwrap();
                /// let y: NonNaN = (-3.5).try_into().unwrap();
                ///
                /// assert_eq!(x.ceil().get(), 4.0);
                /// assert_eq!(y.ceil().get(), -3.0);
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
                /// let x: NonNaN = 3.5.try_into().unwrap();
                /// let y: NonNaN = (-3.5).try_into().unwrap();
                ///
                /// assert_eq!(x.floor().get(), 3.0);
                /// assert_eq!(y.floor().get(), -4.0);
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
                /// let x: NonNaN = 3.5.try_into().unwrap();
                /// let y: NonNaN = (-3.5).try_into().unwrap();
                ///
                /// assert_eq!(x.round().get(), 4.0);
                /// assert_eq!(y.round().get(), -4.0);
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
                /// let x: NonNaN = 3.5.try_into().unwrap();
                /// let y: NonNaN = (-3.5).try_into().unwrap();
                ///
                /// assert_eq!(x.trunc().get(), 3.0);
                /// assert_eq!(y.trunc().get(), -3.0);
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
                ///
                /// # Examples
                ///
                /// ```
                /// # use typed_floats::*;
                /// let x: NonNaN = 3.5.try_into().unwrap();
                /// let y: NonNaN = (-3.5).try_into().unwrap();
                /// let z: NonNaN = (-3.0).try_into().unwrap();
                ///
                /// assert_eq!(x.fract(), 0.5);
                /// assert_eq!(y.fract(), -0.5);
                /// assert_eq!(z.fract(), 0.0);
                /// assert_eq!(z.fract().is_sign_positive(), true);
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
                /// let c: NonNaN = 0.0.try_into().unwrap();
                /// let d: NonNaN = (-0.0).try_into().unwrap();
                ///
                /// assert_eq!(a.signum().get(), 1.0);
                /// assert_eq!(b.signum().get(), -1.0);
                /// assert_eq!(c.signum().get(), 1.0);
                /// assert_eq!(d.signum().get(), -1.0);
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
                /// (It returns `-0.0` if `self` is `-0.0`.)
                ///
                /// # Examples
                ///
                /// ```
                /// # use typed_floats::*;
                /// let a: NonNaN = 25.0.try_into().unwrap();
                /// let b: NonNaN = (-1).try_into().unwrap();
                /// let c: NonNaN = 0.0.try_into().unwrap();
                /// let d: NonNaN = (-0.0).try_into().unwrap();
                ///
                /// assert_eq!(a.sqrt(), 5.0);
                /// assert_eq!(b.sqrt().is_nan(), true);
                /// assert_eq!(c.sqrt(), 0.0);
                /// assert_eq!(d.sqrt(), -0.0);
                /// assert_eq!(d.sqrt().is_sign_negative(), true);
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
                /// let b: NonNaN = 0.0.try_into().unwrap();
                /// let c: NonNaN = (-0.0).try_into().unwrap();
                /// let d: NonNaN = (-1.0).try_into().unwrap();
                ///
                /// assert_eq!(a.exp().get(), core::f64::consts::E);
                /// assert_eq!(b.exp().get(), 1.0);
                /// assert_eq!(c.exp().get(), 1.0);
                /// assert_eq!(d.exp().get(), 1.0 / core::f64::consts::E);
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
                /// let b: NonNaN = 0.0.try_into().unwrap();
                /// let c: NonNaN = (-0.0).try_into().unwrap();
                /// let d: NonNaN = (-2.0).try_into().unwrap();
                ///
                /// assert_eq!(a.exp2().get(), 4.0);
                /// assert_eq!(b.exp2().get(), 1.0);
                /// assert_eq!(c.exp2().get(), 1.0);
                /// assert_eq!(d.exp2().get(), 0.25);
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
                /// let b: NonNaN = 0.0.try_into().unwrap();
                /// let c: NonNaN = (-0.0).try_into().unwrap();
                /// let d: NonNaN = (-1.0).try_into().unwrap();
                ///
                /// assert_eq!(a.ln(), 0.0);
                /// assert_eq!(b.ln(), f64::NEG_INFINITY);
                /// assert_eq!(c.ln(), f64::NEG_INFINITY);
                /// assert_eq!(d.ln().is_nan(), true);
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
                /// let c: NonNaN = 0.0.try_into().unwrap();
                /// let d: NonNaN = (-0.0).try_into().unwrap();
                /// let e: NonNaN = (-1.0).try_into().unwrap();
                ///
                /// assert_eq!(a.log2(), 1.0);
                /// assert_eq!(b.log2(), 0.0);
                /// assert_eq!(c.log2(), f64::NEG_INFINITY);
                /// assert_eq!(d.log2(), f64::NEG_INFINITY);
                /// assert_eq!(e.log2().is_nan(), true);
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
                /// let c: NonNaN = 0.0.try_into().unwrap();
                /// let d: NonNaN = (-0.0).try_into().unwrap();
                /// let e: NonNaN = (-1.0).try_into().unwrap();
                ///
                /// assert_eq!(a.log10(), 2.0);
                /// assert_eq!(b.log10(), 0.0);
                /// assert_eq!(c.log10(), f64::NEG_INFINITY);
                /// assert_eq!(d.log10(), f64::NEG_INFINITY);
                /// assert_eq!(e.log10().is_nan(), true);
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
                /// let b: NonNaN = 0.0.try_into().unwrap();
                /// let c: NonNaN = (-core::f64::consts::PI).try_into().unwrap();
                /// let d: NonNaN = (2.0 * core::f64::consts::PI).try_into().unwrap();
                ///
                /// assert_eq!(a.to_degrees().get(), 180.0);
                /// assert_eq!(b.to_degrees().get(), 0.0);
                /// assert_eq!(c.to_degrees().get(), -180.0);
                /// assert_eq!(d.to_degrees().get(), 360.0);
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
                /// assert_eq!(a.to_radians().get(), core::f64::consts::PI);
                /// assert_eq!(b.to_radians().get(), 0.0);
                /// assert_eq!(c.to_radians().get(), -core::f64::consts::PI);
                /// assert_eq!(d.to_radians().get(), 2.0 * core::f64::consts::PI);
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
                /// let b: NonNaN = 0.0.try_into().unwrap();
                /// let c: NonNaN = (-0.0).try_into().unwrap();
                /// let d: NonNaN = (-1.0).try_into().unwrap();
                ///
                /// assert_eq!(a.cbrt().get(), 2.0);
                /// assert_eq!(b.cbrt().get(), 0.0);
                /// assert_eq!(c.cbrt().get(), -0.0);
                /// assert_eq!(d.cbrt().get(), -1.0);
                /// ```
                ///
                /// See [`f64::cbrt()`] for more details.
            })
            .result(Box::new(|float| Some(float.s.clone())))
            .build(),
        OpBuilder::new("sin")
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
                /// assert!(a.sin().get().abs() < 1.0e-10);
                /// assert!(b.sin().get().abs() < 1.0e-10);
                /// assert!(c.sin().get().abs() < 1.0e-10);
                /// assert!(d.sin().get().abs() < 1.0e-10);
                /// assert!((e.sin().get() - 1.0).abs() < 1.0e-10);
                /// assert!((f.sin().get() + 1.0).abs() < 1.0e-10);
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
                /// assert!((a.cos().get() + 1.0).abs() < 1.0e-10);
                /// assert!((b.cos().get() - 1.0).abs() < 1.0e-10);
                /// assert!((c.cos().get() - 1.0).abs() < 1.0e-10);
                /// assert!((d.cos().get() + 1.0).abs() < 1.0e-10);
                /// assert!(e.cos().get().abs() < 1.0e-10);
                /// assert!(f.cos().get().abs() < 1.0e-10);
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
    ]
}
