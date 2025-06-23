use std::{collections::HashMap, fmt::Display, path::Path, process::Output};

use color_eyre::eyre::{Context, Result};

use crate::{Package, get_repo_root, run};

enum WasmPackTarget {
    Web,
    Bundler,
}

impl Display for WasmPackTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Web => f.write_str("web"),
            Self::Bundler => f.write_str("bundler"),
        }
    }
}

enum WasmPackMode {
    Esm,
    Cjs,
    Wasm2js,
}

impl Display for WasmPackMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Esm => f.write_str("esm"),
            Self::Cjs => f.write_str("cjs"),
            Self::Wasm2js => f.write_str("wasm2js"),
        }
    }
}

fn wasm_pack(package: &Package, target: &WasmPackTarget, dir: &Path) -> Result<Output> {
    let crate_name = package.crate_name();
    let command = format!(
        "bunx wasm-pack build --out-dir ../../packages/typescript/{package}/pkg --mode normal --release --target {target} ../../../crates/{crate_name} --no-default-features --features ffi_wasm"
    );

    let mut env_vars = HashMap::new();
    env_vars.insert("RUSTFLAGS".to_string(), "-C strip=symbols".to_string());
    run(&command, Some(dir), Some(env_vars))
}

fn pack_and_bundle(package: &Package, mode: &WasmPackMode, dir: &Path) -> Result<Output> {
    run("bun install", Some(dir), None)?;
    match mode {
        WasmPackMode::Esm => wasm_pack(package, &WasmPackTarget::Web, dir),
        WasmPackMode::Cjs => wasm_pack(package, &WasmPackTarget::Web, dir),
        WasmPackMode::Wasm2js => {
            let output = wasm_pack(package, &WasmPackTarget::Bundler, dir)?;

            run(
                &format!(
                    "bunx wasm2js -O pkg/{package}_ffi_bg.wasm -o pkg/{package}_ffi_bg.wasm.js",
                ),
                Some(dir),
                None,
            )?;

            // Replace references to the wasm file with the wasm2js file
            [".js", "_bg.js"]
                .iter()
                .map(|ext| {
                    let file = dir.join("pkg").join(format!("{package}_ffi{ext}"));

                    let content = std::fs::read_to_string(&file)?;
                    std::fs::write(
                        file,
                        content.replace(
                            format!("{package}_ffi_bg.wasm").as_str(),
                            format!("{package}_ffi_bg.wasm.js").as_str(),
                        ),
                    )
                })
                .collect::<Result<Vec<_>, _>>()?;

            // When decoding, rust is passing numbers when it should be BigInt
            // Could be issue in wasm-bindgen or serde, but for now this fixes it
            let bg_js = dir.join("pkg").join(format!("{package}_ffi_bg.js"));
            let content = std::fs::read_to_string(&bg_js)?;
            std::fs::write(
                bg_js,
                content.replace(
                    "BigInt.asUintN(64, arg0)",
                    "BigInt.asUintN(64, BigInt(arg0))",
                ),
            )?;

            Ok(output)
        }
    }?;

    run(
        &format!("bunx rollup -c rollup.config.{mode}.mjs"),
        Some(dir),
        None,
    )
}

pub fn build(package: &Package) -> Result<()> {
    let dir = get_repo_root()
        .join("packages/typescript")
        .join(package.to_string());

    // Ensure the directory exists
    if !dir.exists() {
        std::fs::create_dir_all(&dir).context("Failed to create directory")?;
    }

    // Clean up previous builds
    for subdir in ["pkg", "dist", "types"] {
        let path = dir.join(subdir);
        if path.exists() {
            std::fs::remove_dir_all(&path).context("Failed to remove old build directory")?;
        }
    }

    // Build for each mode
    pack_and_bundle(package, &WasmPackMode::Wasm2js, &dir).context("Failed to build wasm2js")?;
    pack_and_bundle(package, &WasmPackMode::Esm, &dir).context("Failed to build esm")?;
    pack_and_bundle(package, &WasmPackMode::Cjs, &dir).context("Failed to build cjs")?;

    // Copy the type definitions
    let src = dir.join("pkg").join(format!("{package}_ffi.d.ts"));
    let dest = dir.join("dist").join("index.d.ts");
    std::fs::copy(src, dest).context("Failed to copy type definitions")?;

    Ok(())
}
