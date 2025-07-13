use std::fs;
use std::path::Path;

/// Creates the README.md and README.truncated.md files in the crate root.
///
/// They are created from the original README.md file located in the parent directory.
/// The README.md is used by crates.io, while the README.truncated.md is used
/// as the introduction in the documentation.
///
/// # Panics
/// Panics if the original README.md file does not exist.
pub fn create_creadmes() {
    let orig_readme = Path::new("../README.md");
    let crate_readme = Path::new("../typed_floats/README.md");

    // Copy the README.md from the root (used by GitHub)
    // to the crate root (used by crates.io)
    fs::copy(orig_readme, crate_readme)
        .expect("Failed to copy README.md from project root to crate root");

    // Truncate the README to include it in the documentation of the crate
    let trucated_readme = Path::new("../typed_floats/README.truncated.md");

    // remove the parts that are not used by docs.io
    // That truncated version is the introduction of the documentation
    let text_readme = fs::read_to_string(crate_readme).expect("Failed to read crate README.md");

    let text = text_readme
        .split("# Full documentation")
        .next()
        .expect("Failed to find '# Full documentation' section in README.md");

    let text = text.replace("```rust", "```ignore");

    fs::write(trucated_readme, text).expect("Failed to write truncated README.md");
}

fn main() {
    println!("cargo:rerun-if-changed=../README.md");
    create_creadmes();
}
