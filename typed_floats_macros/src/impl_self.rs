use proc_macro2::Span;
use quote::quote;
use syn::Ident;

use crate::types::*;

fn impl_fn(
    float: &FloatDefinition,
    output: &Option<FloatDefinition>,
    op: &proc_macro2::TokenStream,
    fn_name: &str,
) -> proc_macro2::TokenStream {
    let float_full_type = &float.full_type_ident();

    let return_value = match output {
        Some(d) => {
            let output_call = &d.call_tokens();

            quote! {
                unsafe { #output_call::new_unchecked(#op) }
            }
        }
        None => {
            quote! { #op }
        }
    };

    let output_name = output_name(output, &float.float_type_ident());

    let fn_ident = Ident::new(fn_name, Span::call_site());

    quote! {
        impl #float_full_type {
            #[inline]
            #[must_use]
            fn #fn_ident(self) -> #output_name {
                #return_value
            }
        }
    }
}

pub(crate) fn neg_result(
    float: &FloatDefinition,
    floats: &[FloatDefinition],
) -> Option<FloatDefinition> {
    let mut output_spec = float.s.clone();

    if !output_spec.accept_positive {
        output_spec.accept_positive = true;
        output_spec.accept_negative = false;
    } else if !output_spec.accept_negative {
        output_spec.accept_positive = false;
        output_spec.accept_negative = true;
    }

    find_float(&output_spec, floats)
}

pub(crate) fn impl_neg(
    float: &FloatDefinition,
    floats: &[FloatDefinition],
) -> proc_macro2::TokenStream {
    let full_type = &float.full_type_ident();

    let output = neg_result(float, floats);
    let output_name = output_name(&output, &float.float_type_ident());

    quote! {
        impl core::ops::Neg for #full_type {
            type Output = #output_name;

            #[inline]
            fn neg(self) -> Self::Output {
                unsafe { Self::Output::new_unchecked(-self.get()) }
            }
        }
    }
}

pub(crate) fn floor_result(
    float: &FloatDefinition,
    floats: &[FloatDefinition],
) -> Option<FloatDefinition> {
    if float.s.accept_positive {
        let mut output_spec = float.s.clone();
        output_spec.accept_zero = true;

        find_float(&output_spec, floats)
    } else {
        Some(float.clone())
    }
}

pub(crate) fn impl_floor(
    float: &FloatDefinition,
    floats: &[FloatDefinition],
) -> proc_macro2::TokenStream {
    let output = floor_result(float, floats);

    let op = quote! { self.get().floor() };

    impl_fn(float, &output, &op, "floor")
}

pub(crate) fn ceil_result(
    float: &FloatDefinition,
    floats: &[FloatDefinition],
) -> Option<FloatDefinition> {
    if float.s.accept_negative {
        let mut output_spec = float.s.clone();
        output_spec.accept_zero = true;

        find_float(&output_spec, floats)
    } else {
        Some(float.clone())
    }
}

pub(crate) fn impl_ceil(
    float: &FloatDefinition,
    floats: &[FloatDefinition],
) -> proc_macro2::TokenStream {
    let output = ceil_result(float, floats);

    let op = quote! { self.get().ceil() };

    impl_fn(float, &output, &op, "ceil")
}

pub(crate) fn round_result(
    float: &FloatDefinition,
    floats: &[FloatDefinition],
) -> Option<FloatDefinition> {
    let mut output_spec = float.s.clone();
    output_spec.accept_zero = true;

    find_float(&output_spec, floats)
}

pub(crate) fn impl_round(
    float: &FloatDefinition,
    floats: &[FloatDefinition],
) -> proc_macro2::TokenStream {
    let output = round_result(float, floats);

    let op = quote! { self.get().round() };

    impl_fn(float, &output, &op, "round")
}

pub(crate) fn abs_result(
    float: &FloatDefinition,
    floats: &[FloatDefinition],
) -> Option<FloatDefinition> {
    let mut output_spec = float.s.clone();

    output_spec.accept_positive = true;
    output_spec.accept_negative = false;

    find_float(&output_spec, floats)
}

pub(crate) fn impl_abs(
    float: &FloatDefinition,
    floats: &[FloatDefinition],
) -> proc_macro2::TokenStream {
    let full_type = &float.full_type_ident();

    let output = abs_result(float, floats).unwrap();
    let output_call = output.call_tokens();
    let output_type = output.full_type_ident();

    if !float.s.accept_negative {
        // no-op
        quote! {
            impl #full_type {
                #[inline]
                pub fn abs(self) -> Self {
                    unsafe {Self::new_unchecked(self.get()) }
                }
            }
        }
    } else if !float.s.accept_positive {
        // inv
        quote! {
            impl #full_type {
                #[inline]
                pub fn abs(self) -> #output_type {
                    unsafe { #output_call::new_unchecked(-self.get()) }
                }
            }
        }
    } else {
        quote! {
            impl #full_type {
                #[inline]
                pub fn abs(self) -> #output_type {
                    unsafe { #output_call::new_unchecked(self.get().abs()) }
                }
            }
        }
    }
}
