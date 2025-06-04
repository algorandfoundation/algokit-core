use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Move the necessary files to the book directory

    // let source_base = Path::new(".");
    // let target_book = Path::new("book");

    // // Create target directories in book/
    // fs::create_dir_all(target_book.join("research"))?;
    // fs::create_dir_all(target_book.join("decisions"))?;
    // fs::create_dir_all(target_book.join("contributing"))?;

    // // Copy research files
    // copy_if_exists(
    //     &source_base.join("research/glibc_and_musl.md"),
    //     &target_book.join("research/glibc_and_musl.md"),
    // )?;
    // copy_if_exists(
    //     &source_base.join("research/openapi-generators.md"),
    //     &target_book.join("research/openapi_generators.md"),
    // )?;

    Ok(())
}

fn _copy_if_exists(source: &Path, target: &Path) -> Result<(), Box<dyn std::error::Error>> {
    if source.exists() {
        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::copy(source, target)?;
    }
    Ok(())
}
