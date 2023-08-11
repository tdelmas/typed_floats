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
pub fn generate_tests_self(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut output = proc_macro2::TokenStream::new();

    output.extend(gen_tests::generate_tests_self(F32));
    output.extend(gen_tests::generate_tests_self(F64));

    output.into()
}

#[proc_macro]
pub fn generate_tests_self_rhs(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut output = proc_macro2::TokenStream::new();

    output.extend(gen_tests::generate_tests_self_rhs(F32));
    output.extend(gen_tests::generate_tests_self_rhs(F64));

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
    let specifications = get_specifications();
    let floats_f64 = get_definitions("f64");
    let floats_f32 = get_definitions("f32");

    let mut output = proc_macro2::TokenStream::new();

    output.extend(do_generate_generic_floats(&specifications, "f64"));
    output.extend(do_generate_floats(&floats_f64));
    output.extend(do_generate_floats(&floats_f32));

    output.into()
}

fn do_generate_generic_floats(
    specifications: &[(&'static str, FloatSpecifications)],
    default_float_type: &str,
) -> proc_macro2::TokenStream {
    let mut output = proc_macro2::TokenStream::new();

    let default_float_type = syn::Ident::new(default_float_type, proc_macro2::Span::call_site());

    for (name, s) in specifications {
        let name = syn::Ident::new(name, proc_macro2::Span::call_site());

        let mut constraints = quote! {
            /// - It is not NaN.
        };

        if !s.accept_inf {
            constraints.extend(quote! {
                /// - It is not infinite.
            });
        }

        if !s.accept_zero {
            constraints.extend(quote! {
                /// - It is not zero.
            });
        }

        if !s.accept_positive {
            constraints.extend(quote! {
                /// - It is not positive.
            });
        }

        if !s.accept_negative {
            constraints.extend(quote! {
                /// - It is not negative.
            });
        }

        output.extend(quote! {
            /// A floating point number that satisfies the following constraints:
            #constraints
            #[derive(Debug, Copy, Clone)]
            #[repr(transparent)]
            pub struct #name<T=#default_float_type>(T);
        });
    }

    output
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
            impl TypedFloat for #full_type {
                type Content = #float_type;
                #[inline]
                #[must_use]
                unsafe fn new_unchecked(value: #float_type) -> Self {
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
                fn get(&self) -> #float_type {
                    self.0
                }

                #[inline]
                #[must_use]
                fn is_nan(&self) -> bool {
                    return false;
                }

                #[inline]
                #[must_use]
                fn is_infinite(&self) -> bool {
                    self.0.is_infinite()
                }

                #[inline]
                #[must_use]
                fn is_finite(&self) -> bool {
                    self.0.is_finite()
                }

                #[inline]
                #[must_use]
                fn is_subnormal(&self) -> bool {
                    self.0.is_subnormal()
                }

                #[inline]
                #[must_use]
                fn is_normal(&self) -> bool {
                    self.0.is_normal()
                }

                #[inline]
                #[must_use]
                fn classify(&self) -> core::num::FpCategory {
                    self.0.classify()
                }

                #[inline]
                #[must_use]
                fn is_sign_positive(&self) -> bool {
                    self.0.is_sign_positive()
                }

                #[inline]
                #[must_use]
                fn is_sign_negative(&self) -> bool {
                    self.0.is_sign_negative()
                }

                #[inline]
                #[must_use]
                fn is_negative_zero(&self) -> bool {
                    self.0 == 0.0 && self.0.is_sign_negative()
                }

                #[inline]
                #[must_use]
                fn is_positive_zero(&self) -> bool {
                    self.0 == 0.0 && self.0.is_sign_positive()
                }
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

            impl core::str::FromStr for #full_type {
                type Err = FromStrError;

                #[inline]
                #[must_use]
                fn from_str(s: &str) -> Result<Self, Self::Err> {
                    let f: #float_type = s.parse::<#float_type>().map_err(FromStrError::ParseFloatError)?;

                    Self::try_from(f).map_err(FromStrError::InvalidNumber)
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
