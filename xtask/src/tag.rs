pub fn tag(args: Vec<String>) {
    let mut force = false;
    let mut skip_tests = false;

    for arg in args {
        match arg.as_str() {
            "--skip-tests" => skip_tests = true,
            "--force" => force = true,
            _ => panic!("Invalid argument"),
        }
    }

    std::process::Command::new("cargo")
        .args(["fmt"])
        .output()
        .unwrap();

    std::process::Command::new("cargo")
        .args(["semver-checks"])
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

    let crate_version = get_version().replace("-dev", "");

    update_version("./Cargo.toml");
    update_version("./typed_floats/Cargo.toml");
    //build
    std::process::Command::new("cargo")
        .args(["build", "--release"])
        .output()
        .unwrap();

    println!("Committing files...");

    std::process::Command::new("git")
        .args([
            "add",
            "./Cargo.toml",
            "./typed_floats/Cargo.toml",
            "./typed_floats_macros/Cargo.toml",
            "./Cargo.lock",
        ])
        .output()
        .unwrap();

    std::process::Command::new("git")
        .args(["commit", "-m", &crate_version])
        .output()
        .unwrap();

    println!("Push to remote...");

    std::process::Command::new("git")
        .args(["push"])
        .output()
        .unwrap();

    println!("Tagging...");

    std::process::Command::new("git")
        .args(["tag", "-a", &crate_version, "-m", &crate_version])
        .output()
        .unwrap();

    println!("Pushing tag to trigger publish...");

    std::process::Command::new("git")
        .args(["push", "origin", &crate_version])
        .output()
        .unwrap();
}

fn get_version() -> String {
    let v = std::fs::read_to_string("./Cargo.toml").unwrap();
    let v = v.parse::<toml::Value>().unwrap();

    v["workspace"]["package"]["version"]
        .as_str()
        .unwrap()
        .into()
}

fn update_version(path: &str) {
    let str = std::fs::read_to_string(path).unwrap().replace("-dev", "");

    std::fs::write(path, str).unwrap();
}
