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

pub(crate) fn impl_neg(
    float: &FloatDefinition,
    floats: &[FloatDefinition],
) -> proc_macro2::TokenStream {
    let full_type = &float.full_type_ident();

    let mut output_def = float.s.clone();

    if float.s.accept_negative {
        output_def.accept_positive = true;
    }

    if float.s.accept_positive {
        output_def.accept_negative = true;
    }

    let output = find_float(&output_def, floats);
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

pub(crate) fn floor_result(float: &FloatDefinition, floats: &[FloatDefinition]) -> FloatDefinition {
    if float.s.accept_positive {
        let mut output_spec = float.s.clone();
        output_spec.accept_zero = true;

        find_float(&output_spec, floats).unwrap()
    } else {
        float.clone()
    }
}

pub(crate) fn impl_floor(
    float: &FloatDefinition,
    floats: &[FloatDefinition],
) -> proc_macro2::TokenStream {
    let output = floor_result(&float, floats);

    let op = quote! { self.get().floor() };

    impl_fn(float, &Some(output), &op, "floor")
}

pub(crate) fn ceil_result(float: &FloatDefinition, floats: &[FloatDefinition]) -> FloatDefinition {
    if float.s.accept_negative {
        let mut output_spec = float.s.clone();
        output_spec.accept_zero = true;

        find_float(&output_spec, floats).unwrap()
    } else {
        float.clone()
    }
}

pub(crate) fn impl_ceil(
    float: &FloatDefinition,
    floats: &[FloatDefinition],
) -> proc_macro2::TokenStream {
    let output = ceil_result(&float, floats);

    let op = quote! { self.get().ceil() };

    impl_fn(float, &Some(output), &op, "ceil")
}

pub(crate) fn round_result(float: &FloatDefinition, floats: &[FloatDefinition]) -> FloatDefinition {
    let mut output_spec = float.s.clone();
    output_spec.accept_zero = true;

    find_float(&output_spec, floats).unwrap()
}

pub(crate) fn impl_round(
    float: &FloatDefinition,
    floats: &[FloatDefinition],
) -> proc_macro2::TokenStream {
    let output = round_result(&float, floats);

    let op = quote! { self.get().round() };

    impl_fn(float, &Some(output), &op, "round")
}

pub(crate) fn impl_abs(
    float: &FloatDefinition,
    floats: &[FloatDefinition],
) -> proc_macro2::TokenStream {
    let full_type = &float.full_type_ident();

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
        let mut output_spec = float.s.clone();
        output_spec.accept_negative = false;
        output_spec.accept_positive = true;

        let output = find_float(&output_spec, floats).unwrap();

        let output_type = output.full_type_ident();
        let output_call = output.call_tokens();

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
        let mut output_spec = float.s.clone();
        output_spec.accept_negative = false;

        let output = find_float(&output_spec, floats).unwrap();

        let output_type = output.full_type_ident();
        let output_call = output.call_tokens();

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
