use std::{env, fs, path::PathBuf};
fn main() {
    let root = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    println!("cargo:rerun-if-changed=assets.txt");
    let inventory = fs::read_to_string(root.join("assets.txt")).unwrap();
    let manifest = fs::read_to_string(root.join("Cargo.toml")).unwrap();
    for path in inventory.lines().filter(|s| !s.is_empty()) {
        assert!(
            manifest.contains(&format!("\"{path}\"")),
            "asset {path} missing from Cargo include list"
        );
    }
    let mut generated = String::from("&[\n");
    for path in inventory.lines().filter(|s| !s.is_empty()) {
        assert!(!path.starts_with('/') && !path.contains("..") && !path.contains('\\'));
        println!("cargo:rerun-if-changed={path}");
        generated.push_str(&format!(
            "({path:?}, include_bytes!({:?}) as &[u8]),\n",
            root.join(path).to_string_lossy()
        ));
    }
    generated.push_str("]\n");
    fs::write(
        PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("assets.rs"),
        generated,
    )
    .unwrap();
}
