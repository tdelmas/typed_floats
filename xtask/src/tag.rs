pub fn tag(args: Vec<String>) {
    let mut version = None;
    let mut force = false;
    let mut skip_tests = false;

    let mut args = args.into_iter();
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--skip-tests" => skip_tests = true,
            "--force" => force = true,
            "--version" => version = args.next(),
            _ => panic!("Invalid argument"),
        }
    }

    std::process::Command::new("cargo")
        .args(["fmt"])
        .output()
        .unwrap();

    let dir = std::env::current_dir()
        .unwrap()
        .join(std::path::Path::new("typed_floats"));

    if !skip_tests {
        println!("Running tests...");
        std::process::Command::new("./tests.sh")
            .current_dir(dir)
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .output()
            .expect("failed run tests");
    } else {
        println!("Skipping tests...");
    }

    let is_clean = std::process::Command::new("git")
        .args(["status", "--porcelain"])
        .output()
        .unwrap()
        .stdout
        .is_empty();

    if !is_clean {
        println!("The git repository is not clean");
        if !force {
            println!("Use --force to force the tag");
            std::process::exit(1);
        }
    }

    let crate_version = get_version();

    let (major, minor, patch) = parse_version(&crate_version);

    let (major, minor, patch) = if let Some(version) = version {
        let version = parse_version(&version);

        if version.0 > major || version.1 > minor || version.2 > patch {
            version
        } else {
            println!("The version must be greater than the current version. Provided: {version:?}, Current: {crate_version:?}");
            std::process::exit(1);
        }
    } else {
        (major, minor, patch + 1)
    };

    let new_version = format!("{}.{}.{}", major, minor, patch);

    println!("Current version: {:?}", crate_version);
    println!("New version: {:?}", new_version);

    println!("Updating version in Cargo.toml files...");

    update_version(&crate_version, &new_version, "./Cargo.toml");
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

fn parse_version(version: &str) -> (u8, u8, u8) {
    let mut version = version.split('.');

    let major = version.next().unwrap().parse::<u8>().unwrap();
    let minor = version.next().unwrap().parse::<u8>().unwrap();
    let patch = version.next().unwrap().parse::<u8>().unwrap();

    (major, minor, patch)
}

fn get_version() -> String {
    let v = std::fs::read_to_string("./Cargo.toml").unwrap();
    let v = v.parse::<toml::Value>().unwrap();

    v["workspace"]["package"]["version"]
        .as_str()
        .unwrap()
        .into()
}

fn update_version(previous: &str, next: &str, path: &str) {
    let str = std::fs::read_to_string(path)
        .unwrap()
        .replace(previous, next);

    std::fs::write(path, str).unwrap();
}
