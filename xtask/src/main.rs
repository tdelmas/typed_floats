mod pre_build;
mod tag;

fn main() {
    let command: Option<String> = std::env::args().nth(1);
    let args = std::env::args().skip(2).collect::<Vec<_>>();

    match command.as_deref() {
        Some("pre_build") => pre_build::create_creadmes(),
        Some("tag") => tag::tag(args),
        Some(command) => panic!("Invalid command: {command}"),
        None => panic!("No command provided"),
    }
}
