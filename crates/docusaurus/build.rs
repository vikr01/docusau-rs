use std::path::PathBuf;
use std::process::Command;

fn main() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    // Walk up: crates/docusaurus → crates → workspace root
    let workspace_root = manifest_dir
        .parent()
        .expect("crate parent")
        .parent()
        .expect("workspace root");

    let shim_ts = workspace_root.join("shim").join("runner.ts");
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());

    // Locate tsc via PATH.
    let tsc = which::which("tsc")
        .expect("tsc not found in PATH — install typescript: npm install -g typescript");

    let status = Command::new(tsc)
        .arg(&shim_ts)
        .arg("--outDir")
        .arg(&out_dir)
        .arg("--target")
        .arg("ES2020")
        .arg("--module")
        .arg("commonjs")
        .arg("--esModuleInterop")
        .arg("--skipLibCheck")
        .arg("--strict")
        .status()
        .expect("failed to invoke tsc");

    if !status.success() {
        panic!("tsc compilation of shim/runner.ts failed");
    }

    println!("cargo:rerun-if-changed={}", shim_ts.display());
}
