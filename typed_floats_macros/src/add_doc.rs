use crate::impl_self_rhs::*;

use crate::types::FloatDefinition;

pub(crate) fn generate_main_description(floats: &[FloatDefinition]) -> proc_macro2::TokenStream {
    let mut output: proc_macro2::TokenStream = proc_macro2::TokenStream::new();

    output.extend(comment_line(
        "/// When the result is [`f64`], it may be `NaN`.",
    ));
    output.extend(generate_op_table(floats, "+"));
    output.extend(generate_op_table(floats, "-"));
    output.extend(generate_op_table(floats, "%"));
    output.extend(generate_op_table(floats, "/"));

    output
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
