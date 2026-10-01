//! Generate and verify the TypeScript bindings for the schema-5 diagram product.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use diagram_product::wire::DiagramProduct;
use ts_rs::TS;

fn files_in(directory: &Path) -> Result<BTreeMap<PathBuf, Vec<u8>>, String> {
    let mut files = BTreeMap::new();
    for entry in fs::read_dir(directory).map_err(|error| error.to_string())? {
        let entry = entry.map_err(|error| error.to_string())?;
        let path = entry.path();
        if !path.is_file() || path.extension().is_none_or(|extension| extension != "ts") {
            return Err(format!("unexpected binding entry: {}", path.display()));
        }
        files.insert(
            path.file_name().expect("binding filename").into(),
            fs::read(&path).map_err(|error| error.to_string())?,
        );
    }
    Ok(files)
}

fn run() -> Result<(), String> {
    let check = std::env::args().skip(1).any(|arg| arg == "--check");
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .map_err(|error| error.to_string())?;
    let published = root.join("vscode/src/generated/diagram-product");
    let temporary = root
        .join("target")
        .join(format!("diagram-bindings-{}", std::process::id()));
    fs::create_dir_all(&temporary).map_err(|error| error.to_string())?;
    std::env::set_var("TS_RS_EXPORT_DIR", &temporary);
    DiagramProduct::export_all().map_err(|error| error.to_string())?;
    let expected = files_in(&temporary)?;
    if expected.is_empty() {
        return Err("ts-rs produced no bindings".to_owned());
    }
    if check {
        let actual = files_in(&published)?;
        if expected != actual {
            return Err("diagram product TypeScript bindings are stale; run `cargo run -p diagram_product --bin diagram_bindings`".to_owned());
        }
    } else {
        fs::create_dir_all(&published).map_err(|error| error.to_string())?;
        for entry in fs::read_dir(&published).map_err(|error| error.to_string())? {
            let path = entry.map_err(|error| error.to_string())?.path();
            if path.extension().is_some_and(|extension| extension == "ts")
                && !expected
                    .contains_key(&PathBuf::from(path.file_name().expect("binding filename")))
            {
                fs::remove_file(path).map_err(|error| error.to_string())?;
            }
        }
        for (name, contents) in &expected {
            fs::write(published.join(name), contents).map_err(|error| error.to_string())?;
        }
        println!("Generated {} diagram product bindings", expected.len());
    }
    // The temporary directory is created inside this checkout's target directory with a fixed
    // task prefix; never remove a caller-supplied or arbitrary path.
    if !temporary.starts_with(root.join("target")) {
        return Err("binding scratch path escaped target".to_owned());
    }
    fs::remove_dir_all(temporary).map_err(|error| error.to_string())?;
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}
