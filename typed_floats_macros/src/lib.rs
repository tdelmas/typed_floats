//! Crate only used to generate the `typed_floats` crate.

extern crate proc_macro;

use quote::quote;

mod try_from;
use try_from::*;

mod types;
use types::*;

mod impl_self;
use impl_self::*;

mod impl_self_rhs;
use impl_self_rhs::*;

mod add_doc;
use add_doc::*;

mod gen_tests;

static F32: &str = "f32";
static F64: &str = "f64";

#[proc_macro]
pub fn generate_tests(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut output = proc_macro2::TokenStream::new();

    output.extend(gen_tests::generate_tests(F32));
    output.extend(gen_tests::generate_tests(F64));

    output.into()
}

fn get_specifications() -> Vec<(&'static str, FloatSpecifications)> {
    vec![
        (
            "NonNaN",
            FloatSpecifications {
                accept_inf: true,
                accept_zero: true,
                accept_positive: true,
                accept_negative: true,
            },
        ),
        (
            "NonZeroNonNaN",
            FloatSpecifications {
                accept_inf: true,
                accept_zero: false,
                accept_positive: true,
                accept_negative: true,
            },
        ),
        (
            "NonNaNFinite",
            FloatSpecifications {
                accept_inf: false,
                accept_zero: true,
                accept_positive: true,
                accept_negative: true,
            },
        ),
        (
            "NonZeroNonNaNFinite",
            FloatSpecifications {
                accept_inf: false,
                accept_zero: false,
                accept_positive: true,
                accept_negative: true,
            },
        ),
        (
            "Positive",
            FloatSpecifications {
                accept_inf: true,
                accept_zero: true,
                accept_positive: true,
                accept_negative: false,
            },
        ),
        (
            "Negative",
            FloatSpecifications {
                accept_inf: true,
                accept_zero: true,
                accept_positive: false,
                accept_negative: true,
            },
        ),
        (
            "PositiveFinite",
            FloatSpecifications {
                accept_inf: false,
                accept_zero: true,
                accept_positive: true,
                accept_negative: false,
            },
        ),
        (
            "NegativeFinite",
            FloatSpecifications {
                accept_inf: false,
                accept_zero: true,
                accept_positive: false,
                accept_negative: true,
            },
        ),
        (
            "StrictlyPositive",
            FloatSpecifications {
                accept_inf: true,
                accept_zero: false,
                accept_positive: true,
                accept_negative: false,
            },
        ),
        (
            "StrictlyNegative",
            FloatSpecifications {
                accept_inf: true,
                accept_zero: false,
                accept_positive: false,
                accept_negative: true,
            },
        ),
        (
            "StrictlyPositiveFinite",
            FloatSpecifications {
                accept_inf: false,
                accept_zero: false,
                accept_positive: true,
                accept_negative: false,
            },
        ),
        (
            "StrictlyNegativeFinite",
            FloatSpecifications {
                accept_inf: false,
                accept_zero: false,
                accept_positive: false,
                accept_negative: true,
            },
        ),
    ]
}

fn get_definitions(float_type: &'static str) -> Vec<FloatDefinition> {
    let specifications = get_specifications();

    specifications
        .iter()
        .map(|specification| FloatDefinition {
            name: specification.0,
            float_type,
            s: specification.1.clone(),
        })
        .collect::<Vec<_>>()
}

#[proc_macro]
pub fn generate_floats(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let floats_f64 = get_definitions("f64");
    let floats_f32 = get_definitions("f32");

    let mut output = proc_macro2::TokenStream::new();

    output.extend(generate_main_description(&floats_f64));

    output.extend(do_generate_floats(&floats_f64, true));
    output.extend(do_generate_floats(&floats_f32, false));

    output.into()
}

fn do_generate_floats(floats: &[FloatDefinition], with_generic: bool) -> proc_macro2::TokenStream {
    let mut output = proc_macro2::TokenStream::new();

    let neg = get_neg();
    let floor = get_floor();
    let ceil = get_ceil();
    let round = get_round();
    let abs = get_abs();

    let add = get_add();
    let sub = get_sub();
    let mul = get_mul();
    let rem = get_rem();
    let div = get_div();

    for float in floats {
        let name = float.name_ident();
        let float_type = float.float_type_ident();
        let full_type = float.full_type_ident();

        if with_generic {
            output.extend(quote! {
                #[derive(Debug, Copy, Clone)]
                pub struct #name<T: Sized=#float_type>(T);
            });
        }

        output.extend(quote! {
            impl #full_type {
                /// # Safety
                /// The caller must ensure that the value is valid
                /// It will panic in debug mode if the value is not valid
                /// but in release mode the behavior is undefined
                #[inline]
                #[must_use]
                pub unsafe fn new_unchecked(value: #float_type) -> Self {
                    debug_assert!(
                        Self::try_from(value).is_ok(),
                        "{value} is not a valid {name}",
                        value = value,
                        name = stringify!(#name)
                    );

                    Self(value)
                }

                #[inline]
                #[must_use]
                pub fn new(value: #float_type) -> Result<Self, InvalidNumber> {
                    Self::try_from(value)
                }

                #[inline]
                #[must_use]
                pub const fn get(self) -> #float_type {
                    self.0
                }
            }

            impl Float for #full_type {
                type Content = #float_type;
            }

            impl PartialEq for #full_type {
                fn eq(&self, other: &Self) -> bool {
                    self.0 == other.0
                }
            }

            impl Eq for #full_type {
                // This is safe because we know that both values are not NaN
            }

            impl Ord for #full_type {
                #[inline]
                fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                    // This is safe because we know that both values are not NaN
                    self.0.partial_cmp(&other.0).unwrap()
                }
            }

            impl PartialOrd for #full_type {
                #[inline]
                fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                    self.0.partial_cmp(&other.0)
                }
            }

            impl From<#name<Self>> for #float_type {
                #[inline]
                #[must_use]
                fn from(value: #name<Self>) -> Self {
                    value.0
                }
            }

            impl core::fmt::Display for #full_type {
                #[inline]
                #[must_use]
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    write!(f, "{}", self.0)
                }
            }
        });

        output.extend(generate_try_from_float(float));
        output.extend(generate_try_ints(float));
    }

    for float_a in floats {
        for float_b in floats {
            if float_a.name != float_b.name {
                output.extend(impl_from_or_try_from(float_a, float_b));
            }
        }
    }

    for float_a in floats {
        output.extend(neg.get_impl(float_a, floats));
        output.extend(floor.get_impl(float_a, floats));
        output.extend(ceil.get_impl(float_a, floats));
        output.extend(round.get_impl(float_a, floats));
        output.extend(abs.get_impl(float_a, floats));

        for float_b in floats {
            output.extend(add.get_impl(float_a, float_b, floats));
            output.extend(sub.get_impl(float_a, float_b, floats));
            output.extend(mul.get_impl(float_a, float_b, floats));
            output.extend(div.get_impl(float_a, float_b, floats));
            output.extend(rem.get_impl(float_a, float_b, floats));
        }
    }

    output
}
