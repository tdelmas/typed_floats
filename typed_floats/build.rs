use std::fs;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let orig_readme = Path::new("../README.md");
    let crate_readme = Path::new("./README.md");

    fs::copy(orig_readme, crate_readme).unwrap();

    let text_readme = fs::read_to_string(crate_readme).unwrap();

    let text = text_readme.split("# Full documentation").next().unwrap();

    let readme_docs_rs = Path::new("./README.truncated.md");

    fs::write(readme_docs_rs, text).unwrap();
}
