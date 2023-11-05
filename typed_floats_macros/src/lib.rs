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

fn get_specifications() -> Vec<(&'static str, &'static str, FloatSpecifications)> {
    vec![
        (
            "NonNaN",
            "A non-NaN floating point number",
            FloatSpecifications {
                accept_inf: true,
                accept_zero: true,
                accept_positive: true,
                accept_negative: true,
            },
        ),
        (
            "NonZeroNonNaN",
            "A non-NaN floating point number different from zero",
            FloatSpecifications {
                accept_inf: true,
                accept_zero: false,
                accept_positive: true,
                accept_negative: true,
            },
        ),
        (
            "NonNaNFinite",
            "A non-NaN finite floating point number",
            FloatSpecifications {
                accept_inf: false,
                accept_zero: true,
                accept_positive: true,
                accept_negative: true,
            },
        ),
        (
            "NonZeroNonNaNFinite",
            "A non-NaN finite floating point number different from zero",
            FloatSpecifications {
                accept_inf: false,
                accept_zero: false,
                accept_positive: true,
                accept_negative: true,
            },
        ),
        (
            "Positive",
            "A non-NaN positive floating point number",
            FloatSpecifications {
                accept_inf: true,
                accept_zero: true,
                accept_positive: true,
                accept_negative: false,
            },
        ),
        (
            "Negative",
            "A non-NaN negative floating point number",
            FloatSpecifications {
                accept_inf: true,
                accept_zero: true,
                accept_positive: false,
                accept_negative: true,
            },
        ),
        (
            "PositiveFinite",
            "A non-NaN positive finite floating point number",
            FloatSpecifications {
                accept_inf: false,
                accept_zero: true,
                accept_positive: true,
                accept_negative: false,
            },
        ),
        (
            "NegativeFinite",
            "A non-NaN negative finite floating point number",
            FloatSpecifications {
                accept_inf: false,
                accept_zero: true,
                accept_positive: false,
                accept_negative: true,
            },
        ),
        (
            "StrictlyPositive",
            "A non-NaN strictly positive floating point number",
            FloatSpecifications {
                accept_inf: true,
                accept_zero: false,
                accept_positive: true,
                accept_negative: false,
            },
        ),
        (
            "StrictlyNegative",
            "A non-NaN strictly negative floating point number",
            FloatSpecifications {
                accept_inf: true,
                accept_zero: false,
                accept_positive: false,
                accept_negative: true,
            },
        ),
        (
            "StrictlyPositiveFinite",
            "A non-NaN strictly positive finite floating point number",
            FloatSpecifications {
                accept_inf: false,
                accept_zero: false,
                accept_positive: true,
                accept_negative: false,
            },
        ),
        (
            "StrictlyNegativeFinite",
            "A non-NaN strictly negative finite floating point number",
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
            s: specification.2.clone(),
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
    specifications: &[(&'static str, &'static str, FloatSpecifications)],
    default_float_type: &str,
) -> proc_macro2::TokenStream {
    let mut output = proc_macro2::TokenStream::new();

    let default_float_type = syn::Ident::new(default_float_type, proc_macro2::Span::call_site());

    for (name, description, s) in specifications {
        let name = syn::Ident::new(name, proc_macro2::Span::call_site());
        let description = add_doc::comment_line(description);

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
            #description
            ///
            /// It satisfies the following constraints:
            #constraints
            #[cfg_attr(feature = "serde", derive(Serialize))]
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

        let compiler_hints = float
            .s
            .get_compiler_hints(&syn::Ident::new("value", proc_macro2::Span::call_site()));

        output.extend(quote! {
            impl #full_type {
                /// Creates a new value from a primitive type
                /// It adds a little overhead compared to `new_unchecked`
                /// because it checks that the value is valid
                ///
                /// # Examples
                ///
                /// ```
                /// # use typed_floats::tf64::NonNaN;
                /// let x = NonNaN::new(3.0).unwrap();
                ///
                /// assert_eq!(x, 3.0);
                /// ```
                ///
                /// # Errors
                /// Returns an error if the value is not valid
                #[inline]
                pub fn new(value: #float_type) -> Result<Self, InvalidNumber> {
                    Self::try_from(value)
                }

                /// Creates a new value from a primitive type with zero overhead (in release mode).
                /// It is up to the caller to ensure that the value is valid
                ///
                /// # Examples
                ///
                /// ```
                /// # use typed_floats::tf64::NonNaN;
                /// let x = unsafe { NonNaN::new_unchecked(3.0) };
                ///
                /// assert_eq!(x, 3.0);
                /// ```
                /// # Safety
                /// The caller must ensure that the value is valid.
                /// It will panic in debug mode if the value is not valid,
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

                    #compiler_hints

                    Self(value)
                }

                /// Returns the value as a primitive type
                /// 
                /// # Examples
                /// 
                /// ```
                /// use typed_floats::tf64::NonNaN;
                /// 
                /// let x = NonNaN::new(3.0).unwrap();
                /// 
                /// let y: f64 = x.into();
                /// 
                /// assert_eq!(y, 3.0);
                /// ```
                #[inline]
                #[must_use]
                pub fn get(&self) -> #float_type {
                    self.0
                }

                /// Returns `true` if this value is NaN.
                /// This is never the case for the provided types
                ///
                /// # Examples
                ///
                /// ```
                /// # use typed_floats::*;
                /// let x: NonNaN = 3.0.try_into().unwrap();
                ///
                /// assert_eq!(x.is_nan(), false);
                /// ```
                ///
                /// See [`f64::is_nan()`] for more details.
                #[inline]
                #[must_use]
                pub fn is_nan(&self) -> bool {
                    return false;
                }

                /// Returns `true` if this value is positive infinity or negative infinity.
                ///
                /// # Examples
                ///
                /// ```
                /// # use typed_floats::*;
                /// let x: NonNaN = 3.0.try_into().unwrap();
                ///
                /// assert_eq!(x.is_infinite(), false);
                /// ```
                ///
                /// See [`f64::is_infinite()`] for more details.
                #[inline]
                #[must_use]
                pub fn is_infinite(&self) -> bool {
                    self.0.is_infinite()
                }

                /// Returns `true` if this number is positive infinity nor negative infinity.
                ///
                /// # Examples
                ///
                /// ```
                /// # use typed_floats::*;
                /// let x: NonNaN = 3.0.try_into().unwrap();
                ///
                /// assert_eq!(x.is_finite(), true);
                /// ```
                ///
                /// See [`f64::is_finite()`] for more details.
                #[inline]
                #[must_use]
                pub fn is_finite(&self) -> bool {
                    self.0.is_finite()
                }

                /// Returns `true` if the number is [subnormal](https://en.wikipedia.org/wiki/Denormal_number).
                ///
                /// # Examples
                ///
                /// ```
                /// # use typed_floats::*;
                /// let x: NonNaN = 3.0.try_into().unwrap();
                ///
                /// assert_eq!(x.is_subnormal(), false);
                /// ```
                ///
                /// See [`f64::is_subnormal()`] for more details.
                #[inline]
                #[must_use]
                pub fn is_subnormal(&self) -> bool {
                    self.0.is_subnormal()
                }

                /// Returns `true` if the number is neither zero, infinite or [subnormal](https://en.wikipedia.org/wiki/Denormal_number).
                ///
                /// # Examples
                ///
                /// ```
                /// # use typed_floats::*;
                /// let x: NonNaN = 3.0.try_into().unwrap();
                ///
                /// assert_eq!(x.is_normal(), true);
                /// ```
                ///
                /// See [`f64::is_normal()`] for more details.
                #[inline]
                #[must_use]
                pub fn is_normal(&self) -> bool {
                    self.0.is_normal()
                }

                /// Returns the floating point category of the number. If only one property
                /// is going to be tested, it is generally faster to use the specific
                /// predicate instead.
                ///
                /// # Examples
                ///
                /// ```
                /// # use typed_floats::*;
                /// let x: NonNaN = 3.0.try_into().unwrap();
                ///
                /// assert_eq!(x.classify(), core::num::FpCategory::Normal);
                /// ```
                ///
                /// See [`f64::classify()`] for more details.
                #[inline]
                #[must_use]
                pub fn classify(&self) -> core::num::FpCategory {
                    self.0.classify()
                }

                /// Returns `true` if `self` has a positive sign, including `+0.0` and positive infinity.
                ///
                /// # Examples
                ///
                /// ```
                /// # use typed_floats::*;
                /// let x: NonNaN = 3.0.try_into().unwrap();
                ///
                /// assert_eq!(x.is_sign_positive(), true);
                /// ```
                ///
                /// See [`f64::is_sign_positive()`] for more details.
                #[inline]
                #[must_use]
                pub fn is_sign_positive(&self) -> bool {
                    self.0.is_sign_positive()
                }

                /// Returns `true` if `self` has a negative sign, including `-0.0` and negative infinity.
                ///
                /// # Examples
                ///
                /// ```
                /// # use typed_floats::*;
                /// let x: NonNaN = 3.0.try_into().unwrap();
                ///
                /// assert_eq!(x.is_sign_negative(), false);
                /// ```
                ///
                /// See [`f64::is_sign_negative()`] for more details.
                #[inline]
                #[must_use]
                pub fn is_sign_negative(&self) -> bool {
                    self.0.is_sign_negative()
                }

                /// Returns `true` if the number is negative zero.
                ///     
                /// # Examples
                ///
                /// ```
                /// # use typed_floats::*;
                /// let x: NonNaN = 3.0.try_into().unwrap();
                /// let y: NonNaN = (-0.0).try_into().unwrap();
                /// let z: NonNaN = (0.0).try_into().unwrap();
                ///
                /// assert_eq!(x.is_negative_zero(), false);
                /// assert_eq!(y.is_negative_zero(), true);
                /// assert_eq!(z.is_negative_zero(), false);
                /// ```
                #[inline]
                #[must_use]
                pub fn is_negative_zero(&self) -> bool {
                    self.0 == 0.0 && self.0.is_sign_negative()
                }

                /// Returns `true` if the number is positive zero.
                ///
                /// # Examples
                ///
                /// ```
                /// # use typed_floats::*;
                /// let x: NonNaN = 3.0.try_into().unwrap();
                /// let y: NonNaN = (-0.0).try_into().unwrap();
                /// let z: NonNaN = (0.0).try_into().unwrap();
                ///
                /// assert_eq!(x.is_positive_zero(), false);
                /// assert_eq!(y.is_positive_zero(), false);
                /// assert_eq!(z.is_positive_zero(), true);
                /// ```
                #[inline]
                #[must_use]
                pub fn is_positive_zero(&self) -> bool {
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
                    match self.0.partial_cmp(&other.0) {
                        Some(ordering) => ordering,
                        // This is safe because we know that both values are not NaN
                        None => unsafe { core::hint::unreachable_unchecked() },
                    }
                }
            }

            impl PartialOrd for #full_type {
                #[inline]
                fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                    Some(self.cmp(other))
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

        // If the type doesn't accept simultaneously `+0.0` and `-0.0` we can implement `Hash`
        // > When implementing both Hash and Eq, it is important that the following property holds:
        // > `k1 == k2 -> hash(k1) == hash(k2)`
        // This is sound because `NaN` is not a possible value.
        // https://doc.rust-lang.org/std/hash/trait.Hash.html
        if !float.s.accept_zero || !float.s.accept_positive || !float.s.accept_negative {
            output.extend(quote! {
                impl core::hash::Hash for #full_type {
                    #[inline]
                    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
                        self.0.to_bits().hash(state)
                    }
                }
            });
        }

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
