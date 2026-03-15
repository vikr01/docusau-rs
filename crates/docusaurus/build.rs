use std::path::{Path, PathBuf};

use oxc_allocator::Allocator;
use oxc_codegen::Codegen;
use oxc_parser::Parser;
use oxc_semantic::SemanticBuilder;
use oxc_span::SourceType;
use oxc_transformer::{Module, TransformOptions, Transformer};

fn transpile_ts(path: &Path, source: &str) -> String {
    let allocator = Allocator::default();
    let source_type = SourceType::from_path(path).expect("unrecognised source type");

    let parse = Parser::new(&allocator, source, source_type).parse();
    assert!(parse.errors.is_empty(), "parse errors in shim: {:?}", parse.errors);

    let mut program = parse.program;
    let scoping = SemanticBuilder::new().build(&program).semantic.into_scoping();

    let mut opts = TransformOptions::default();
    opts.env.module = Module::CommonJS;

    let result = Transformer::new(&allocator, path, &opts)
        .build_with_scoping(scoping, &mut program);
    assert!(result.errors.is_empty(), "transform errors in shim: {:?}", result.errors);

    Codegen::new().build(&program).code
}

fn main() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let workspace_root = manifest_dir
        .parent()
        .expect("crate parent")
        .parent()
        .expect("workspace root");

    let shim_ts = workspace_root.join("shim").join("runner.ts");
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());

    let source = std::fs::read_to_string(&shim_ts)
        .unwrap_or_else(|e| panic!("cannot read {}: {e}", shim_ts.display()));

    let js = transpile_ts(&shim_ts, &source);

    std::fs::write(out_dir.join("runner.js"), js)
        .expect("cannot write runner.js to OUT_DIR");

    println!("cargo:rerun-if-changed={}", shim_ts.display());
}
