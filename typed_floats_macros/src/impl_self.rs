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
            .result(Box::new(|float| {
                let mut output_spec = float.s.clone();

                if float.s.accept_negative {
                    output_spec.accept_zero = true;
                }

                Some(output_spec)
            }))
            .build(),
        OpBuilder::new("floor")
            .result(Box::new(|float| {
                let mut output_spec = float.s.clone();

                if float.s.accept_positive {
                    output_spec.accept_zero = true;
                }

                Some(output_spec)
            }))
            .build(),
        OpBuilder::new("round")
            .result(Box::new(|float| {
                let mut output_spec = float.s.clone();

                output_spec.accept_zero = true;

                Some(output_spec)
            }))
            .build(),
        OpBuilder::new("trunc")
            .result(Box::new(|float| {
                let mut output_spec = float.s.clone();

                output_spec.accept_zero = true;

                Some(output_spec)
            }))
            .build(),
        OpBuilder::new("fract")
        .comment("Returns +0.0 if the factional part is zero")
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
        .comment("sqrt(-0.0) = -0.0")
            .op_fn(Box::new(|float| {
                // sqrt(-0.0) = -0.0
                if !float.s.accept_positive && !float.s.accept_zero {
                    let float_type = float.float_type_ident();

                    quote! { #float_type::NAN }
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
            .op_fn(Box::new(|float| {
                let is_strictly_negative = !float.s.accept_positive && !float.s.accept_zero;

                if is_strictly_negative {
                    let float_type = float.float_type_ident();

                    quote! { #float_type::NAN }
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
    ]
}
