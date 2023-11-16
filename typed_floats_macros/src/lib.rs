//! Crate only used to generate the `typed_floats` crate.

#![warn(clippy::indexing_slicing)]
#![warn(clippy::nursery)]
#![warn(clippy::panic_in_result_fn)]
#![warn(clippy::panic)]
#![warn(clippy::pedantic)]
#![warn(clippy::unwrap_in_result)]
#![warn(clippy::unwrap_used)]
#![warn(missing_docs)]
#![warn(unsafe_op_in_unsafe_fn)]
#![warn(unused_crate_dependencies)]
#![forbid(unsafe_code)]

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
pub fn generate_tests_self(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let filter = input.to_string();

    let mut output = proc_macro2::TokenStream::new();

    output.extend(gen_tests::generate_tests_self(F32, &filter));
    output.extend(gen_tests::generate_tests_self(F64, &filter));

    output.into()
}

#[proc_macro]
pub fn generate_tests_self_rhs(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let filter = input.to_string();

    let mut output = proc_macro2::TokenStream::new();

    output.extend(gen_tests::generate_tests_self_rhs(F32, &filter));
    output.extend(gen_tests::generate_tests_self_rhs(F64, &filter));

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
pub fn generate_docs(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let floats_f64 = get_definitions("f64");
    let doc = generate_main_description(&floats_f64);
    let input = proc_macro2::TokenStream::from(input);

    let mut output = proc_macro2::TokenStream::new();
    output.extend(doc);
    output.extend(input);

    output.into()
}

#[proc_macro]
pub fn generate_floats(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let floats_f64 = get_definitions("f64");
    let floats_f32 = get_definitions("f32");

    let mut output = proc_macro2::TokenStream::new();

    output.extend(do_generate_floats(&floats_f64));
    output.extend(do_generate_floats(&floats_f32));

    output.into()
}

fn do_generate_floats(floats: &[FloatDefinition]) -> proc_macro2::TokenStream {
    let mut output = proc_macro2::TokenStream::new();

    let ops = get_impl_self();
    let ops_rhs = get_impl_self_rhs();

    for float in floats {
        let name = float.name_ident();
        let float_type = float.float_type_ident();
        let full_type = float.full_type_ident();

        output.extend(quote! {
            impl From<#name<Self>> for #float_type {
                #[inline]
                #[must_use]
                fn from(value: #name<Self>) -> Self {
                    value.0
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
        let float_type = float_a.float_type_ident();
        let a_full_type = &float_a.full_type_ident();

        output.extend(quote! {
            impl PartialEq<#a_full_type> for #float_type {
                #[inline]
                fn eq(&self, other: &#a_full_type) -> bool {
                    self == &other.0
                }
            }
            impl PartialEq<#float_type> for #a_full_type {
                #[inline]
                fn eq(&self, other: &#float_type) -> bool {
                    &self.0 == other
                }
            }
        });

        for float_b in floats {
            if float_a.name != float_b.name {
                let b_full_type = &float_b.full_type_ident();

                output.extend(quote! {
                    impl PartialEq<#a_full_type> for #b_full_type {
                        #[inline]
                        fn eq(&self, other: &#a_full_type) -> bool {
                            self.0 == other.0
                        }
                    }
                });
            }
        }
    }

    for float_a in floats {
        for op in &ops {
            output.extend(op.get_impl(float_a, floats));
        }

        for float_b in floats {
            for op in &ops_rhs {
                output.extend(op.get_impl(float_a, float_b, floats));
            }
        }
    }

    output
}
