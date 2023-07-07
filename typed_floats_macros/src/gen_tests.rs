use quote::quote;

use crate::impl_self::*;
use crate::*;

fn test_op_checks(
    float: &FloatDefinition,
    op_name: &str,
    result_type: &Option<FloatDefinition>,
) -> proc_macro2::TokenStream {
    let (full_type, accept_inf, accept_zero, accept_positive, accept_negative) = match result_type {
        None => (float.float_type, true, true, true, true),
        Some(result_type) => (
            result_type.name,
            result_type.s.accept_inf,
            result_type.s.accept_zero,
            result_type.s.accept_positive,
            result_type.s.accept_negative,
        ),
    };

    let mut res = proc_macro2::TokenStream::new();

    let check_inf = if accept_inf {
        quote! {
            let has_inf = all_res.iter().any(|x| x.is_infinite());
            assert!(has_inf, "No inf generated with {} but the output type {} accept it", #op_name, stringify!(#full_type));
        }
    } else {
        quote! {}
    };

    let check_zero = if accept_zero {
        quote! {
            let has_zero = all_res.iter().any(|x| x == &0.0);
            assert!(has_zero, "No zero generated with {} but the output type {} accept it", #op_name, stringify!(#full_type));
        }
    } else {
        quote! {}
    };

    let check_positive = if accept_positive {
        quote! {
            let has_positive = all_res.iter().any(|x| x.is_sign_positive());
            assert!(has_positive, "No positive generated with {} but the output type {} accept it", #op_name, stringify!(#full_type));
        }
    } else {
        quote! {}
    };

    let check_negative = if accept_negative {
        quote! {
            let has_negative = all_res.iter().any(|x| x.is_sign_negative());
            assert!(has_negative, "No negative generated with {} but the output type {} accept it", #op_name, stringify!(#full_type));
        }
    } else {
        quote! {}
    };

    res.extend(quote! {
        let has_nan = all_res.iter().any(|x| x.is_nan());

        if !has_nan {
            #check_inf
            #check_zero
            #check_positive
            #check_negative
        }
    });

    res
}

pub(crate) fn generate_tests() -> proc_macro::TokenStream {
    let floats_f64 = get_definitions("f64");

    let mut output = proc_macro2::TokenStream::new();

    let float_type = floats_f64[0].float_type_ident();

    for float in &floats_f64 {
        let float_type = float.float_type_ident();
        let full_type = float.full_type_ident();

        let neg_result_type = neg_result(float, &floats_f64);
        let checks_neg = test_op_checks(float, "neg", &neg_result_type);

        let floor_result_type = floor_result(float, &floats_f64);
        let checks_floor = test_op_checks(float, "floor", &floor_result_type);

        let ceil_result_type = ceil_result(float, &floats_f64);
        let checks_ceil = test_op_checks(float, "ceil", &ceil_result_type);

        let round_result_type = round_result(float, &floats_f64);
        let checks_round = test_op_checks(float, "round", &round_result_type);

        let abs_result_type = abs_result(float, &floats_f64);
        let checks_abs = test_op_checks(float, "abs", &abs_result_type);

        output.extend(quote! {
            let mut all_neg = Vec::<#float_type>::new();
            let mut all_floor = Vec::<#float_type>::new();
            let mut all_ceil = Vec::<#float_type>::new();
            let mut all_round = Vec::<#float_type>::new();
            let mut all_abs = Vec::<#float_type>::new();

            for a in values.iter() {
                let a = <#full_type>::try_from(*a);

                if let Ok(num_a) = a {
                    println!("compute with a = {:?}", num_a);

                    let neg = -num_a;
                    println!("neg = {:?}", neg);
                    all_neg.push(neg.get());

                    let floor = num_a.floor();
                    println!("floor = {:?}", floor);
                    all_floor.push(floor.get());

                    let ceil = num_a.ceil();
                    println!("ceil = {:?}", ceil);
                    all_ceil.push(ceil.get());

                    let round = num_a.round();
                    println!("round = {:?}", round);
                    all_round.push(round.get());

                    let abs = num_a.abs();
                    println!("abs = {:?}", abs);
                    all_abs.push(abs.get());
                }
            }

            let all_res = all_neg;
            #checks_neg

            let all_res = all_floor;
            #checks_floor

            let all_res = all_ceil;
            #checks_ceil

            let all_res = all_round;
            #checks_round

            let all_res = all_abs;
            #checks_abs




        });

        for float_rhs in &floats_f64 {
            let full_type_rhs = float_rhs.full_type_ident();

            let add_result_type = add_result(&float.s, &float_rhs.s, &floats_f64);
            let checks_add = test_op_checks(float, "add", &add_result_type);
            let add_get = match &add_result_type {
                None => quote! { res },
                Some(_) => {
                    quote! { res.get() }
                }
            };

            let sub_result_type = sub_result(&float.s, &float_rhs.s, &floats_f64);
            let checks_sub = test_op_checks(float, "sub", &sub_result_type);
            let sub_get = match &sub_result_type {
                None => quote! { res },
                Some(_) => {
                    quote! { res.get() }
                }
            };

            let mul_result_type = mul_result(&float.s, &float_rhs.s, &floats_f64);
            let checks_mul = test_op_checks(float, "mul", &mul_result_type);
            let mul_get = match &mul_result_type {
                None => quote! { res },
                Some(_) => {
                    quote! { res.get() }
                }
            };

            let div_result_type = div_result(&float.s, &float_rhs.s, &floats_f64);
            let checks_div = test_op_checks(float, "div", &div_result_type);
            let div_get = match &div_result_type {
                None => quote! { res },
                Some(_) => {
                    quote! { res.get() }
                }
            };

            let rem_result_type = rem_result(&float.s, &float_rhs.s, &floats_f64);
            let checks_rem = test_op_checks(float, "rem", &rem_result_type);
            let rem_get = match &rem_result_type {
                None => quote! { res },
                Some(_) => {
                    quote! { res.get() }
                }
            };

            output.extend(quote! {
                let mut all_add = Vec::<#float_type>::new();
                let mut all_sub = Vec::<#float_type>::new();
                let mut all_mul = Vec::<#float_type>::new();
                let mut all_div = Vec::<#float_type>::new();
                let mut all_rem = Vec::<#float_type>::new();


                for a in values.iter() {
                    for b in values.iter() {
                        let a = <#full_type>::try_from(*a);
                        let b = <#full_type_rhs>::try_from(*b);

                        if let Ok(num_a) = a {
                            if let Ok(num_b) = b {
                                println!("a = {:?} and b = {:?}", num_a, num_b);

                                let res = num_a+num_b;
                                println!("a+b = {:?}", res);
                                all_add.push(#add_get);

                                let res = num_a-num_b;
                                println!("a-b = {:?}", res);
                                all_sub.push(#sub_get);

                                let res = num_a*num_b;
                                println!("a*b = {:?}", res);
                                all_mul.push(#mul_get);

                                let res = num_a/num_b;
                                println!("a/b = {:?}", res);
                                all_div.push(#div_get);

                                let res = num_a%num_b;
                                println!("a%b = {:?}", res);
                                all_rem.push(#rem_get);
                            }
                        }
                    }
                }

                let all_res = all_add;
                #checks_add

                let all_res = all_sub;
                #checks_sub

                let all_res = all_mul;
                #checks_mul

                let all_res = all_div;
                #checks_div

                let all_res = all_rem;
                #checks_rem
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
