use std::path::PathBuf;

fn main() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    // Walk up: crates/docusaurus → crates → workspace root
    let workspace_root = manifest_dir
        .parent()
        .expect("crate parent")
        .parent()
        .expect("workspace root");

    let vendor_js = workspace_root.join("vendor").join("runner.js");

    if !vendor_js.exists() {
        panic!(
            "vendor/runner.js not found — run `tsc` at the workspace root to compile the shim"
        );
    }

    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    std::fs::copy(&vendor_js, out_dir.join("runner.js")).unwrap();

    println!("cargo:rerun-if-changed={}", vendor_js.display());
    println!(
        "cargo:rerun-if-changed={}",
        workspace_root.join("shim").join("runner.ts").display()
    );
}
