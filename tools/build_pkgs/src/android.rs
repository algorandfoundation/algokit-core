use crate::{Package, get_repo_root, run};
use color_eyre::eyre::Result;

pub fn build(package: &Package) -> Result<()> {
    let gradle_root_dir = get_repo_root()
        .join("packages")
        .join("android")
        .join(package.to_string());

    let so_file_output_dir = gradle_root_dir.join("src").join("main").join("jniLibs");
    let kotlin_out_dir = gradle_root_dir.join("src").join("main").join("kotlin");

    // Build for Android targets
    let cargo_build_cmd = format!(
        "cargo ndk -o {} --manifest-path {} -t armeabi-v7a -t arm64-v8a -t x86_64 build --release",
        so_file_output_dir.display(),
        package.crate_manifest().display()
    );

    run(&cargo_build_cmd, None, None)?;

    // Build for host platform (for running unit tests)
    println!("Building for host platform to enable unit tests...");
    let host_build_cmd = format!(
        "cargo build --manifest-path {} --release",
        package.crate_manifest().display()
    );
    run(&host_build_cmd, None, None)?;

    // Copy host library to test resources so JNA can find it
    let test_resources_dir = gradle_root_dir.join("src").join("test").join("resources");

    std::fs::create_dir_all(&test_resources_dir)?;

    let host_dylib = package.dylib(None);
    let test_lib_dest = test_resources_dir.join(host_dylib.file_name().unwrap());

    std::fs::copy(&host_dylib, &test_lib_dest)?;
    println!(
        "Copied host library from {} to {} for unit tests",
        host_dylib.display(),
        test_lib_dest.display()
    );

    if kotlin_out_dir.exists() {
        std::fs::remove_dir_all(&kotlin_out_dir)?;
    }

    run(
        &format!(
            "cargo run -p uniffi-bindgen generate --library {} --language kotlin --out-dir {}",
            package.dylib(Some("aarch64-linux-android")).display(),
            kotlin_out_dir.display()
        ),
        None,
        None,
    )?;

    run(
        "sh -c './gradlew assembleRelease'",
        Some(&gradle_root_dir),
        None,
    )?;

    Ok(())
}
