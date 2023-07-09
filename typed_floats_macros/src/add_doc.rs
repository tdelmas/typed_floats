use crate::impl_self::*;
use crate::impl_self_rhs::*;
use crate::types::*;

use crate::types::FloatDefinition;

pub(crate) fn generate_main_description(floats: &[FloatDefinition]) -> proc_macro2::TokenStream {
    let mut output: proc_macro2::TokenStream = proc_macro2::TokenStream::new();

    let ops = get_impl_self_rhs();

    output.extend(comment_line(
        "/// When the result is [`f64`], it may be `NaN`.",
    ));
    output.extend(comment_line("///"));
    output.extend(generate_fn_table(floats));

    for op in ops {
        output.extend(generate_op_table(floats, &op));
    }

    output.extend(comment_line("///"));

    output
}

fn comment_line(str: &str) -> proc_macro2::TokenStream {
    str.parse().unwrap()
}

fn generate_op_table(floats: &[FloatDefinition], op: &OpRhs) -> proc_macro2::TokenStream {
    let mut output = proc_macro2::TokenStream::new();

    let op_name = &op.display;
    let mut str: String = format!("/// |  {op_name}  |");

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
            let result = op.get_result(float, rhs, floats);

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

    let ops = get_impl_self();

    for op in ops {
        let name = &op.display;
        let mut str: String = format!("/// | {name} | ").to_string();

        for float in floats {
            let float_type = float.float_type;

            let result = op.get_result(float, floats);

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
