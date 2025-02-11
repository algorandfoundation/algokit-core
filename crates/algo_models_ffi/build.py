#! /usr/bin/env python3

import subprocess
import select
import sys
import os
import shutil
import itertools


def run(command: str, *, cwd: str | None = None):
    """
    Run a subprocess and print stdout and stderr in real-time

    Args:
        command (list): Command and arguments as a list (e.g., ["ls", "-la"])
    """
    print(f"Running '{command}'")
    process = subprocess.Popen(
        command.split(" "), stdout=subprocess.PIPE, stderr=subprocess.PIPE, cwd=cwd
    )

    # Use select to read from stdout and stderr without blocking
    while True:
        reads = [process.stdout.fileno(), process.stderr.fileno()]
        ret = select.select(reads, [], [])

        for fd in ret[0]:
            if fd == process.stdout.fileno():
                line = process.stdout.readline()
                if line:
                    sys.stdout.buffer.write(line)
                    sys.stdout.buffer.flush()
            if fd == process.stderr.fileno():
                line = process.stderr.readline()
                if line:
                    sys.stderr.buffer.write(line)
                    sys.stderr.buffer.flush()

        if process.poll() is not None:
            break

    if process.wait() != 0:
        sys.exit(process.returncode)


build_mode = "rust"

if len(sys.argv) > 1:
    build_mode = sys.argv[1]

if build_mode == "wasm":
    run(
        "wasm-pack build --target web --out-dir ./tests/js/pkg -- --color always --no-default-features --features ffi_wasm"
    )

    # Remove the generated .gitignore file from the pkg directory
    if os.path.exists("tests/js/pkg/.gitignore"):
        os.remove("tests/js/pkg/.gitignore")
elif build_mode == "py":
    run("maturin build")
elif build_mode == "swift":
    # For now just support Tier 1 & 2 targets: https://doc.rust-lang.org/nightly/rustc/platform-support.html
    #
    # For some targets we need to combine binaries, thus we have targets and fat_targets
    # See https://developer.apple.com/forums/thread/666335
    targets = ["aarch64-apple-ios"]

    fat_targets = {
        "ios-sim": [
            "x86_64-apple-ios",
            "aarch64-apple-ios-sim",
        ],
        "catalyst": [
            "x86_64-apple-ios-macabi",
            "aarch64-apple-ios-macabi",
        ],
        "macos": [
            "x86_64-apple-darwin",
            "aarch64-apple-darwin",
        ],
    }

    cargo_build_cmd = "cargo --color always build --features ffi_uniffi"

    all_targets = list(itertools.chain.from_iterable(fat_targets.values())) + targets
    for target in all_targets:
        run(f"rustup target add {target}")
        cargo_build_cmd += f" --target {target}"

    run(cargo_build_cmd)
    run(
        "cargo --color always run -p uniffi-bindgen generate --no-format --library ../../target/aarch64-apple-darwin/debug/libalgo_models_ffi.dylib --language swift --out-dir ../../target/debug/swift/algo_models"
    )

    create_xcf_cmd = "xcodebuild -create-xcframework"
    for target in targets:
        create_xcf_cmd += f" -library target/{target}/debug/libalgo_models_ffi.dylib"

    for fat_target_name in fat_targets:
        lib_paths: list[str] = []
        for target in fat_targets[fat_target_name]:
            lib_paths.append(f"target/{target}/debug/libalgo_models_ffi.dylib")

        run(
            f"lipo -create {' '.join(lib_paths)} -output target/debug/libalgo_models_ffi-{fat_target_name}.dylib",
            cwd="../../",
        )
        create_xcf_cmd += (
            f" -library target/debug/libalgo_models_ffi-{fat_target_name}.dylib"
        )

    create_xcf_cmd += " -headers target/debug/swift/algo_models/ -output target/debug/algo_models.xcframework"

    if os.path.exists("../../target/debug/algo_models.xcframework"):
        shutil.rmtree("../../target/debug/algo_models.xcframework")

    # xcframework needs the modulemap to be named module.modulemap
    os.rename(
        "../../target/debug/swift/algo_models/algo_modelsFFI.modulemap",
        "../../target/debug/swift/algo_models/module.modulemap",
    )

    # replace var with let to resolve swift concurrency issues
    # I believe this is fixed in https://github.com/mozilla/uniffi-rs/pull/2294
    # The above PR is available in uniffi-rs 0.29.0, but we won't be updating until
    # Nord generators (i.e. Golang) are updated to use 0.29.0
    with open("../../target/debug/swift/algo_models/algo_models.swift", "r") as file:
        content = file.read()

    content = content.replace("private var initializationResult", "private let initializationResult")

    with open("../../target/debug/swift/algo_models/algo_models.swift", "w") as file:
        file.write(content)

    shutil.move(
        "../../target/debug/swift/algo_models/algo_models.swift",
        "build/swift/AlgoModels/Sources/AlgoModels/AlgoModels.swift",
    )

    run(create_xcf_cmd, cwd="../../")
else:
    run("cargo --color always build --features ffi_uniffi")
