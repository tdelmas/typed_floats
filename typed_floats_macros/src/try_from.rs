use quote::quote;

use crate::types::FloatDefinition;

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

pub fn impl_from_or_try_from(
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
