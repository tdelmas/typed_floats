use clap::{Parser, Subcommand};

mod pre_build;
mod tag;

#[derive(Parser, Debug)]
#[command(arg_required_else_help(true))]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Tag(tag::TagArgs),
    PreBuild,
}

fn main() {
    let cli: Cli = Cli::parse();

    match &cli.command {
        Some(Commands::PreBuild) => pre_build::create_creadmes(),
        Some(Commands::Tag(args)) => tag::tag(args),
        None => {}
    }
}
