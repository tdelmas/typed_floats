use proc_macro2::Span;
use quote::quote;
use syn::Ident;

#[derive(Clone, Debug)]
pub(crate) struct FloatSpecifications {
    pub(crate) accept_inf: bool,
    pub(crate) accept_zero: bool,
    pub(crate) accept_positive: bool,
    pub(crate) accept_negative: bool,
}

#[derive(Clone, Debug)]
pub(crate) struct FloatDefinition {
    pub(crate) name: &'static str,
    pub(crate) float_type: &'static str,
    pub(crate) s: FloatSpecifications,
}

impl FloatDefinition {
    pub(crate) fn name_ident(&self) -> Ident {
        Ident::new(self.name, Span::call_site())
    }

    pub(crate) fn float_type_ident(&self) -> Ident {
        Ident::new(self.float_type, Span::call_site())
    }

    pub(crate) fn full_type_ident(&self) -> proc_macro2::TokenStream {
        let name = self.name_ident();
        let float_type = self.float_type_ident();

        quote! { #name<#float_type> }
    }

    pub(crate) fn call_tokens(&self) -> proc_macro2::TokenStream {
        let name = self.name_ident();
        let float_type = self.float_type_ident();

        quote! { #name::<#float_type> }
    }
}

impl FloatSpecifications {
    pub(crate) fn can_fit_into(&self, into: &FloatSpecifications) -> bool {
        (!self.accept_inf || into.accept_inf)
            && (!self.accept_zero || into.accept_zero)
            && (!self.accept_positive || into.accept_positive)
            && (!self.accept_negative || into.accept_negative)
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

pub(crate) fn find_float(
    float: &FloatSpecifications,
    floats: &[FloatDefinition],
) -> Option<FloatDefinition> {
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

pub(crate) fn output_name(
    output: &Option<FloatDefinition>,
    float_type: &Ident,
) -> proc_macro2::TokenStream {
    match output {
        Some(output) => {
            let full_type = output.full_type_ident();

            quote! { #full_type }
        }
        None => quote! { #float_type },
    }
}


pub(crate) struct Op {
    pub(crate) fn_name: String,
    pub(crate) trait_name: Option<String>,
    op: Box<dyn Fn(&FloatDefinition) -> proc_macro2::TokenStream>,
    result: Box<dyn Fn(&FloatDefinition, &[FloatDefinition]) -> Option<FloatDefinition>>,
}

impl Op {
    pub(crate) fn new(
        fn_name: &str,
        trait_name: Option<&str>,
        op: Box<dyn Fn(&FloatDefinition) -> proc_macro2::TokenStream>,
        result: Box<dyn Fn(&FloatDefinition, &[FloatDefinition]) -> Option<FloatDefinition>>,
    ) -> Self {
        Self {
            fn_name: fn_name.to_string(),
            trait_name: trait_name.map(|s| s.to_string()),
            op,
            result,
        }
    }

    pub(crate) fn get_result(
        &self,
        float: &FloatDefinition,
        floats: &[FloatDefinition],
    ) -> Option<FloatDefinition> {
        (self.result)(float, floats)
    }

    pub(crate) fn get_op(&self, float: &FloatDefinition) -> proc_macro2::TokenStream {
        (self.op)(float)
    }
    
    pub(crate) fn get_impl(
        &self,
        float: &FloatDefinition,
        floats: &[FloatDefinition],
    ) -> proc_macro2::TokenStream {
        let output = self.get_result(float, &floats);

        let float_full_type = &float.full_type_ident();

        let op = &self.get_op(float);

        let return_value = match &output {
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

        let output_name = output_name(&output, &float.float_type_ident());

        let fn_ident = Ident::new(self.fn_name.as_str(), Span::call_site());

        if let Some(trait_name) = &self.trait_name {
            let trait_name: proc_macro2::TokenStream = trait_name.parse().unwrap();

            quote! {
                impl #trait_name for #float_full_type {
                    type Output = #output_name;

                    #[inline]
                    #[must_use]
                    fn #fn_ident(self) -> Self::Output {
                        #return_value
                    }
                }
            }
        } else {
            quote! {
                impl #float_full_type {
                    #[inline]
                    #[must_use]
                    pub fn #fn_ident(self) -> #output_name {
                        #return_value
                    }
                }
            }
        }
    }
}
