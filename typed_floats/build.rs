use std::fs;
use std::path::Path;

pub fn create_creadmes() {
    let orig_readme = Path::new("../README.md");
    let crate_readme = Path::new("../typed_floats/README.md");

    // Copy the README.md from the root (used by GitHub)
    // to the crate root (used by crates.io)
    fs::copy(orig_readme, crate_readme).unwrap();

    // Truncate the README to include it in the documentation of the crate
    let trucated_readme = Path::new("../typed_floats/README.truncated.md");

    // remove the parts that are not used by docs.io
    // That truncated version is the introduction of the documentation
    let text_readme = fs::read_to_string(crate_readme).unwrap();

    let text = text_readme.split("# Full documentation").next().unwrap();

    let text = text.replace("```rust", "```ignore");

    fs::write(trucated_readme, text).unwrap();
}

fn main() {
    println!("cargo:rerun-if-changed=../README.md");
    create_creadmes()
}