use quote::quote;

use crate::types::*;

fn can_one_be_zero_neg_and_the_other_zero_pos(
    spec_a: &FloatSpecifications,
    spec_b: &FloatSpecifications,
) -> bool {
    let can_a_be_zero_neg = spec_a.accept_zero && spec_a.accept_negative;
    let can_a_be_zero_pos = spec_a.accept_zero && spec_a.accept_positive;
    let can_b_be_zero_neg = spec_b.accept_zero && spec_b.accept_negative;
    let can_b_be_zero_pos = spec_b.accept_zero && spec_b.accept_positive;

    (can_a_be_zero_neg && can_b_be_zero_pos) || (can_a_be_zero_pos && can_b_be_zero_neg)
}

pub(crate) fn get_impl_self_rhs() -> Vec<OpRhs> {
    vec![
        OpRhsBuilder::new("core::ops::Add", "add")
            .with_assign("core::ops::AddAssign", "add_assign")
            .bin_op("+")
            .op_fn(Box::new(|_, _| quote! { self.get() + rhs.get() }))
            .op_test(Box::new(|var1, var2| quote! { #var1 + #var2 }))
            .op_is_commutative()
            .comment("The addition of two opposite infinity is `NaN`.")
            .result(Box::new(|float, rhs| {
                let spec_a = &float.s;
                let spec_b = &rhs.s;

                let can_sign_be_different = (spec_a.accept_negative && spec_b.accept_positive)
                    || (spec_a.accept_positive && spec_b.accept_negative);
                let can_sign_be_same = (spec_a.accept_negative && spec_b.accept_negative)
                    || (spec_a.accept_positive && spec_b.accept_positive);

                let can_add_inf_and_negative_inf = (spec_a.accept_inf
                    && spec_a.accept_negative
                    && spec_b.accept_inf
                    && spec_b.accept_positive)
                    || (spec_b.accept_inf
                        && spec_b.accept_negative
                        && spec_a.accept_inf
                        && spec_a.accept_positive);

                let can_be_nan = can_add_inf_and_negative_inf;

                match can_be_nan {
                    true => ReturnTypeSpecification::NativeFloat,
                    false => ReturnTypeSpecification::FloatSpecifications(FloatSpecifications {
                        accept_inf: spec_a.accept_inf || spec_b.accept_inf || can_sign_be_same,
                        accept_zero: can_sign_be_different
                            || (spec_a.accept_zero && spec_b.accept_zero),
                        accept_positive: spec_a.accept_positive || spec_b.accept_positive,
                        accept_negative: spec_a.accept_negative || spec_b.accept_negative,
                    }),
                }
            }))
            .build(),
        OpRhsBuilder::new("core::ops::Sub", "sub")
            .with_assign("core::ops::SubAssign", "sub_assign")
            .bin_op("-")
            .comment("The substraction of two infinity of the same sign is `NaN`")
            .result(Box::new(|float, rhs| {
                let spec_a = &float.s;
                let spec_b = &rhs.s;

                let can_sign_be_different = (spec_a.accept_negative && spec_b.accept_positive)
                    || (spec_a.accept_positive && spec_b.accept_negative);
                let can_sign_be_same = (spec_a.accept_negative && spec_b.accept_negative)
                    || (spec_a.accept_positive && spec_b.accept_positive);

                let can_overflow = can_sign_be_different;

                let can_sub_inf_and_inf =
                    spec_a.accept_inf && spec_b.accept_inf && can_sign_be_same;

                let can_be_nan = can_sub_inf_and_inf;

                match can_be_nan {
                    true => ReturnTypeSpecification::NativeFloat,
                    false => ReturnTypeSpecification::FloatSpecifications(FloatSpecifications {
                        accept_inf: spec_a.accept_inf || spec_b.accept_inf || can_overflow,
                        accept_zero: can_sign_be_same || (spec_a.accept_zero && spec_b.accept_zero),
                        accept_positive: spec_a.accept_positive || spec_b.accept_negative,
                        accept_negative: spec_a.accept_negative || spec_b.accept_positive,
                    }),
                }
            }))
            .build(),
        OpRhsBuilder::new("core::ops::Rem", "rem")
            .with_assign("core::ops::RemAssign", "rem_assign")
            .bin_op("%")
            .result(Box::new(|float, rhs| {
                let spec_a = &float.s;
                let spec_b = &rhs.s;

                let can_be_nan = spec_b.accept_zero || spec_a.accept_inf;

                match can_be_nan {
                    true => ReturnTypeSpecification::NativeFloat,
                    false => ReturnTypeSpecification::FloatSpecifications(FloatSpecifications {
                        accept_inf: false,
                        accept_zero: true,
                        accept_positive: spec_a.accept_positive,
                        accept_negative: spec_a.accept_negative,
                    }),
                }
            }))
            .build(),
        OpRhsBuilder::new("core::ops::Div", "div")
            .with_assign("core::ops::DivAssign", "div_assign")
            .bin_op("/")
            .comment("`NaN` can be generated by dividing zero by zero or infinity by infinity.")
            .result(Box::new(|float, rhs| {
                let spec_a = &float.s;
                let spec_b = &rhs.s;

                let can_zero_divide_zero = spec_a.accept_zero && spec_b.accept_zero;
                let can_inf_divide_inf = spec_a.accept_inf && spec_b.accept_inf;

                let can_sign_be_different = (spec_a.accept_negative && spec_b.accept_positive)
                    || (spec_a.accept_positive && spec_b.accept_negative);
                let can_sign_be_same = (spec_a.accept_negative && spec_b.accept_negative)
                    || (spec_a.accept_positive && spec_b.accept_positive);

                let can_be_nan = can_zero_divide_zero || can_inf_divide_inf;

                match can_be_nan {
                    true => ReturnTypeSpecification::NativeFloat,
                    false => ReturnTypeSpecification::FloatSpecifications(FloatSpecifications {
                        accept_inf: true,
                        accept_zero: true,
                        accept_positive: can_sign_be_same,
                        accept_negative: can_sign_be_different,
                    }),
                }
            }))
            .build(),
        OpRhsBuilder::new("core::ops::Mul", "mul")
            .with_assign("core::ops::MulAssign", "mul_assign")
            .bin_op("*")
            .op_is_commutative()
            .comment("The result of zero multiplied by infinity is `NaN`. Rounding errors may generate zero from non-zero values.")
            .result(Box::new(|float, rhs| {
                let spec_a = &float.s;
                let spec_b = &rhs.s;

                let can_sign_be_different = (spec_a.accept_negative && spec_b.accept_positive)
                    || (spec_a.accept_positive && spec_b.accept_negative);
                let can_sign_be_same = (spec_a.accept_negative && spec_b.accept_negative)
                    || (spec_a.accept_positive && spec_b.accept_positive);

                let can_zero_multiply_inf = spec_a.accept_zero && spec_b.accept_inf
                    || spec_a.accept_inf && spec_b.accept_zero;

                let can_be_nan = can_zero_multiply_inf;

                match can_be_nan {
                    true => ReturnTypeSpecification::NativeFloat,
                    false => {
                        ReturnTypeSpecification::FloatSpecifications(FloatSpecifications {
                            accept_inf: true,  // it can always overflow
                            accept_zero: true, // it can always round to zero
                            accept_positive: can_sign_be_same,
                            accept_negative: can_sign_be_different,
                        })
                    }
                }
            }))
            .build(),
            #[cfg(any(feature = "std", feature = "libm"))]
        OpRhsBuilder::new("Hypot", "hypot")
            .op_is_commutative()
            .op_test_primitive(Box::new(|var1, var2| { quote! { #var1.hypot(#var2) } }))
            .result(Box::new(|float, rhs| {
                ReturnTypeSpecification::FloatSpecifications(FloatSpecifications {
                    accept_inf: true, // it can always overflow
                    accept_zero: float.s.accept_zero && rhs.s.accept_zero,
                    accept_positive: true,
                    accept_negative: false,
                })
            }))
            .build(),
        OpRhsBuilder::new("Min", "min")
            .op_is_commutative()
            .op_test_primitive(Box::new(|var1, var2| quote! { #var1.min(#var2) }))
            // Because the result of `min(-0.0,0.0)` depends on the architecture, we cannot check it.
            .skip_check_return_type_strictness()
            .comment("The result type is not always as strict as possible because `min(-0.0,0.0)` may return either.")
            .result(Box::new(|float, rhs| {
                // https://llvm.org/docs/LangRef.html#llvm-minnum-intrinsic
                // fmin(+0.0, -0.0) returns either operand.
                // (0.0_f64).min(-0.0_f64) == 0.0_f64
                let can_confuse_zero = can_one_be_zero_neg_and_the_other_zero_pos(&float.s, &rhs.s);

                let can_be_neg_inf = (float.s.accept_negative && float.s.accept_inf)
                    || (rhs.s.accept_negative && rhs.s.accept_inf);
                let can_be_pos_inf = float.s.accept_positive
                    && float.s.accept_inf
                    && rhs.s.accept_positive
                    && rhs.s.accept_inf;
                let accept_inf = can_be_neg_inf || can_be_pos_inf;

                let output_def = if !float.s.accept_positive {
                    let accept_zero =
                        float.s.accept_zero && (rhs.s.accept_zero || rhs.s.accept_positive);

                    FloatSpecifications {
                        accept_inf,
                        accept_zero,
                        accept_positive: accept_zero || can_confuse_zero,
                        accept_negative: true,
                    }
                } else if !rhs.s.accept_positive {
                    let accept_zero =
                        rhs.s.accept_zero && (float.s.accept_zero || float.s.accept_positive);

                    FloatSpecifications {
                        accept_inf,
                        accept_zero,
                        accept_positive: accept_zero || can_confuse_zero,
                        accept_negative: true,
                    }
                } else if !float.s.accept_negative && !rhs.s.accept_negative {
                    FloatSpecifications {
                        accept_inf,
                        accept_zero: float.s.accept_zero || rhs.s.accept_zero,
                        accept_positive: true,
                        accept_negative: false,
                    }
                } else {
                    FloatSpecifications {
                        accept_inf: can_be_neg_inf || can_be_pos_inf,
                        accept_zero: float.s.accept_zero || rhs.s.accept_zero,
                        accept_positive: true,
                        accept_negative: true,
                    }
                };

                ReturnTypeSpecification::FloatSpecifications(output_def)
            }))
            .build(),
        OpRhsBuilder::new("Max", "max")
            .op_is_commutative()
            .op_test_primitive(Box::new(|var1, var2| quote! { #var1.max(#var2) }))
            // Because the result of `max(-0.0,0.0)` depends on the architecture, we cannot check it.
            .skip_check_return_type_strictness()
            .comment("The result type is not always as strict as possible because `max(-0.0,0.0)` may return either.")
            .result(Box::new(|float, rhs| {
                // https://llvm.org/docs/LangRef.html#llvm-maxnum-intrinsic
                // fmin(+0.0, -0.0) returns either -0.0 or 0.0
                let can_confuse_zero = can_one_be_zero_neg_and_the_other_zero_pos(&float.s, &rhs.s);

                let can_be_neg_inf = (float.s.accept_negative && float.s.accept_inf)
                    && (rhs.s.accept_negative && rhs.s.accept_inf);
                let can_be_pos_inf = (float.s.accept_positive && float.s.accept_inf)
                    || (rhs.s.accept_positive && rhs.s.accept_inf);

                let accept_inf = can_be_neg_inf || can_be_pos_inf;

                let output_def = if !float.s.accept_negative {
                    let accept_zero =
                        float.s.accept_zero && (rhs.s.accept_zero || rhs.s.accept_negative);

                    FloatSpecifications {
                        accept_inf,
                        accept_zero,
                        accept_positive: true,
                        accept_negative: accept_zero && can_confuse_zero,
                    }
                } else if !rhs.s.accept_negative {
                    let accept_zero =
                        rhs.s.accept_zero && (float.s.accept_zero || float.s.accept_negative);

                    FloatSpecifications {
                        accept_inf,
                        accept_zero,
                        accept_positive: true,
                        accept_negative: accept_zero && can_confuse_zero,
                    }
                } else if !float.s.accept_positive && !rhs.s.accept_positive {
                    FloatSpecifications {
                        accept_inf,
                        accept_zero: float.s.accept_zero || rhs.s.accept_zero,
                        accept_positive: false,
                        accept_negative: true,
                    }
                } else {
                    FloatSpecifications {
                        accept_inf: can_be_neg_inf || can_be_pos_inf,
                        accept_zero: float.s.accept_zero || rhs.s.accept_zero,
                        accept_positive: true,
                        accept_negative: true,
                    }
                };

                ReturnTypeSpecification::FloatSpecifications(output_def)
            }))
            .build(),
        #[cfg(any(feature = "std", feature = "libm"))]
        OpRhsBuilder::new("Copysign", "copysign")
            .op_test_primitive(Box::new(|var1, var2| quote! { #var1.copysign(#var2) }))
            .result(Box::new(|float, rhs| {
                ReturnTypeSpecification::FloatSpecifications(FloatSpecifications {
                    accept_inf: float.s.accept_inf,
                    accept_zero: float.s.accept_zero,
                    accept_positive: rhs.s.accept_positive,
                    accept_negative: rhs.s.accept_negative,
                })
            }))
            .build(),
        #[cfg(any(feature = "std", feature = "libm"))]
        OpRhsBuilder::new("DivEuclid", "div_euclid")
            // Because of rounding errors we can't check that the result is always as strict as possible.
            .skip_check_return_type_strictness()
            .op_fn(
                if cfg!(feature = "std") {
                    Box::new(|_, _| quote! { self.get().div_euclid(rhs.get()) })
                } else if cfg!(feature = "libm") {
                    Box::new(|_, _| quote! { self.get().div_euclid(&rhs.get()) })
                } else {
                    Box::new(|_, _| quote! { panic!("No implementation for div_euclid") })
                }
            )
            .op_test_primitive(Box::new(|var1, var2| quote! { 
                if cfg!(feature = "std") {
                    #var1.div_euclid(#var2)
                } else if cfg!(feature = "libm") {
                    #var1.div_euclid(&#var2)
                } else {
                    panic!("No implementation for div_euclid")
                }
            }))
            .result(Box::new(|float, rhs| {
                let spec_a = &float.s;
                let spec_b = &rhs.s;

                let can_be_nan = (spec_a.accept_inf && spec_b.accept_inf)
                    || (spec_a.accept_zero && spec_b.accept_zero);

                let sign_can_be_different = (spec_a.accept_negative && spec_b.accept_positive)
                    || (spec_a.accept_positive && spec_b.accept_negative);

                let sign_can_be_same = (spec_a.accept_negative && spec_b.accept_negative)
                    || (spec_a.accept_positive && spec_b.accept_positive);

                match can_be_nan {
                    true => ReturnTypeSpecification::NativeFloat,
                    false => ReturnTypeSpecification::FloatSpecifications(FloatSpecifications {
                        accept_inf: true,
                        accept_zero: true, // Rounding errors can happen
                        accept_positive: sign_can_be_same,
                        accept_negative: sign_can_be_different,
                    }),
                }
            }))
            .build(),
        #[cfg(any(feature = "std", feature = "libm"))]
        OpRhsBuilder::new("Atan2", "atan2")
            .op_test_primitive(Box::new(|var1, var2| quote! { #var1.atan2(#var2) }))
            // Because of rounding errors we can't check that the result is always as strict as possible.
            .skip_check_return_type_strictness()
            .result(Box::new(|float, rhs| {
                let spec_a = &float.s;
                let spec_b = &rhs.s;

                let can_be_positive = spec_a.accept_positive || spec_b.accept_positive;
                let can_be_negative = spec_a.accept_negative || spec_b.accept_negative;

                ReturnTypeSpecification::FloatSpecifications(FloatSpecifications {
                    accept_inf: false,
                    accept_zero: true,
                    accept_positive: can_be_positive,
                    accept_negative: can_be_negative,
                })
            }))
            .build(),
        #[cfg(any(feature = "std", feature = "libm"))]
        OpRhsBuilder::new("Powf", "powf")
            .op_test_primitive(Box::new(|var1, var2| quote! { #var1.powf(#var2) }))
            .comment("If the base is negative and the exponent is not an integer, the result is `NaN`.")
            .result(Box::new(|float, _| {
                if float.s.accept_negative {
                    ReturnTypeSpecification::NativeFloat
                } else {
                    ReturnTypeSpecification::FloatSpecifications(FloatSpecifications {
                        accept_negative: false,
                        accept_positive: true,
                        accept_zero: true,
                        accept_inf: true,
                    })
                }
            }))
            .build(),
    ]
}
