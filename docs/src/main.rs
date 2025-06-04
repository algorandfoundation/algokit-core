use mdbook::MDBook;
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // First, build the mdBook using the Rust API
    println!("Building mdBook documentation...");

    // Load the mdBook from the docs directory
    let mut book = MDBook::load("docs")?;
    let output_dir = Path::new("../target/doc");

    // Set the output directory
    book.config.build.build_dir = output_dir.to_path_buf();

    // Build the book
    book.build()?;

    // Then generate API documentation to a temporary location to move to the docs directory
    println!("Generating API documentation...");
    let cargo_doc = Command::new("cargo")
        .args(&[
            "doc",
            "-p",
            "algokit_transact",
            "-p",
            "algokit_transact_ffi",
            "-p",
            "ffi_macros",
            "-p",
            "uniffi-bindgen",
            "--no-deps",
            "--target-dir",
            "target/temp_cargo",
        ])
        .status()?;

    if !cargo_doc.success() {
        return Err("Failed to generate API documentation".into());
    }

    // Copy API documentation into the docs output
    let api_source = Path::new("target/temp_cargo/doc");
    let api_target = Path::new("target/doc/api");

    if api_source.exists() {
        println!("Copying API documentation...");
        copy_dir_all(api_source, &api_target)?;

        // Clean up temporary directory
        let _ = fs::remove_dir_all("target/temp_cargo");
    }

    println!("Documentation generated successfully in target/doc");
    println!("  - Main docs: target/doc/index.html");
    println!("  - API docs accessible from main navigation");
    Ok(())
}

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> std::io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}
