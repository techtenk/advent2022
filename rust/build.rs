
// build.rs
fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo:rerun-if-changed=src/day11/day11.c");
    // Use the `cc` crate to build a C file and statically link it.
    cc::Build::new()
        .file("src/day11/day11.c")
        .compile("libday11.a");
}