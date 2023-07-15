use quote::quote;

use crate::types::*;

pub(crate) fn get_impl_self_rhs() -> Vec<OpRhs> {
    vec![
        OpRhs::new(
            "add",
            "+",
            ("core::ops::Add", "add"),
            Some(("core::ops::AddAssign", "add_assign")),
            Box::new(|_, _| quote! { self.get() + rhs.get() }),
            Box::new(|var1, var2| {
                quote! { #var1 + #var2 }
            }),
            Box::new(|float, rhs, floats| {
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
                    true => None,
                    false => {
                        let spec = FloatSpecifications {
                            accept_inf: spec_a.accept_inf || spec_b.accept_inf || can_sign_be_same,
                            accept_zero: can_sign_be_different
                                || (spec_a.accept_zero && spec_b.accept_zero),
                            accept_positive: spec_a.accept_positive || spec_b.accept_positive,
                            accept_negative: spec_a.accept_negative || spec_b.accept_negative,
                        };

                        find_float(&spec, floats)
                    }
                }
            }),
        ),
        OpRhs::new(
            "sub",
            "-",
            ("core::ops::Sub", "sub"),
            Some(("core::ops::SubAssign", "sub_assign")),
            Box::new(|_, _| quote! { self.get() - rhs.get() }),
            Box::new(|var1, var2| {
                quote! { #var1 - #var2 }
            }),
            Box::new(|float, rhs, floats| {
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
                    true => None,
                    false => {
                        let spec = FloatSpecifications {
                            accept_inf: spec_a.accept_inf || spec_b.accept_inf || can_overflow,
                            accept_zero: can_sign_be_same
                                || (spec_a.accept_zero && spec_b.accept_zero),
                            accept_positive: spec_a.accept_positive || spec_b.accept_negative,
                            accept_negative: spec_a.accept_negative || spec_b.accept_positive,
                        };
                        find_float(&spec, floats)
                    }
                }
            }),
        ),
        OpRhs::new(
            "rem",
            "%",
            ("core::ops::Rem", "rem"),
            Some(("core::ops::RemAssign", "rem_assign")),
            Box::new(|_, _| quote! { self.get() % rhs.get() }),
            Box::new(|var1, var2| {
                quote! { #var1 % #var2 }
            }),
            Box::new(|float, rhs, floats| {
                let spec_a = &float.s;
                let spec_b = &rhs.s;

                let can_be_nan = spec_b.accept_zero || spec_a.accept_inf;

                match can_be_nan {
                    true => None,
                    false => {
                        let output_def = FloatSpecifications {
                            accept_inf: false,
                            accept_zero: true,
                            accept_positive: spec_a.accept_positive,
                            accept_negative: spec_a.accept_negative,
                        };

                        find_float(&output_def, floats)
                    }
                }
            }),
        ),
        OpRhs::new(
            "div",
            "/",
            ("core::ops::Div", "div"),
            Some(("core::ops::DivAssign", "div_assign")),
            Box::new(|_, _| quote! { self.get() / rhs.get() }),
            Box::new(|var1, var2| {
                quote! { #var1 / #var2 }
            }),
            Box::new(|float, rhs, floats| {
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
                    true => None,
                    false => {
                        let output_def = FloatSpecifications {
                            accept_inf: true,
                            accept_zero: true,
                            accept_positive: can_sign_be_same,
                            accept_negative: can_sign_be_different,
                        };

                        find_float(&output_def, floats)
                    }
                }
            }),
        ),
        OpRhs::new(
            "mul",
            "*",
            ("core::ops::Mul", "mul"),
            Some(("core::ops::MulAssign", "mul_assign")),
            Box::new(|_, _| quote! { self.get() * rhs.get() }),
            Box::new(|var1, var2| {
                quote! { #var1 * #var2 }
            }),
            Box::new(|float, rhs, floats| {
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
                    true => None,
                    false => {
                        let output_def = FloatSpecifications {
                            accept_inf: true,  // it can always overflow
                            accept_zero: true, // it can always round to zero
                            accept_positive: can_sign_be_same,
                            accept_negative: can_sign_be_different,
                        };

                        find_float(&output_def, floats)
                    }
                }
            }),
        ),
        OpRhs::new(
            "hypot",
            "hypot",
            ("Hypot", "hypot"),
            None,
            Box::new(|_, _| quote! { self.get().hypot(rhs.get()) }),
            Box::new(|var1, var2| {
                quote! { #var1.hypot(#var2) }
            }),
            Box::new(|float, rhs, floats| {
                let output_def = FloatSpecifications {
                    accept_inf: true, // it can always overflow
                    accept_zero: float.s.accept_zero && rhs.s.accept_zero,
                    accept_positive: true,
                    accept_negative: false,
                };

                find_float(&output_def, floats)
            }),
        ),
        OpRhs::new(
            "min",
            "min",
            ("Min", "min"),
            None,
            Box::new(|_, _| quote! { self.get().min(rhs.get()) }),
            Box::new(|var1, var2| {
                quote! { Min::min(#var1,#var2) }
            }),
            Box::new(|float, rhs, floats| {
                let output_def;
                // https://llvm.org/docs/LangRef.html#llvm-minnum-intrinsic
                // fmin(+0.0, -0.0) returns either operand.
                // (0.0_f64).min(-0.0_f64) == 0.0_f64
                let can_confuse_zero = float.s.accept_zero
                    && rhs.s.accept_zero
                    && (float.s.accept_positive || rhs.s.accept_positive);

                let can_be_neg_inf = (float.s.accept_negative && float.s.accept_inf)
                    || (rhs.s.accept_negative && rhs.s.accept_inf);
                let can_be_pos_inf = float.s.accept_positive
                    && float.s.accept_inf
                    && rhs.s.accept_positive
                    && rhs.s.accept_inf;
                let accept_inf = can_be_neg_inf || can_be_pos_inf;

                if !float.s.accept_positive {
                    output_def = FloatSpecifications {
                        accept_inf,
                        accept_zero: float.s.accept_zero
                            && (rhs.s.accept_zero || rhs.s.accept_positive),
                        accept_positive: false,
                        accept_negative: true,
                    };
                } else if !rhs.s.accept_positive {
                    let accept_zero =
                        rhs.s.accept_zero && (float.s.accept_zero || float.s.accept_positive);

                    output_def = FloatSpecifications {
                        accept_inf,
                        accept_zero,
                        accept_positive: accept_zero && can_confuse_zero,
                        accept_negative: true,
                    };
                } else if !float.s.accept_negative && !rhs.s.accept_negative {
                    output_def = FloatSpecifications {
                        accept_inf,
                        accept_zero: float.s.accept_zero || rhs.s.accept_zero,
                        accept_positive: true,
                        accept_negative: false,
                    };
                } else {
                    output_def = FloatSpecifications {
                        accept_inf: can_be_neg_inf || can_be_pos_inf,
                        accept_zero: float.s.accept_zero || rhs.s.accept_zero,
                        accept_positive: true,
                        accept_negative: true,
                    };
                }

                find_float(&output_def, floats)
            }),
        ),
        OpRhs::new(
            "max",
            "max",
            ("Max", "max"),
            None,
            Box::new(|_, _| quote! { self.get().max(rhs.get()) }),
            Box::new(|var1, var2| {
                quote! { Max::max(#var1,#var2) }
            }),
            Box::new(|float, rhs, floats| {
                let output_def;
                // https://llvm.org/docs/LangRef.html#llvm-maxnum-intrinsic
                // fmin(+0.0, -0.0) returns either -0.0 or 0.0
                let can_confuse_zero = float.s.accept_zero
                    && rhs.s.accept_zero
                    && (float.s.accept_negative || rhs.s.accept_negative);

                let can_be_neg_inf = (float.s.accept_negative && float.s.accept_inf)
                    && (rhs.s.accept_negative && rhs.s.accept_inf);
                let can_be_pos_inf = (float.s.accept_positive
                    && float.s.accept_inf)
                    || (rhs.s.accept_positive
                    && rhs.s.accept_inf);

                let accept_inf = can_be_neg_inf || can_be_pos_inf;

                if !float.s.accept_negative {
                    output_def = FloatSpecifications {
                        accept_inf,
                        accept_zero: float.s.accept_zero
                            && (rhs.s.accept_zero || rhs.s.accept_negative),
                        accept_positive: true,
                        accept_negative: false,
                    };
                } else if !rhs.s.accept_negative {
                    let accept_zero =
                        rhs.s.accept_zero && (float.s.accept_zero || float.s.accept_negative);

                    output_def = FloatSpecifications {
                        accept_inf,
                        accept_zero,
                        accept_positive: true,
                        accept_negative: accept_zero && can_confuse_zero,
                    };
                } else if !float.s.accept_positive && !rhs.s.accept_positive {
                    output_def = FloatSpecifications {
                        accept_inf,
                        accept_zero: float.s.accept_zero || rhs.s.accept_zero,
                        accept_positive: false,
                        accept_negative: true,
                    };
                } else {
                    output_def = FloatSpecifications {
                        accept_inf: can_be_neg_inf || can_be_pos_inf,
                        accept_zero: float.s.accept_zero || rhs.s.accept_zero,
                        accept_positive: true,
                        accept_negative: true,
                    };
                }

                find_float(&output_def, floats)
            }),
        ),
    ]
}
