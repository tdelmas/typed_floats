mod tag;

fn main() {
    let command: Option<String> = std::env::args().nth(1);
    let args = std::env::args().skip(2).collect::<Vec<_>>();

    match command.as_deref() {
        Some("tag") => tag::tag(args),
        Some(command) => panic!("Invalid command: {command}"),
        None => panic!("No command provided"),
    }
}
