use proc_macro2::Span;
use quote::quote;
use syn::Ident;

use crate::types::*;

fn impl_from(float_from: &FloatDefinition, float_to: &FloatDefinition) -> proc_macro2::TokenStream {
    let from_full_type = &float_from.full_type_ident();
    let to_full_type = &float_to.full_type_ident();

    quote! {
        impl core::convert::From<#from_full_type> for #to_full_type {
            #[inline]
            #[must_use]
            fn from(value: #from_full_type) -> Self {
                unsafe { Self::new_unchecked(value.get()) }
            }
        }
    }
}

fn impl_try_from(
    float_from: &FloatDefinition,
    float_to: &FloatDefinition,
) -> proc_macro2::TokenStream {
    let from_full_type = &float_from.full_type_ident();
    let to_full_type = &float_to.full_type_ident();

    quote! {
        impl core::convert::TryFrom<#from_full_type> for #to_full_type {
            type Error = InvalidNumber;

            #[inline]
            #[must_use]
            fn try_from(value: #from_full_type) -> Result<Self, Self::Error> {
                Self::try_from(value.get())
            }
        }
    }
}

pub(crate) fn impl_from_or_try_from(
    float_from: &FloatDefinition,
    float_to: &FloatDefinition,
) -> proc_macro2::TokenStream {
    let from = &float_from.s;
    let to = &float_to.s;

    if (!from.accept_positive && to.accept_negative)
        && (!from.accept_negative && to.accept_positive)
    {
        // No conversion between positive and negative
        proc_macro2::TokenStream::new()
    } else if from.can_fit_into(to) {
        impl_from(float_from, float_to)
    } else {
        impl_try_from(float_from, float_to)
    }
}

pub(crate) fn generate_try_ints(float: &FloatDefinition) -> proc_macro2::TokenStream {
    let mut try_from_ints = proc_macro2::TokenStream::new();

    // https://doc.rust-lang.org/1.49.0/reference/expressions/operator-expr.html#type-cast-expressions

    // with the current set of numeric types, overflow can only happen on u128 as f32

    let types = vec!["u8", "u16", "u32", "u64"];
    for int_type in types {
        try_from_ints.extend(generate_try_int(float, int_type, false));
    }

    let types = vec!["i8", "i16", "i32", "i64"];
    for int_type in types {
        try_from_ints.extend(generate_try_int(float, int_type, true));
    }

    let types = vec!["NonZeroU8", "NonZeroU16", "NonZeroU32", "NonZeroU64"];
    for int_type in types {
        try_from_ints.extend(generate_try_int_nonzero(float, int_type, false));
    }

    let types = vec!["NonZeroI8", "NonZeroI16", "NonZeroI32", "NonZeroI64"];
    for int_type in types {
        try_from_ints.extend(generate_try_int_nonzero(float, int_type, true));
    }

    try_from_ints
}

pub(crate) fn generate_try_int(
    float: &FloatDefinition,
    int_type: &str,
    can_be_negative: bool,
) -> proc_macro2::TokenStream {
    let int_type: Ident = Ident::new(int_type, Span::call_site());

    let float_type = &float.float_type_ident();
    let full_type = &float.full_type_ident();
    let call_tokens = &float.call_tokens();

    if can_be_negative && !float.s.accept_negative {
        quote! {
            impl TryFrom<#int_type> for #full_type {
                type Error = InvalidNumber;

                #[inline]
                #[must_use]
                fn try_from(value: #int_type) -> Result<Self, Self::Error> {
                    if value == 0 {
                        Err(InvalidNumber::Zero)
                    } else {
                        unsafe { Ok(#call_tokens::new_unchecked(value as #float_type )) }
                    }
                }
            }
        }
    } else {
        quote! {
            impl From<#int_type> for #full_type {
                #[inline]
                #[must_use]
                fn from(value: #int_type) -> Self {
                    unsafe { #call_tokens::new_unchecked(value as #float_type ) }
                }
            }
        }
    }
}

pub(crate) fn generate_try_int_nonzero(
    float: &FloatDefinition,
    int_type: &str,
    can_be_negative: bool,
) -> proc_macro2::TokenStream {
    let int_type: Ident = Ident::new(int_type, Span::call_site());

    let float_type = &float.float_type_ident();
    let full_type = &float.full_type_ident();
    let call_tokens = &float.call_tokens();

    if can_be_negative && !float.s.accept_negative {
        quote! {
            impl TryFrom<#int_type> for #full_type {
                type Error = InvalidNumber;

                #[inline]
                #[must_use]
                fn try_from(value: #int_type) -> Result<Self, Self::Error> {
                    unsafe { Ok(#call_tokens::new_unchecked(value.get() as #float_type)) }
                }
            }
        }
    } else {
        quote! {
            impl From<#int_type> for #full_type {
                #[inline]
                #[must_use]
                fn from(value: #int_type) -> Self {
                    unsafe {#call_tokens::new_unchecked(value.get() as #float_type )}
                }
            }
        }
    }
}
