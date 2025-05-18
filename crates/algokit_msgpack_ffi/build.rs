fn main() {
    // With proc macros, we don't need to generate scaffolding from UDL files

    // Ensure rebuilds happen when files change
    println!("cargo:rerun-if-changed=src/lib.rs");
}
