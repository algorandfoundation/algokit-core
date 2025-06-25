use crate::{Package, get_repo_root, run};
use color_eyre::eyre::{Result, eyre};

pub fn build(package: &Package) -> Result<()> {
    run(
        &format!(
            r#"cargo --color always build --release --manifest-path "{}""#,
            package.crate_manifest().display()
        ),
        None,
        None,
    )?;

    let package_dir = get_repo_root()
        .join("packages")
        .join("python")
        .join(package.to_string());
    let module_dir = package_dir.join(package.to_string());

    run(
        &format!(
            r#"cargo --color always run -p uniffi-bindgen generate --no-format --library "{}" --language python --out-dir "{}""#,
            package.dylib().display(),
            module_dir.display()
        ),
        None,
        None,
    )?;

    std::fs::copy(
        package.dylib(),
        module_dir.join(package.dylib().file_name().unwrap()),
    )?;

    run("poetry install --only build", Some(&package_dir), None)?;

    run("poetry build --format wheel", Some(&package_dir), None)?;

    if cfg!(target_os = "linux") {
        let dist_files: Vec<_> = std::fs::read_dir(package_dir.join("dist"))?
            .filter_map(Result::ok)
            .filter(|entry| entry.path().extension().is_some_and(|ext| ext == "whl"))
            .collect();

        if dist_files.is_empty() {
            return Err(eyre!("No .whl files found in dist directory"));
        } else if dist_files.len() > 1 {
            return Err(eyre!("Multiple .whl files found in dist directory"));
        }

        let wheel_file = dist_files[0].path();

        run(
            &format!(r#"poetry run auditwheel repair "{}""#, wheel_file.display()),
            Some(&package_dir),
            None,
        )?;
    }

    Ok(())
}
