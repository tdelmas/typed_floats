use quote::quote;

use crate::types::*;

pub(crate) fn get_impl_self_rhs() -> Vec<OpRhs> {
    vec![
        OpRhs::new(
            "add",
            "+",
            "add",
            "Add",
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
            "sub",
            "Sub",
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
            "rem",
            "Rem",
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
            "div",
            "Div",
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
            "mul",
            "Mul",
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
    ]
}
