#!/usr/bin/env cargo

//! ```cargo
//! [package]
//! edition = "2021"
//! [dependencies]
//! ```

use std::fs;
use std::path::Path;

// Copy the README.md from the root (used by GitHub)
// to the crate root (used by crates.io)
fn copy_global_readme_to_crate_readme() {
    let orig_readme = Path::new("./README.md");
    let crate_readme = Path::new("./typed_floats/README.md");

    fs::copy(orig_readme, crate_readme).unwrap();
}

// Copy the README.md from the crate root (used by crates.io)
// to remove the parts that are not used by crates.io
// That truncated version is the introduction of the documentation
fn copy_truncated_readme_to_doc_readme() {
    let crate_readme = Path::new("./typed_floats/README.md");
    let trucated_readme = Path::new("./typed_floats/README.truncated.md");

    let text_readme = fs::read_to_string(crate_readme).unwrap();

    let text = text_readme.split("# Full documentation").next().unwrap();

    let text = text.replace("```rust", "```ignore");

    fs::write(trucated_readme, text).unwrap();
}

fn main() {
    copy_global_readme_to_crate_readme();
    copy_truncated_readme_to_doc_readme();
}
