use std::fs;
use std::path::Path;

// Copy the README.md from the root (used by GitHub)
// to the crate root (used by crates.io)
fn copy_global_readme_to_crate_readme() {
    let orig_readme = Path::new("../README.md");
    let crate_readme = Path::new("./README.md");

    if crate_readme.exists() {
        return;
    }

    fs::copy(orig_readme, crate_readme).unwrap();
}

// Copy the README.md from the crate root (used by crates.io)
// to remove the parts that are not used by crates.io
// That truncated version is the introduction of the documentation
fn copy_truncated_readme_to_doc_readme() {
    let crate_readme = Path::new("./README.md");
    let trucated_readme = Path::new("./README.truncated.md");

    if trucated_readme.exists() {
        return;
    }

    let text_readme = fs::read_to_string(crate_readme).unwrap();

    let text = text_readme.split("# Full documentation").next().unwrap();

    let text = text.replace("```rust", "```ignore");

    fs::write(trucated_readme, text).unwrap();
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=README.md");
    println!("cargo:rerun-if-changed=README.truncated.md");

    copy_global_readme_to_crate_readme();
    copy_truncated_readme_to_doc_readme();
}
