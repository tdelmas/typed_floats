use proc_macro2::Span;
use quote::quote;
use syn::Ident;

use crate::types::*;

fn impl_op_rhs(
    float: &FloatDefinition,
    rhs: &FloatDefinition,
    output: &Option<FloatDefinition>,
    op: &proc_macro2::TokenStream,
    trait_name: &str,
    fn_name: &str,
) -> proc_macro2::TokenStream {
    let float_full_type = &float.full_type_ident();
    let rhs_full_type = &rhs.full_type_ident();

    let return_value = match output {
        Some(_) => {
            quote! {
                unsafe { Self::Output::new_unchecked(#op) }
            }
        }
        None => {
            quote! { #op }
        }
    };

    let output_name = output_name(output, &float.float_type_ident());

    let trait_ident = Ident::new(trait_name, Span::call_site());
    let fn_ident = Ident::new(fn_name, Span::call_site());

    let trait_assign_ident = Ident::new(&format!("{}Assign", trait_name), Span::call_site());
    let fn_assign_ident = Ident::new(&format!("{}_assign", fn_name), Span::call_site());

    let mut res = quote! {
        impl core::ops::#trait_ident<#rhs_full_type> for #float_full_type {
            type Output = #output_name;

            #[inline]
            fn #fn_ident(self, rhs: #rhs_full_type) -> Self::Output {
                #return_value
            }
        }
    };

    if let Some(output) = output {
        if output.s.can_fit_into(&float.s) {
            res.extend(quote! {
                impl core::ops::#trait_assign_ident<#rhs_full_type> for #float_full_type {
                    #[inline]
                    fn #fn_assign_ident(&mut self, rhs: #rhs_full_type) {
                        unsafe {
                            *self = Self::new_unchecked(#op);
                        }
                    }
                }
            })
        }
    }

    res
}

pub(crate) fn add_result(
    spec_a: &FloatSpecifications,
    spec_b: &FloatSpecifications,
    floats: &[FloatDefinition],
) -> Option<FloatDefinition> {
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
                accept_zero: can_sign_be_different || (spec_a.accept_zero && spec_b.accept_zero),
                accept_positive: spec_a.accept_positive || spec_b.accept_positive,
                accept_negative: spec_a.accept_negative || spec_b.accept_negative,
            };

            find_float(&spec, floats)
        }
    }
}

pub(crate) fn impl_add(
    float_a: &FloatDefinition,
    float_b: &FloatDefinition,
    floats: &[FloatDefinition],
) -> proc_macro2::TokenStream {
    let output = add_result(&float_a.s, &float_b.s, floats);

    let op = quote! { self.get() + rhs.get() };

    impl_op_rhs(float_a, float_b, &output, &op, "Add", "add")
}

pub(crate) fn sub_result(
    spec_a: &FloatSpecifications,
    spec_b: &FloatSpecifications,
    floats: &[FloatDefinition],
) -> Option<FloatDefinition> {
    let can_sign_be_different = (spec_a.accept_negative && spec_b.accept_positive)
        || (spec_a.accept_positive && spec_b.accept_negative);
    let can_sign_be_same = (spec_a.accept_negative && spec_b.accept_negative)
        || (spec_a.accept_positive && spec_b.accept_positive);

    let can_overflow = can_sign_be_different;

    let can_sub_inf_and_inf = spec_a.accept_inf && spec_b.accept_inf && can_sign_be_same;

    let can_be_nan = can_sub_inf_and_inf;

    match can_be_nan {
        true => None,
        false => {
            let spec = FloatSpecifications {
                accept_inf: spec_a.accept_inf || spec_b.accept_inf || can_overflow,
                accept_zero: can_sign_be_same || (spec_a.accept_zero && spec_b.accept_zero),
                accept_positive: spec_a.accept_positive || spec_b.accept_negative,
                accept_negative: spec_a.accept_negative || spec_b.accept_positive,
            };
            find_float(&spec, floats)
        }
    }
}

pub(crate) fn impl_sub(
    float_a: &FloatDefinition,
    float_b: &FloatDefinition,
    floats: &[FloatDefinition],
) -> proc_macro2::TokenStream {
    let output = sub_result(&float_a.s, &float_b.s, floats);

    let op = quote! { self.get() - rhs.get() };

    impl_op_rhs(float_a, float_b, &output, &op, "Sub", "sub")
}

pub(crate) fn rem_result(
    spec_a: &FloatSpecifications,
    spec_b: &FloatSpecifications,
    floats: &[FloatDefinition],
) -> Option<FloatDefinition> {
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
}

pub(crate) fn impl_rem(
    float_a: &FloatDefinition,
    float_b: &FloatDefinition,
    floats: &[FloatDefinition],
) -> proc_macro2::TokenStream {
    let output = rem_result(&float_a.s, &float_b.s, floats);

    let op = quote! { self.get() % rhs.get() };

    impl_op_rhs(float_a, float_b, &output, &op, "Rem", "rem")
}

pub(crate) fn div_result(
    spec_a: &FloatSpecifications,
    spec_b: &FloatSpecifications,
    floats: &[FloatDefinition],
) -> Option<FloatDefinition> {
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
}

pub(crate) fn impl_div(
    float_a: &FloatDefinition,
    float_b: &FloatDefinition,
    floats: &[FloatDefinition],
) -> proc_macro2::TokenStream {
    let output = div_result(&float_a.s, &float_b.s, floats);

    let op = quote! { self.get() / rhs.get() };

    impl_op_rhs(float_a, float_b, &output, &op, "Div", "div")
}

pub(crate) fn mul_result(
    spec_a: &FloatSpecifications,
    spec_b: &FloatSpecifications,
    floats: &[FloatDefinition],
) -> Option<FloatDefinition> {
    let can_sign_be_different = (spec_a.accept_negative && spec_b.accept_positive)
        || (spec_a.accept_positive && spec_b.accept_negative);
    let can_sign_be_same = (spec_a.accept_negative && spec_b.accept_negative)
        || (spec_a.accept_positive && spec_b.accept_positive);

    let can_zero_multiply_inf =
        spec_a.accept_zero && spec_b.accept_inf || spec_a.accept_inf && spec_b.accept_zero;

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
}

pub(crate) fn impl_mul(
    float: &FloatDefinition,
    rhs: &FloatDefinition,
    floats: &[FloatDefinition],
) -> proc_macro2::TokenStream {
    let output = mul_result(&float.s, &rhs.s, floats);

    let op = quote! { self.get() * rhs.get() };

    impl_op_rhs(float, rhs, &output, &op, "Mul", "mul")
}
