use color_eyre::eyre::{Context, Result};

use crate::{Package, run};

/// Destination SwiftPM package directory for all Swift build outputs.
const SWIFT_PACKAGE_DIR: &str = "packages/swift/AlgoKitUtils";

pub fn build(package: &Package) -> Result<()> {
    let crate_name = package.crate_name();
    let targets = ["aarch64-apple-ios"];

    let fat_targets: Vec<(&str, [&str; 2])> = vec![
        ("macos", ["x86_64-apple-darwin", "aarch64-apple-darwin"]),
        ("ios-sim", ["x86_64-apple-ios", "aarch64-apple-ios-sim"]),
        (
            "catalyst",
            ["x86_64-apple-ios-macabi", "aarch64-apple-ios-macabi"],
        ),
    ];

    let mut cargo_build_cmd = format!(
        "cargo --color always build --manifest-path crates/{}/Cargo.toml",
        crate_name
    );

    let all_targets: Vec<&str> = fat_targets
        .iter()
        .flat_map(|(_, targets)| targets.iter())
        .chain(targets.iter())
        .cloned()
        .collect();

    for target in all_targets {
        cargo_build_cmd.push_str(&format!(" --target {}", target));
    }

    run(&cargo_build_cmd, None, None).context("Failed to build the package")?;

    run(
        &format!(
            "cargo --color always run -p uniffi-bindgen generate --no-format --library target/aarch64-apple-darwin/debug/lib{crate_name}.a --language swift --out-dir target/debug/swift/{package}",
        ),
        None,
        None,
    )?;

    let mut create_xcf_cmd = "xcodebuild -create-xcframework".to_string();
    for target in targets {
        create_xcf_cmd.push_str(&format!(
            " -library target/{target}/debug/lib{crate_name}.a -headers target/debug/swift/{package}/",
        ));
    }

    for (fat_target_name, targets) in &fat_targets {
        let lib_paths: Vec<String> = targets
            .iter()
            .map(|target| format!("target/{target}/debug/lib{crate_name}.a"))
            .collect();
        let lib_paths_str = lib_paths.join(" ");
        run(
            &format!(
                "lipo -create {lib_paths_str} -output target/debug/lib{crate_name}-{fat_target_name}.a"
            ),
            None,
            None,
        )?;

        create_xcf_cmd.push_str(&format!(
            " -library target/debug/lib{crate_name}-{fat_target_name}.a -headers target/debug/swift/{package}/",
        ));
    }

    let swift_module = package.swift_module_name();
    let binding_module = format!("{swift_module}FFI");

    create_xcf_cmd += &format!(" -output {SWIFT_PACKAGE_DIR}/Frameworks/{package}.xcframework");

    let xcframework_path = format!("{SWIFT_PACKAGE_DIR}/Frameworks/{package}.xcframework");

    if std::path::Path::new(&xcframework_path).exists() {
        std::fs::remove_dir_all(&xcframework_path)
            .context("Failed to remove existing xcframework directory")?;
        println!(
            "Removed existing xcframework directory: {}",
            xcframework_path
        );
    } else {
        println!(
            "No existing xcframework directory to remove: {}",
            xcframework_path
        );
    }

    // xcframework needs the modulemap to be named module.modulemap
    let modulemap_path = format!("target/debug/swift/{package}/{package}FFI.modulemap");
    let new_modulemap_path = format!("target/debug/swift/{package}/module.modulemap");
    std::fs::rename(modulemap_path, new_modulemap_path)
        .context("Failed to rename modulemap file")?;

    // replace var with let to resolve swift concurrency issues
    // I believe this is fixed in https://github.com/mozilla/uniffi-rs/pull/2294
    // The above PR is available in uniffi-rs 0.29.0, but we won't be updating until
    // Nord generators (i.e. Golang) are updated to use 0.29.0
    let content = std::fs::read_to_string(format!("target/debug/swift/{package}/{package}.swift"))
        .context("Failed to read Swift file")?;

    let updated_content = content.replace(
        "private var initializationResult",
        "private let initializationResult",
    );

    std::fs::write(
        format!("target/debug/swift/{package}/{package}.swift"),
        updated_content,
    )
    .context("Failed to write updated Swift file")?;

    run(&create_xcf_cmd, None, None).context("Failed to create xcframework")?;

    nest_xcframework_headers(&xcframework_path, &format!("{package}FFI"))
        .context("Failed to nest xcframework headers under per-module subdir")?;

    std::fs::rename(
        format!("target/debug/swift/{package}/{package}.swift"),
        format!("{SWIFT_PACKAGE_DIR}/Sources/{binding_module}/{binding_module}.swift"),
    )
    .context("Failed to rename Swift file")?;

    if matches!(package, Package::Transact) {
        std::fs::copy(
            format!("crates/{crate_name}/test_data.json"),
            format!("{SWIFT_PACKAGE_DIR}/Tests/AlgoKitTransactTests/Resources/test_data.json"),
        )
        .context("Failed to copy test data file")?;
    }

    Ok(())
}

/// Move each slice's `module.modulemap` and `<module>.h` into a
/// `Headers/<module>/` subdir so multiple xcframeworks don't collide on the
/// `module.modulemap` filename when linked into the same Apple app.
fn nest_xcframework_headers(xcframework_path: &str, module_name: &str) -> Result<()> {
    for slice in std::fs::read_dir(xcframework_path)? {
        let slice = slice?;
        if !slice.file_type()?.is_dir() {
            continue;
        }
        let headers_dir = slice.path().join("Headers");
        if !headers_dir.exists() {
            continue;
        }
        let nested_dir = headers_dir.join(module_name);
        std::fs::create_dir_all(&nested_dir)?;

        for entry in std::fs::read_dir(&headers_dir)? {
            let entry = entry?;
            let name = entry.file_name();
            if name == std::ffi::OsStr::new(module_name) {
                continue;
            }
            if name != std::ffi::OsStr::new("module.modulemap")
                && name != std::ffi::OsStr::new(&format!("{module_name}.h"))
            {
                continue;
            }
            std::fs::rename(entry.path(), nested_dir.join(&name))?;
        }
    }
    Ok(())
}
