use quote::quote;

use crate::types::*;

pub(crate) fn get_neg() -> Op {
    Op::new(
        "neg",
        Some("core::ops::Neg"),
        Box::new(|_| {
            quote! { -self.get() }
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
    )
}

pub(crate) fn get_abs() -> Op {
    Op::new(
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
        Box::new(|float, floats| {
            let mut output_spec = float.s.clone();

            output_spec.accept_positive = true;
            output_spec.accept_negative = false;

            find_float(&output_spec, floats)
        }),
    )
}

pub(crate) fn get_ceil() -> Op {
    Op::new(
        "ceil",
        None,
        Box::new(|_| quote! { self.get().ceil() }),
        Box::new(|float, floats| {
            if float.s.accept_negative {
                let mut output_spec = float.s.clone();
                output_spec.accept_zero = true;

                find_float(&output_spec, floats)
            } else {
                Some(float.clone())
            }
        }),
    )
}

pub(crate) fn get_floor() -> Op {
    Op::new(
        "floor",
        None,
        Box::new(|_| quote! { self.get().floor() }),
        Box::new(|float, floats| {
            if float.s.accept_positive {
                let mut output_spec = float.s.clone();
                output_spec.accept_zero = true;

                find_float(&output_spec, floats)
            } else {
                Some(float.clone())
            }
        }),
    )
}

pub(crate) fn get_round() -> Op {
    Op::new(
        "round",
        None,
        Box::new(|_| quote! { self.get().round() }),
        Box::new(|float, floats| {
            let mut output_spec = float.s.clone();
            output_spec.accept_zero = true;

            find_float(&output_spec, floats)
        }),
    )
}
