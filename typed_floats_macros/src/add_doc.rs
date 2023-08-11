use crate::impl_self::*;
use crate::impl_self_rhs::*;
use crate::types::*;

use crate::types::FloatDefinition;

pub(crate) fn generate_main_description(floats: &[FloatDefinition]) -> proc_macro2::TokenStream {
    let mut output: proc_macro2::TokenStream = proc_macro2::TokenStream::new();

    let ops = get_impl_self_rhs();

    output.extend(comment_line(
        "When the result may be `Nan`, the result type is [`f64`].",
    ));

    output.extend(generate_fn_table(floats));

    for op in ops {
        output.extend(generate_op_table(floats, &op));
    }

    output.extend(comment_line(""));

    output
}

fn comment_line(str: &str) -> proc_macro2::TokenStream {
    let comment: String = "/// ".to_owned() + str + "\n";
    comment.parse().unwrap()
}

fn print_table(content: Vec<Vec<String>>) -> proc_macro2::TokenStream {
    let mut output = proc_macro2::TokenStream::new();

    output.extend(comment_line(""));

    let first_line = content[0].clone();
    let lines = content[1..].to_vec();

    fn line_to_string(line: Vec<String>) -> String {
        let mut str = "|".to_string();
        for cell in line {
            str += cell.as_str();
            str += "|";
        }
        str
    }

    let sep: Vec<String> = first_line.iter().map(|_| "---".to_string()).collect();

    output.extend(comment_line(&line_to_string(first_line)));
    output.extend(comment_line(&line_to_string(sep)));
    lines.iter().for_each(|line| {
        output.extend(comment_line(&line_to_string(line.clone())));
    });

    output.extend(comment_line(""));

    output
}

fn generate_op_table(floats: &[FloatDefinition], op: &OpRhs) -> proc_macro2::TokenStream {
    let mut output = proc_macro2::TokenStream::new();

    let mut table: Vec<Vec<String>> = Vec::new();

    let mut header: Vec<String> = Vec::new();

    let mut title: String = op.display.to_string();

    if let Some(comment) = op.comment {
        let footnote = format!("[^{}]", op.key);
        title += &footnote;
        output.extend(comment_line(""));
        output.extend(comment_line(&(footnote + ": " + comment)));
    }

    header.push(title);
    header.extend(
        floats
            .iter()
            .map(|float| float.name.to_string())
            .collect::<Vec<String>>(),
    );

    table.push(header);

    for float in floats {
        let name = float.name;
        let float_type = float.float_type;

        let mut line: Vec<String> = Vec::new();

        line.push(name.to_string());

        for rhs in floats {
            let result = op.get_result(float, rhs, floats);

            let result_str = match result {
                ReturnTypeDefinition::FloatDefinition(result) => result.name,
                ReturnTypeDefinition::NativeFloat => float_type,
            };
            line.push(result_str.to_string());
        }

        table.push(line);
    }

    output.extend(print_table(table));

    output
}

fn generate_fn_table(floats: &[FloatDefinition]) -> proc_macro2::TokenStream {
    let mut output = proc_macro2::TokenStream::new();

    let mut table: Vec<Vec<String>> = Vec::new();

    let mut header: Vec<String> = Vec::new();
    header.push(" ".to_string());

    for rhs in floats {
        header.push(rhs.name.to_string());
    }

    let ops = get_impl_self();

    table.push(header);

    for op in ops {
        let mut line: Vec<String> = Vec::new();

        let mut title = op.display.to_string();

        if let Some(comment) = op.comment {
            let footnote = format!("[^{}]", op.key);
            title += &footnote;
            output.extend(comment_line(""));
            output.extend(comment_line(&(footnote + ": " + comment)));
        }

        line.push(title);

        for float in floats {
            let float_type = float.float_type;

            let result = op.get_result(float, floats);

            match result {
                ReturnTypeDefinition::FloatDefinition(result) => line.push(result.name.to_string()),
                ReturnTypeDefinition::NativeFloat => line.push(float_type.to_string()),
            };
        }

        table.push(line);
    }

    output.extend(print_table(table));

    output
}
