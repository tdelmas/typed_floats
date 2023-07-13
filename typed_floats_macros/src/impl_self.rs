use quote::quote;

use crate::types::*;

pub(crate) fn get_impl_self() -> Vec<Op> {
    vec![
        Op::new(
            "neg",
            "-",
            "neg",
            Some("core::ops::Neg"),
            Box::new(|_| {
                quote! { -self.get() }
            }),
            Box::new(|var| {
                quote! { -#var }
            }),
            Box::new(|float, floats| {
                let mut output_spec = float.s.clone();

                if !output_spec.accept_positive {
                    output_spec.accept_positive = true;
                    output_spec.accept_negative = false;
                } else if !output_spec.accept_negative {
                    output_spec.accept_positive = false;
                    output_spec.accept_negative = true;
                }

                find_float(&output_spec, floats)
            }),
        ),
        Op::new(
            "abs",
            "abs",
            "abs",
            None,
            Box::new(|float| {
                if !float.s.accept_negative {
                    // no-op
                    quote! {self.get() }
                } else if !float.s.accept_positive {
                    // inv
                    quote! { -self.get() }
                } else {
                    quote! { self.get().abs() }
                }
            }),
            Box::new(|var| {
                quote! { #var.abs() }
            }),
            Box::new(|float, floats| {
                let mut output_spec = float.s.clone();

                output_spec.accept_positive = true;
                output_spec.accept_negative = false;

                find_float(&output_spec, floats)
            }),
        ),
        Op::new(
            "ceil",
            "ceil",
            "ceil",
            None,
            Box::new(|_| quote! { self.get().ceil() }),
            Box::new(|var| {
                quote! { #var.ceil() }
            }),
            Box::new(|float, floats| {
                if float.s.accept_negative {
                    let mut output_spec = float.s.clone();
                    output_spec.accept_zero = true;

                    find_float(&output_spec, floats)
                } else {
                    Some(float.clone())
                }
            }),
        ),
        Op::new(
            "floor",
            "floor",
            "floor",
            None,
            Box::new(|_| quote! { self.get().floor() }),
            Box::new(|var| {
                quote! { #var.floor() }
            }),
            Box::new(|float, floats| {
                if float.s.accept_positive {
                    let mut output_spec = float.s.clone();
                    output_spec.accept_zero = true;

                    find_float(&output_spec, floats)
                } else {
                    Some(float.clone())
                }
            }),
        ),
        Op::new(
            "round",
            "round",
            "round",
            None,
            Box::new(|_| quote! { self.get().round() }),
            Box::new(|var| {
                quote! { #var.round() }
            }),
            Box::new(|float, floats| {
                let mut output_spec = float.s.clone();
                output_spec.accept_zero = true;

                find_float(&output_spec, floats)
            }),
        ),
        Op::new(
            "trunc",
            "trunc",
            "trunc",
            None,
            Box::new(|_| quote! { self.get().trunc() }),
            Box::new(|var| {
                quote! { #var.trunc() }
            }),
            Box::new(|float, floats| {
                let mut output_spec = float.s.clone();

                output_spec.accept_zero = true;

                find_float(&output_spec, floats)
            }),
        ),
        Op::new(
            "fract",
            "fract",
            "fract",
            None,
            Box::new(|_| quote! { self.get().fract() }),
            Box::new(|var| {
                quote! { #var.fract() }
            }),
            Box::new(|float, floats| {
                if float.s.accept_inf {
                    return None;
                }

                let mut output_spec = float.s.clone();
                output_spec.accept_zero = true;
                // Returns POSITIVE zero if the factional part is zero
                output_spec.accept_positive = true;

                find_float(&output_spec, floats)
            }),
        ),
        Op::new(
            "signum",
            "signum",
            "signum",
            None,
            Box::new(|float| {
                if !float.s.accept_negative {
                    quote! { 1.0 }
                } else if !float.s.accept_positive {
                    quote! { -1.0 }
                } else {
                    quote! { self.get().signum() }
                }
            }),
            Box::new(|var| {
                quote! { #var.signum() }
            }),
            Box::new(|float, floats| {
                let spec;

                if !float.s.accept_negative {
                    spec = FloatSpecifications {
                        accept_negative: false,
                        accept_positive: true,
                        accept_zero: false,
                        accept_inf: false,
                    };
                } else if !float.s.accept_positive {
                    spec = FloatSpecifications {
                        accept_negative: true,
                        accept_positive: false,
                        accept_zero: false,
                        accept_inf: false,
                    };
                } else {
                    spec = FloatSpecifications {
                        accept_negative: true,
                        accept_positive: true,
                        accept_zero: false,
                        accept_inf: false,
                    };
                }

                find_float(&spec, floats)
            }),
        ),
        Op::new(
            "sqrt",
            "sqrt",
            "sqrt",
            None,
            Box::new(|float| {
                // sqrt(-0.0) = -0.0
                if !float.s.accept_positive && !float.s.accept_zero {
                    let float_type = float.float_type_ident();

                    quote! { #float_type::NAN }
                } else {
                    quote! { self.get().sqrt() }
                }
            }),
            Box::new(|var| {
                quote! { #var.sqrt() }
            }),
            Box::new(|float, _| {
                if float.s.accept_negative {
                    return None;
                }

                Some(float.clone())
            }),
        ),
        Op::new(
            "exp",
            "exp",
            "exp",
            None,
            Box::new(|_| {
                quote! { self.get().exp() }
            }),
            Box::new(|var| {
                quote! { #var.exp() }
            }),
            Box::new(|float, floats| {
                let spec = FloatSpecifications {
                    accept_negative: false,
                    accept_positive: true,
                    accept_zero: float.s.accept_negative,
                    accept_inf: float.s.accept_positive,
                };

                find_float(&spec, floats)
            }),
        ),
        Op::new(
            "exp2",
            "exp2",
            "exp2",
            None,
            Box::new(|float| {
                quote! { self.get().exp2() }
            }),
            Box::new(|var| {
                quote! { #var.exp2() }
            }),
            Box::new(|float, floats| {
                let spec = FloatSpecifications {
                    accept_negative: false,
                    accept_positive: true,
                    accept_zero: float.s.accept_negative,
                    accept_inf: float.s.accept_positive,
                };

                find_float(&spec, floats)
            }),
        ),
        Op::new(
            "ln",
            "ln",
            "ln",
            None,
            Box::new(|float| {
                let is_strictly_negative = !float.s.accept_positive && !float.s.accept_zero;

                if is_strictly_negative {
                    let float_type = float.float_type_ident();

                    quote! { #float_type::NAN }
                } else {
                    quote! { self.get().ln() }
                }
            }),
            Box::new(|var| {
                quote! { #var.ln() }
            }),
            Box::new(|float, floats| {
                if float.s.accept_negative {
                    return None;
                }

                let spec = FloatSpecifications {
                    accept_negative: true,
                    accept_positive: true,
                    accept_zero: true,
                    accept_inf: float.s.accept_inf || float.s.accept_zero,
                };

                find_float(&spec, floats)
            }),
        ),
        Op::new(
            "log2",
            "log2",
            "log2",
            None,
            Box::new(|float| {
                quote! { self.get().log2() }
            }),
            Box::new(|var| {
                quote! { #var.log2() }
            }),
            Box::new(|float, floats| {
                if float.s.accept_negative {
                    return None;
                }

                let spec = FloatSpecifications {
                    accept_negative: true,
                    accept_positive: true,
                    accept_zero: true,
                    accept_inf: float.s.accept_inf || float.s.accept_zero,
                };

                find_float(&spec, floats)
            }),
        ),
        Op::new(
            "log10",
            "log10",
            "log10",
            None,
            Box::new(|float| {
                quote! { self.get().log10() }
            }),
            Box::new(|var| {
                quote! { #var.log10() }
            }),
            Box::new(|float, floats| {
                if float.s.accept_negative {
                    return None;
                }

                let spec = FloatSpecifications {
                    accept_negative: true,
                    accept_positive: true,
                    accept_zero: true,
                    accept_inf: float.s.accept_inf || float.s.accept_zero,
                };

                find_float(&spec, floats)
            }),
        ),
    ]
}
