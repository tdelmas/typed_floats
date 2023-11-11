use clap::{Parser, Subcommand};
use std::fs;
use std::path::Path;

#[derive(Parser, Debug)]
#[command(arg_required_else_help(true))]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Tag(TagArgs),
    PreBuild,
}

#[derive(Parser, Debug)]
struct TagArgs {
    #[clap(long, help = "The new version")]
    version: Option<String>,
    #[clap(long, help = "Force the tag")]
    force: bool,
}

fn main() {
    let cli: Cli = Cli::parse();

    match &cli.command {
        Some(Commands::PreBuild) => create_creadmes(),
        Some(Commands::Tag(args)) => tag(args),
        None => {}
    }
}

fn create_creadmes() {
    let orig_readme = Path::new("./README.md");
    let crate_readme = Path::new("./typed_floats/README.md");

    // Copy the README.md from the root (used by GitHub)
    // to the crate root (used by crates.io)
    fs::copy(orig_readme, crate_readme).unwrap();

    // Truncate the README to include it in the documentation of the crate
    let trucated_readme = Path::new("./typed_floats/README.truncated.md");

    // remove the parts that are not used by docs.io
    // That truncated version is the introduction of the documentation
    let text_readme = fs::read_to_string(crate_readme).unwrap();

    let text = text_readme.split("# Full documentation").next().unwrap();

    let text = text.replace("```rust", "```ignore");

    fs::write(trucated_readme, text).unwrap();
}

fn parse_version(version: &str) -> (u8, u8, u8) {
    let mut version = version.split('.');

    let major = version.next().unwrap().parse::<u8>().unwrap();
    let minor = version.next().unwrap().parse::<u8>().unwrap();
    let patch = version.next().unwrap().parse::<u8>().unwrap();

    (major, minor, patch)
}

fn get_version(path: std::path::PathBuf) -> String {
    let v = std::fs::read_to_string(path.join("Cargo.toml")).unwrap();
    let v = v.parse::<toml::Value>().unwrap();

    v["package"]["version"].as_str().unwrap().into()
}

fn update_version(previous: &str, next: &str, path: &str) {
    let str = std::fs::read_to_string(path)
        .unwrap()
        .replace(previous, next);

    std::fs::write(path, str).unwrap();
}

fn tag(args: &TagArgs) {
    std::process::Command::new("cargo")
        .args(["fmt"])
        .output()
        .unwrap();

    std::process::Command::new("./tests.sh")
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .output()
        .expect("failed run tests");

    let is_clean = std::process::Command::new("git")
        .args(["status", "--porcelain"])
        .output()
        .unwrap()
        .stdout
        .is_empty();

    if !is_clean {
        println!("The git repository is not clean");
        if !args.force {
            println!("Use --force to force the tag");
            std::process::exit(1);
        }
    }

    let crate_version = get_version("./typed_floats".into());
    let macros_version = get_version("./typed_floats_macros".into());

    if crate_version != macros_version {
        println!("The crate version and the macros version must be the same");
        std::process::exit(1);
    }

    let (major, minor, patch) = parse_version(&crate_version);

    let (major, minor, patch) = if args.version.is_some() {
        let version = args.version.clone().unwrap();
        let version = parse_version(&version);

        if version.0 > major || version.1 > minor || version.2 > patch {
            version
        } else {
            println!("The version must be greater than the current version");
            std::process::exit(1);
        }
    } else {
        (major, minor, patch + 1)
    };

    let new_version = format!("{}.{}.{}", major, minor, patch);

    println!("Current version: {:?}", crate_version);
    println!("New version: {:?}", new_version);

    println!("Updating version in Cargo.toml files...");

    update_version(&crate_version, &new_version, "./typed_floats/Cargo.toml");
    update_version(
        &crate_version,
        &new_version,
        "./typed_floats_macros/Cargo.toml",
    );

    //build
    std::process::Command::new("cargo")
        .args(["build", "--release"])
        .output()
        .unwrap();

    println!("Commiting files...");

    std::process::Command::new("git")
        .args([
            "add",
            "./typed_floats/Cargo.toml",
            "./typed_floats_macros/Cargo.toml",
            "./Cargo.lock",
        ])
        .output()
        .unwrap();

    std::process::Command::new("git")
        .args(["commit", "-m", &new_version])
        .output()
        .unwrap();

    println!("Push to remote...");

    std::process::Command::new("git")
        .args(["push"])
        .output()
        .unwrap();

    println!("Tagging...");

    std::process::Command::new("git")
        .args(["tag", "-a", &new_version, "-m", &new_version])
        .output()
        .unwrap();

    println!("Pushing tag to trigger publish...");

    std::process::Command::new("git")
        .args(["push", "origin", &new_version])
        .output()
        .unwrap();
}
