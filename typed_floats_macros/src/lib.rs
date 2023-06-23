//! Crate only used to generate the `typed_floats` crate.

extern crate proc_macro;

use proc_macro2::Span;

use quote::quote;
use syn::Ident;

#[derive(Clone, Debug)]
struct FloatSpecifications {
    accept_inf: bool,
    accept_zero: bool,
    accept_positive: bool,
    accept_negative: bool,
}

#[derive(Clone, Debug)]
struct FloatDefinition {
    name: &'static str,
    float_type: &'static str,
    s: FloatSpecifications,
}

impl FloatDefinition {
    fn name_ident(&self) -> Ident {
        Ident::new(self.name, Span::call_site())
    }

    fn float_type_ident(&self) -> Ident {
        Ident::new(self.float_type, Span::call_site())
    }

    fn full_type_ident(&self) -> proc_macro2::TokenStream {
        let name = self.name_ident();
        let float_type = self.float_type_ident();

        quote! { #name<#float_type> }
    }

    fn call_tokens(&self) -> proc_macro2::TokenStream {
        let name = self.name_ident();
        let float_type = self.float_type_ident();

        quote! { #name::<#float_type> }
    }
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

    output.extend(comment_line(
        "/// When the result is [`f64`], it may be `NaN`.",
    ));
    output.extend(generate_op_table(&floats_f64, "+"));
    output.extend(generate_op_table(&floats_f64, "-"));
    output.extend(generate_op_table(&floats_f64, "%"));
    output.extend(generate_op_table(&floats_f64, "/"));
    output.extend(quote! {
        pub trait Float {}
    });

    output.extend(do_generate_floats(&floats_f64, true));
    output.extend(do_generate_floats(&floats_f32, false));

    output.into()
}

#[proc_macro]
pub fn generate_tests(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let floats_f64 = get_definitions("f64");

    let mut output = proc_macro2::TokenStream::new();

    let float_type = floats_f64[0].float_type_ident();

    for float_a in &floats_f64 {
        let full_type_a = float_a.full_type_ident();

        output.extend(quote! {
            for a in values.iter() {
                let a = <#full_type_a>::try_from(*a);

                println!("a = {:?}", a);
                match a {
                    Ok(num_a) => {
                        println!("num={:?}", num_a);
                        println!("neg={:?}", -num_a);
                        println!("abs={:?}", num_a.abs());
                    }
                    Err(_) => {}
                }
            }
        });

        for float_b in &floats_f64 {
            let full_type_b = float_b.full_type_ident();

            output.extend(quote! {
                for a in values.iter() {

                    for b in values.iter() {
                        println!("a = {:?} b = {:?}", a, b);
                        let a = <#full_type_a>::try_from(*a);
                        let b = <#full_type_b>::try_from(*b);
                        println!("a = {:?} b = {:?}", a, b);


                        match (a, b) {
                            (Ok(num_a), Ok(num_b)) => {
                                println!("{:?} + {:?} = {:?}", num_a, num_b, num_a + num_b);
                                println!("{:?} - {:?} = {:?}", num_a, num_b, num_a - num_b);
                                println!("{:?} * {:?} = {:?}", num_a, num_b, num_a * num_b);
                                println!("{:?} / {:?} = {:?}", num_a, num_b, num_a / num_b);
                                println!("{:?} % {:?} = {:?}", num_a, num_b, num_a % num_b);
                            }
                            _ => {}
                        }
                    }
                }
            });
        }
    }

    quote! {
        #[test]
        fn test_floats() {

            let values = [
                #float_type::NAN,
                #float_type::INFINITY,
                #float_type::NEG_INFINITY,
                #float_type::MAX,
                #float_type::MIN,
                #float_type::MIN_POSITIVE,
                -#float_type::MIN_POSITIVE,
                0.0,
                -0.0,
                1.0,
                2.0,
                -1.0,
                -2.0,
            ];

            #output
        }
    }
    .into()
}

fn comment_line(str: &str) -> proc_macro2::TokenStream {
    str.parse().unwrap()
}

fn generate_op_table(floats: &[FloatDefinition], op: &str) -> proc_macro2::TokenStream {
    let mut output = proc_macro2::TokenStream::new();

    let mut str: String = format!("/// |  {op}  |");
    for rhs in floats {
        str += format!(" {rhs_name} |", rhs_name = rhs.name).as_str();
    }
    str += "\n";

    output.extend(comment_line(&str));

    let mut str: String = "/// |-|".to_string();
    for _ in floats {
        str += "-|";
    }
    str += "\n";

    output.extend(comment_line(&str));

    for float in floats {
        let name = float.name;
        let float_type = float.float_type;

        let mut str: String = format!("/// {name} | ").to_string();

        for rhs in floats {
            let result = match op {
                "+" => add_result(&float.s, &rhs.s, floats),
                "-" => sub_result(&float.s, &rhs.s, floats),
                "%" => rem_result(&float.s, &rhs.s, floats),
                "/" => div_result(&float.s, &rhs.s, floats),
                _ => panic!("Unknown op {}", op),
            };

            let result_str = match result {
                Some(result) => result.name,
                None => float_type,
            };
            str += format!(" {result_str} |").as_str();
        }

        output.extend(comment_line(&str));
    }

    output.extend(comment_line("///\n"));

    output
}

fn do_generate_floats(floats: &[FloatDefinition], with_generic: bool) -> proc_macro2::TokenStream {
    let mut output = proc_macro2::TokenStream::new();

    for float in floats {
        let name = float.name_ident();
        let float_type = float.float_type_ident();
        let full_type = float.full_type_ident();

        if with_generic {
            output.extend(quote! {
                //#[cfg_attr(feature = "serde", derive(Serialize))]
                #[derive(Debug, Copy, Clone)]
                pub struct #name<T=#float_type>(T);
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
        output.extend(impl_neg(float_a, floats));
        output.extend(impl_abs(float_a, floats));

        for float_b in floats {
            output.extend(impl_add(float_a, float_b, floats));
            output.extend(impl_sub(float_a, float_b, floats));
            output.extend(impl_mul(float_a, float_b, floats));
            output.extend(impl_div(float_a, float_b, floats));
            output.extend(impl_rem(float_a, float_b, floats));
        }
    }

    output
}

fn output_name(output: &Option<FloatDefinition>, float_type: &Ident) -> proc_macro2::TokenStream {
    match output {
        Some(output) => {
            let full_type = output.full_type_ident();

            quote! { #full_type }
        }
        None => quote! { #float_type },
    }
}

fn compute_similarity(float_a: &FloatSpecifications, float_b: &FloatSpecifications) -> usize {
    let mut score = 0;

    if float_a.accept_inf == float_b.accept_inf {
        score += 1;
    }
    if float_a.accept_zero == float_b.accept_zero {
        score += 1;
    }
    if float_a.accept_positive == float_b.accept_positive {
        score += 1;
    }
    if float_a.accept_negative == float_b.accept_negative {
        score += 1;
    }

    score
}

fn find_float(float: &FloatSpecifications, floats: &[FloatDefinition]) -> Option<FloatDefinition> {
    //filter incompatibles
    let mut floats = floats
        .iter()
        .filter(|f| {
            if float.accept_inf && !f.s.accept_inf {
                return false;
            }
            if float.accept_zero && !f.s.accept_zero {
                return false;
            }
            if float.accept_positive && !f.s.accept_positive {
                return false;
            }
            if float.accept_negative && !f.s.accept_negative {
                return false;
            }
            true
        })
        .collect::<Vec<&FloatDefinition>>();

    let highest_score = floats
        .iter()
        .map(|f| compute_similarity(float, &f.s))
        .max()
        .unwrap();

    //keep only the highest score
    floats.retain(|f| compute_similarity(float, &f.s) == highest_score);

    if floats.len() > 1 {
        panic!("Ambiguous float type");
    }

    floats.first().map(|float| (*float).clone())
}

fn impl_neg(float: &FloatDefinition, floats: &[FloatDefinition]) -> proc_macro2::TokenStream {
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

fn impl_abs(float: &FloatDefinition, floats: &[FloatDefinition]) -> proc_macro2::TokenStream {
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

const fn can_fit_into(from: &FloatSpecifications, to: &FloatSpecifications) -> bool {
    (!from.accept_inf || to.accept_inf)
        && (!from.accept_zero || to.accept_zero)
        && (!from.accept_positive || to.accept_positive)
        && (!from.accept_negative || to.accept_negative)
}

fn add_result(
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

fn impl_add(
    float_a: &FloatDefinition,
    float_b: &FloatDefinition,
    floats: &[FloatDefinition],
) -> proc_macro2::TokenStream {
    let output = add_result(&float_a.s, &float_b.s, floats);

    let op = quote! { self.get() + rhs.get() };

    impl_op_rhs(float_a, float_b, &output, &op, "Add", "add")
}

fn sub_result(
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

fn impl_sub(
    float_a: &FloatDefinition,
    float_b: &FloatDefinition,
    floats: &[FloatDefinition],
) -> proc_macro2::TokenStream {
    let output = sub_result(&float_a.s, &float_b.s, floats);

    let op = quote! { self.get() - rhs.get() };

    impl_op_rhs(float_a, float_b, &output, &op, "Sub", "sub")
}

fn rem_result(
    spec_a: &FloatSpecifications,
    spec_b: &FloatSpecifications,
    floats: &[FloatDefinition],
) -> Option<FloatDefinition> {
    let can_be_nan = spec_b.accept_zero || spec_a.accept_inf;

    match can_be_nan {
        true => None,
        false => {
            let output_def = FloatSpecifications {
                accept_inf: spec_a.accept_inf || spec_b.accept_zero,
                accept_zero: true,
                accept_positive: spec_a.accept_positive,
                accept_negative: spec_a.accept_negative,
            };

            find_float(&output_def, floats)
        }
    }
}

fn impl_rem(
    float_a: &FloatDefinition,
    float_b: &FloatDefinition,
    floats: &[FloatDefinition],
) -> proc_macro2::TokenStream {
    let output = rem_result(&float_a.s, &float_b.s, floats);

    let op = quote! { self.get() % rhs.get() };

    impl_op_rhs(float_a, float_b, &output, &op, "Rem", "rem")
}

fn div_result(
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

fn impl_div(
    float_a: &FloatDefinition,
    float_b: &FloatDefinition,
    floats: &[FloatDefinition],
) -> proc_macro2::TokenStream {
    let output = div_result(&float_a.s, &float_b.s, floats);

    let op = quote! { self.get() / rhs.get() };

    impl_op_rhs(float_a, float_b, &output, &op, "Div", "div")
}

fn mul_result(
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

fn impl_mul(
    float: &FloatDefinition,
    rhs: &FloatDefinition,
    floats: &[FloatDefinition],
) -> proc_macro2::TokenStream {
    let output = mul_result(&float.s, &rhs.s, floats);

    let op = quote! { self.get() * rhs.get() };

    impl_op_rhs(float, rhs, &output, &op, "Mul", "mul")
}

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
        if can_fit_into(&output.s, &float.s) {
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

/// Conversion from float_a to float_b
fn impl_from_or_try_from(
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
    } else if can_fit_into(from, to) {
        impl_from(float_from, float_to)
    } else {
        impl_try_from(float_from, float_to)
    }
}

fn generate_try_from_float(float: &FloatDefinition) -> proc_macro2::TokenStream {
    let float_type = &float.float_type_ident();
    let full_type = &float.full_type_ident();
    let call_tokens = &float.call_tokens();

    let mut try_from_float = proc_macro2::TokenStream::new();

    try_from_float.extend(quote! {
        if value.is_nan() {
            return Err(InvalidNumber::NaN);
        }
    });

    if !float.s.accept_inf {
        try_from_float.extend(quote! {
            if value.is_infinite() {
                return Err(InvalidNumber::Infinite);
            }
        });
    }

    if !float.s.accept_zero {
        try_from_float.extend(quote! {
            if value == 0.0 {
                return Err(InvalidNumber::Zero);
            }
        });
    }

    if !float.s.accept_positive {
        try_from_float.extend(quote! {
            if value.is_sign_positive() {
                return Err(InvalidNumber::Positive);
            }
        });
    }

    if !float.s.accept_negative {
        try_from_float.extend(quote! {
            if value.is_sign_negative() {
                return Err(InvalidNumber::Negative);
            }
        });
    }

    quote! {
        impl TryFrom<#float_type> for #full_type {
            type Error = InvalidNumber;

            #[inline]
            #[must_use]
            fn try_from(value: #float_type) -> Result<Self, Self::Error> {
                #try_from_float

                Ok(#call_tokens(value))
            }
        }
    }
}

fn generate_try_ints(float: &FloatDefinition) -> proc_macro2::TokenStream {
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

fn generate_try_int(
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

fn generate_try_int_nonzero(
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
