//! Crate only used to generate the `typed_floats` crate.

extern crate proc_macro;

use quote::quote;

mod try_from;
use try_from::impl_from_or_try_from;

mod types;
use types::{FloatDefinition, FloatSpecifications, ReturnTypeDefinition};

mod impl_self;
use impl_self::get_impl_self;

mod impl_self_rhs;
use impl_self_rhs::get_impl_self_rhs;

mod add_doc;
use add_doc::generate_main_description;

mod gen_tests;

static F32: &str = "f32";
static F64: &str = "f64";

const NON_NAN: (&str, FloatSpecifications) = (
    "NonNaN",
    FloatSpecifications {
        accept_inf: true,
        accept_zero: true,
        accept_positive: true,
        accept_negative: true,
    },
);

const NON_ZERO_NON_NAN: (&str, FloatSpecifications) = (
    "NonZeroNonNaN",
    FloatSpecifications {
        accept_inf: true,
        accept_zero: false,
        accept_positive: true,
        accept_negative: true,
    },
);

const NON_NAN_FINITE: (&str, FloatSpecifications) = (
    "NonNaNFinite",
    FloatSpecifications {
        accept_inf: false,
        accept_zero: true,
        accept_positive: true,
        accept_negative: true,
    },
);

const NON_ZERO_NON_NAN_FINITE: (&str, FloatSpecifications) = (
    "NonZeroNonNaNFinite",
    FloatSpecifications {
        accept_inf: false,
        accept_zero: false,
        accept_positive: true,
        accept_negative: true,
    },
);

const POSITIVE: (&str, FloatSpecifications) = (
    "Positive",
    FloatSpecifications {
        accept_inf: true,
        accept_zero: true,
        accept_positive: true,
        accept_negative: false,
    },
);

const NEGATIVE: (&str, FloatSpecifications) = (
    "Negative",
    FloatSpecifications {
        accept_inf: true,
        accept_zero: true,
        accept_positive: false,
        accept_negative: true,
    },
);

const POSITIVE_FINITE: (&str, FloatSpecifications) = (
    "PositiveFinite",
    FloatSpecifications {
        accept_inf: false,
        accept_zero: true,
        accept_positive: true,
        accept_negative: false,
    },
);

const NEGATIVE_FINITE: (&str, FloatSpecifications) = (
    "NegativeFinite",
    FloatSpecifications {
        accept_inf: false,
        accept_zero: true,
        accept_positive: false,
        accept_negative: true,
    },
);

const STRICTLY_POSITIVE: (&str, FloatSpecifications) = (
    "StrictlyPositive",
    FloatSpecifications {
        accept_inf: true,
        accept_zero: false,
        accept_positive: true,
        accept_negative: false,
    },
);

const STRICTLY_NEGATIVE: (&str, FloatSpecifications) = (
    "StrictlyNegative",
    FloatSpecifications {
        accept_inf: true,
        accept_zero: false,
        accept_positive: false,
        accept_negative: true,
    },
);

const STRICTLY_POSITIVE_FINITE: (&str, FloatSpecifications) = (
    "StrictlyPositiveFinite",
    FloatSpecifications {
        accept_inf: false,
        accept_zero: false,
        accept_positive: true,
        accept_negative: false,
    },
);

const STRICTLY_NEGATIVE_FINITE: (&str, FloatSpecifications) = (
    "StrictlyNegativeFinite",
    FloatSpecifications {
        accept_inf: false,
        accept_zero: false,
        accept_positive: false,
        accept_negative: true,
    },
);

const TYPES: &[(&str, FloatSpecifications)] = &[
    NON_NAN,
    NON_ZERO_NON_NAN,
    NON_NAN_FINITE,
    NON_ZERO_NON_NAN_FINITE,
    POSITIVE,
    NEGATIVE,
    POSITIVE_FINITE,
    NEGATIVE_FINITE,
    STRICTLY_POSITIVE,
    STRICTLY_NEGATIVE,
    STRICTLY_POSITIVE_FINITE,
    STRICTLY_NEGATIVE_FINITE,
];

/// Generate the tests for unary operations.
#[proc_macro]
pub fn generate_tests_self(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let filter = input.to_string();

    let mut output = proc_macro2::TokenStream::new();

    output.extend(gen_tests::generate_tests_self(F32, &filter));
    output.extend(gen_tests::generate_tests_self(F64, &filter));

    output.into()
}

/// Generate the tests for binary operations.
#[proc_macro]
pub fn generate_tests_self_rhs(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let filter = input.to_string();

    let mut output = proc_macro2::TokenStream::new();

    output.extend(gen_tests::generate_tests_self_rhs(F32, &filter));
    output.extend(gen_tests::generate_tests_self_rhs(F64, &filter));

    output.into()
}

/// Return the `FloatDefinition` for the given type
fn get_definitions(float_type: &'static str) -> [FloatDefinition; 12] {
    TYPES
        .iter()
        .map(|specification| FloatDefinition {
            name: specification.0,
            float_type,
            s: specification.1.clone(),
        })
        .collect::<Vec<FloatDefinition>>()
        .try_into()
        .expect("Failed to convert to array")
}

/// Generate the documentation
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

/// Generate the `PartialEq`, `From` and `TryFrom` implementations.
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

    for float_a in floats {
        for float_b in floats {
            if float_a.name != float_b.name {
                output.extend(impl_from_or_try_from(float_a, float_b));
            }
        }
    }

    for float_a in floats {
        let a_full_type = &float_a.full_type_ident();

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

                    impl PartialOrd<#a_full_type> for #b_full_type {
                        #[inline]
                        fn partial_cmp(&self, other: &#a_full_type) -> Option<core::cmp::Ordering> {
                            Some(if self.get() < other.get() {
                                core::cmp::Ordering::Less
                            } else if self == other {
                                core::cmp::Ordering::Equal
                            } else {
                                core::cmp::Ordering::Greater
                            })
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
