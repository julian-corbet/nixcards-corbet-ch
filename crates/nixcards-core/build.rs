use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn collect_sets(directory: &Path, files: &mut Vec<PathBuf>) {
    let entries = fs::read_dir(directory)
        .unwrap_or_else(|error| panic!("cannot read {}: {error}", directory.display()));

    for entry in entries {
        let path = entry.expect("cannot read card directory entry").path();
        if path.is_dir() {
            collect_sets(&path, files);
        } else if path.file_name().is_some_and(|name| name == "set.md") {
            files.push(path);
        }
    }
}

fn main() {
    let manifest = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));
    let workspace = manifest
        .parent()
        .and_then(Path::parent)
        .expect("core crate must be two levels below the workspace");
    let cards = workspace.join("cards");
    let mut files = Vec::new();
    collect_sets(&cards, &mut files);
    files.sort();

    assert!(
        !files.is_empty(),
        "the bundled catalogue must contain a card set"
    );

    let mut generated =
        String::from("pub(crate) const BUNDLED_CARD_SOURCES: &[(&str, &str)] = &[\n");

    for file in files {
        let relative = file
            .strip_prefix(workspace)
            .expect("card path must be below workspace")
            .to_string_lossy()
            .replace('\\', "/");
        generated.push_str(&format!(
            "    ({relative:?}, include_str!({absolute:?})),\n",
            absolute = file.to_string_lossy()
        ));
        println!("cargo:rerun-if-changed={}", file.display());
    }

    generated.push_str("];\n");
    let output = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR")).join("bundled_cards.rs");
    fs::write(output, generated).expect("cannot write bundled catalogue source");
    println!("cargo:rerun-if-changed={}", cards.display());
}
