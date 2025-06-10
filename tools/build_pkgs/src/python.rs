use std::path::PathBuf;

use crate::{Package, run};
use color_eyre::eyre::Result;

pub fn build(package: &Package) -> Result<()> {
    run(
        "maturin build",
        Some(&PathBuf::from(&format!("packages/python/{package}"))),
        None,
    )?;

    Ok(())
}
