use quote::quote;
use syn::Ident;

use crate::impl_self::get_impl_self;
use crate::{get_definitions, get_impl_self_rhs, FloatDefinition, ReturnTypeDefinition};

fn test_op_checks(
    float: &FloatDefinition,
    op_name: &str,
    result_type: &ReturnTypeDefinition,
    var: &proc_macro2::Ident,
) -> proc_macro2::TokenStream {
    let mut res = proc_macro2::TokenStream::new();

    if let ReturnTypeDefinition::FloatDefinition(def) = result_type {
        let full_type = def.name;

        if def.s.accept_inf {
            res.extend(quote! {
                let has_inf = #var.iter().any(|x| x.is_infinite());
                assert!(has_inf, "No inf generated with {} but the output type {} accept it. Generated: {:?}", #op_name, stringify!(#full_type), #var);
            });
        };

        if def.s.accept_zero {
            res.extend(quote! {
                let has_zero = #var.iter().any(|x| x == &0.0);
                assert!(has_zero, "No zero generated with {} but the output type {} accept it. Generated: {:?}", #op_name, stringify!(#full_type), #var);
            });
        };

        if def.s.accept_positive {
            res.extend(quote! {
                let has_positive = #var.iter().any(|x| x.is_sign_positive());
                assert!(has_positive, "No positive generated with {} but the output type {} accept it. Generated: {:?}", #op_name, stringify!(#full_type), #var);
            });
        };

        if def.s.accept_negative {
            res.extend(quote! {
                let has_negative = #var.iter().any(|x| x.is_sign_negative());
                assert!(has_negative, "No negative generated with {} but the output type {} accept it. Generated: {:?}", #op_name, stringify!(#full_type), #var);
            });
        };
    } else {
        let full_type = float.float_type;

        res.extend(quote! {
            let has_nan = #var.iter().any(|x| x.is_nan());
            assert!(has_nan, "No NaN generated with {} but the output type {} accept it. Generated: {:?}", #op_name, stringify!(#full_type), #var);
        });
    }

    res
}

pub(crate) fn test_values(float_type: &Ident) -> proc_macro2::TokenStream {
    quote! {
        [
            core::#float_type::NAN,
            core::#float_type::NEG_INFINITY,
            core::#float_type::MIN,
            -core::#float_type::consts::PI,
            -core::#float_type::consts::E,
            -2.0,
            -core::#float_type::consts::FRAC_PI_2,
            -1.0,
            -core::#float_type::MIN_POSITIVE,
            -1.0e-308,
            -0.0,
            0.0,
            1.0e-308,
            core::#float_type::MIN_POSITIVE,
            1.0,
            core::#float_type::consts::FRAC_PI_2,
            2.0,
            core::#float_type::consts::E,
            core::#float_type::consts::PI,
            core::#float_type::MAX,
            core::#float_type::INFINITY,
        ]
    }
}

pub(crate) fn get_test_values(float_type: &Ident) -> proc_macro2::TokenStream {
    let values = test_values(float_type);

    quote! {
        let values: [#float_type; 21] = #values;

        for i in 1..values.len() {
            let value = values[i];
            let prev_value = values[i-1];

            if(value.is_nan() || prev_value.is_nan()) {
                continue;
            }

            if value != 0.0 && prev_value != 0.0 {
                assert!(value >prev_value, "values[{}] = {} <= values[{}] = {}", i, value, i-1, prev_value);
            }
        }
    }
}

pub fn generate_tests_self(float_type: &'static str, filter: &str) -> proc_macro2::TokenStream {
    let floats_f64 = get_definitions(float_type);

    let mut output = proc_macro2::TokenStream::new();

    let float_type = floats_f64
        .first()
        .expect("no float returned")
        .float_type_ident();

    let test_fn_name = quote::format_ident!("test_{float_type}_{filter}");

    let ops = get_impl_self()
        .into_iter()
        .filter(|x| x.key == filter)
        .collect::<Vec<_>>();

    assert!(!ops.is_empty());

    for float in &floats_f64 {
        let mut init_test_ops = proc_macro2::TokenStream::new();
        let mut test_ops = proc_macro2::TokenStream::new();
        let mut check_ops = proc_macro2::TokenStream::new();

        for op in &ops {
            let op_name = op.key;
            let vals = quote::format_ident!("all_{}", op_name);

            init_test_ops.extend(quote! {
                let mut #vals = Vec::<#float_type>::new();
            });

            let test = &op.get_test("num_a");
            let test_float = &op.get_test("a");

            let get = match &op.get_result(float, &floats_f64) {
                ReturnTypeDefinition::NativeFloat => quote! { res },
                ReturnTypeDefinition::FloatDefinition(_) => {
                    quote! { res.get() }
                }
            };

            test_ops.extend(quote! {
                println!("{:?} = ...",#op_name);

                // Execute the operation, will throw if the result type is too strict
                let res = #test;
                println!("{:?}({:?}) = {:?}",#op_name, a, res);

                // Get the result as a float
                let as_float = #get;

                // Check that the result is the same as if done with the float directly
                let original = #test_float;
                if original.is_nan() {
                    assert_eq!(original.is_nan(), as_float.is_nan());
                } else {
                    assert_eq!(as_float, original);
                }

                // Add the result to the list of values to check is the result type is as strict as possible
                #vals.push(as_float);
            });

            if op.skip_check_return_type_strictness {
                continue;
            }

            let result_type = op.get_result(float, &floats_f64);
            let checks = test_op_checks(float, op.display, &result_type, &vals);

            check_ops.extend(quote! {
                #checks
            });
        }

        let full_type = float.full_type_ident();

        output.extend(quote! {
            {
                #init_test_ops

                for a in values.iter() {
                    let a_float = <#full_type>::try_from(*a);

                    if let Ok(num_a) = a_float {
                        println!("compute with a = {:?}", num_a);

                        #test_ops
                    }
                }

                #check_ops
            }
        });
    }

    let values = get_test_values(&float_type);

    quote! {
        #[test]
        fn #test_fn_name() {
            #values

            #output
        }
    }
}

pub fn generate_tests_self_rhs(float_type: &'static str, filter: &str) -> proc_macro2::TokenStream {
    let floats_f64 = get_definitions(float_type);

    let mut output = proc_macro2::TokenStream::new();

    let float_type = floats_f64
        .first()
        .expect("no floats returned")
        .float_type_ident();

    let test_fn_name = quote::format_ident!("test_{float_type}_{filter}");

    let ops_rhs = get_impl_self_rhs()
        .into_iter()
        .filter(|x| x.key == filter)
        .collect::<Vec<_>>();

    assert!(!ops_rhs.is_empty());

    for float in &floats_f64 {
        let full_type = float.full_type_ident();

        let float_type = float.float_type_ident();

        for float_rhs in &floats_f64 {
            let mut init_test_ops = proc_macro2::TokenStream::new();
            let mut test_ops = proc_macro2::TokenStream::new();
            let mut check_ops = proc_macro2::TokenStream::new();

            for op in &ops_rhs {
                let op_name = op.key;
                let vals = quote::format_ident!("all_{}", op_name);

                init_test_ops.extend(quote! {
                    let mut #vals = Vec::<#float_type>::new();
                });

                let test = &op.get_test("num_a", "num_b");
                let test_float = &op.get_test_primitive("a", "b");

                let get = match &op.get_result(float, float_rhs, &floats_f64) {
                    ReturnTypeDefinition::NativeFloat => quote! { res },
                    ReturnTypeDefinition::FloatDefinition(_) => {
                        quote! { res.get() }
                    }
                };

                test_ops.extend(quote! {
                    println!("{:?} = ...",#op_name);
                    // This will panic if the result isn't compatible with the return type
                    let res = #test;
                    println!("{:?}({:?},{:?}) = {:?}",#op_name, a, b, res);

                    let f: #float_type = #get;

                    // Check that the result is the same as if done with the float directly
                    let original = #test_float;
                    if original.is_nan() {
                        assert_eq!(original.is_nan(), f.is_nan());
                    } else {
                        assert_eq!(original, f, "original op result is not the same as the implemented op");
                    }

                    #vals.push(f);
                });

                if op.op_is_commutative {
                    let test2 = &op.get_test("num_b", "num_a");

                    test_ops.extend(quote! {
                        {
                            let res2 = #test2;

                            if res == res  {
                                assert_eq!(res, #test2, "commutative property of {}", #op_name);
                            } else if res2 == res2 {
                                panic!("{} is not commutative. res={:?}, res2={:?}", #op_name, res, res2);
                            }
                        }
                    });
                }

                if op.skip_check_return_type_strictness {
                    continue;
                }

                let result_type = op.get_result(float, float_rhs, &floats_f64);
                let checks = test_op_checks(float, op.display, &result_type, &vals);

                check_ops.extend(quote! {
                    #checks
                });
            }

            let full_type_rhs = float_rhs.full_type_ident();

            output.extend(quote! {
                {
                    #init_test_ops

                    for a in values.iter() {
                        let a = *a;
                        let a_float = <#full_type>::try_from(a);

                        if let Ok(num_a) = a_float {
                            for b in values.iter() {
                                let b = *b;
                                let b_float = <#full_type_rhs>::try_from(b);

                                if let Ok(num_b) = b_float {
                                    println!("a = {:?} and b = {:?}", num_a, num_b);

                                    #test_ops
                                }
                            }
                        }
                    }

                    #check_ops
                }
            });
        }
    }

    let values = get_test_values(&float_type);

    quote! {
        #[test]
        fn #test_fn_name() {
            #values

            #output
        }
    }
}
