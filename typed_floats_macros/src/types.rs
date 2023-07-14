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
    pub(crate) key: &'static str,
    pub(crate) display: &'static str,
    pub(crate) fn_name: &'static str,
    pub(crate) trait_name: Option<&'static str>,
    op: Box<dyn Fn(&FloatDefinition) -> proc_macro2::TokenStream>,
    result: Box<dyn Fn(&FloatDefinition, &[FloatDefinition]) -> Option<FloatDefinition>>,
    test: Box<dyn Fn(&Ident) -> proc_macro2::TokenStream>,
}

impl Op {
    pub(crate) fn new(
        key: &'static str,
        display: &'static str,
        fn_name: &'static str,
        trait_name: Option<&'static str>,
        op: Box<dyn Fn(&FloatDefinition) -> proc_macro2::TokenStream>,
        test: Box<dyn Fn(&Ident) -> proc_macro2::TokenStream>,
        result: Box<dyn Fn(&FloatDefinition, &[FloatDefinition]) -> Option<FloatDefinition>>,
    ) -> Self {
        Self {
            key,
            display,
            fn_name,
            trait_name,
            op,
            result,
            test,
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

    pub(crate) fn get_test(&self, var: &str) -> proc_macro2::TokenStream {
        let var = Ident::new(var, Span::call_site());
        (self.test)(&var)
    }

    pub(crate) fn get_impl(
        &self,
        float: &FloatDefinition,
        floats: &[FloatDefinition],
    ) -> proc_macro2::TokenStream {
        let output = self.get_result(float, floats);

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

        let fn_ident = Ident::new(self.fn_name, Span::call_site());

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

pub(crate) struct OpRhs {
    pub(crate) key: &'static str,
    pub(crate) display: &'static str,
    pub(crate) fn_name: &'static str,
    pub(crate) trait_name: &'static str,
    pub(crate) assign: Option<(&'static str, &'static str)>,
    op: Box<dyn Fn(&FloatDefinition, &FloatDefinition) -> proc_macro2::TokenStream>,
    result: Box<
        dyn Fn(&FloatDefinition, &FloatDefinition, &[FloatDefinition]) -> Option<FloatDefinition>,
    >,
    test: Box<dyn Fn(&Ident, &Ident) -> proc_macro2::TokenStream>,
}

impl OpRhs {
    pub(crate) fn new(
        key: &'static str,
        display: &'static str,
        (trait_name, fn_name): (&'static str, &'static str),
        assign: Option<(&'static str, &'static str)>,
        op: Box<dyn Fn(&FloatDefinition, &FloatDefinition) -> proc_macro2::TokenStream>,
        test: Box<dyn Fn(&Ident, &Ident) -> proc_macro2::TokenStream>,
        result: Box<
            dyn Fn(
                &FloatDefinition,
                &FloatDefinition,
                &[FloatDefinition],
            ) -> Option<FloatDefinition>,
        >,
    ) -> Self {
        if !key.chars().all(|c| c.is_ascii_lowercase()) {
            panic!("key must be only a-z lowercase");
        }

        Self {
            key,
            display,
            fn_name,
            trait_name,
            assign,
            op,
            result,
            test,
        }
    }

    pub(crate) fn get_result(
        &self,
        float: &FloatDefinition,
        rhs: &FloatDefinition,
        floats: &[FloatDefinition],
    ) -> Option<FloatDefinition> {
        (self.result)(float, rhs, floats)
    }

    pub(crate) fn get_op(
        &self,
        float: &FloatDefinition,
        rhs: &FloatDefinition,
    ) -> proc_macro2::TokenStream {
        (self.op)(float, rhs)
    }

    pub(crate) fn get_test(&self, var1: &str, var2: &str) -> proc_macro2::TokenStream {
        let var1 = Ident::new(var1, Span::call_site());
        let var2 = Ident::new(var2, Span::call_site());
        (self.test)(&var1, &var2)
    }

    pub(crate) fn get_impl(
        &self,
        float: &FloatDefinition,
        rhs: &FloatDefinition,
        floats: &[FloatDefinition],
    ) -> proc_macro2::TokenStream {
        let output = self.get_result(float, rhs, floats);

        let float_full_type = &float.full_type_ident();
        let rhs_full_type = &rhs.full_type_ident();

        let op = &self.get_op(float, rhs);

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

        let output_name = output_name(&output, &float.float_type_ident());

        let trait_ident: syn::Path = syn::parse_str(&self.trait_name).unwrap();
        let fn_ident = Ident::new(self.fn_name, Span::call_site());

        let mut res = quote! {
            impl #trait_ident<#rhs_full_type> for #float_full_type {
                type Output = #output_name;

                #[inline]
                fn #fn_ident(self, rhs: #rhs_full_type) -> Self::Output {
                    #return_value
                }
            }
        };

        if let Some((assign_trait, assign_fn)) = &self.assign {
            if let Some(output) = output {
                if output.s.can_fit_into(&float.s) {
                    let trait_assign_ident: syn::Path = syn::parse_str(assign_trait).unwrap();
                    let fn_assign_ident = Ident::new(assign_fn, Span::call_site());

                    res.extend(quote! {
                        impl #trait_assign_ident<#rhs_full_type> for #float_full_type {
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
        }

        res
    }
}
