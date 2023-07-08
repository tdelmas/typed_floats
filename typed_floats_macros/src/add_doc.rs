use crate::impl_self::*;
use crate::impl_self_rhs::*;

use crate::types::FloatDefinition;

pub(crate) fn generate_main_description(floats: &[FloatDefinition]) -> proc_macro2::TokenStream {
    let mut output: proc_macro2::TokenStream = proc_macro2::TokenStream::new();

    output.extend(comment_line(
        "/// When the result is [`f64`], it may be `NaN`.",
    ));
    output.extend(comment_line("///"));
    output.extend(generate_fn_table(floats));
    output.extend(generate_op_table(floats, "+"));
    output.extend(generate_op_table(floats, "-"));
    output.extend(generate_op_table(floats, "%"));
    output.extend(generate_op_table(floats, "/"));
    output.extend(comment_line("///"));

    output
}

fn comment_line(str: &str) -> proc_macro2::TokenStream {
    str.parse().unwrap()
}

fn generate_op_table(floats: &[FloatDefinition], op: &str) -> proc_macro2::TokenStream {
    let mut output = proc_macro2::TokenStream::new();

    let add = get_add();
    let sub = get_sub();
    let mul = get_mul();
    let rem = get_rem();
    let div = get_div();

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
                "+" => add.get_result(float, rhs, floats),
                "-" => sub.get_result(float, rhs, floats),
                "*" => mul.get_result(float, rhs, floats),
                "%" => rem.get_result(float, rhs, floats),
                "/" => div.get_result(float, rhs, floats),
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

fn generate_fn_table(floats: &[FloatDefinition]) -> proc_macro2::TokenStream {
    let mut output = proc_macro2::TokenStream::new();

    let fns = vec!["neg", "abs", "ceil", "floor", "round"];

    let mut str: String = "/// |   |".to_string();
    for rhs in floats {
        str += format!(" {rhs_name} |", rhs_name = rhs.name).as_str();
    }
    str += "\n";

    output.extend(comment_line(&str));

    let mut str: String = "/// | - |".to_string();
    for _ in floats {
        str += "-|";
    }
    str += "\n";

    output.extend(comment_line(&str));

    let neg = get_neg();
    let abs = get_abs();
    let ceil = get_ceil();
    let floor = get_floor();
    let round = get_round();

    for func in fns {
        let mut str: String = format!("/// {func} | ").to_string();

        for float in floats {
            let float_type = float.float_type;

            let result = match func {
                "neg" => neg.get_result(float, floats),
                "abs" => abs.get_result(float, floats),
                "ceil" => ceil.get_result(float, floats),
                "floor" => floor.get_result(float, floats),
                "round" => round.get_result(float, floats),
                _ => panic!("Unknown fn {}", func),
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
