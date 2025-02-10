#! /usr/bin/env python3

import subprocess
import select
import sys
import os
import shutil


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
else:
    run("cargo --color always build --features ffi_uniffi")

if build_mode == "swift":
    run(
        "cargo --color always run -p uniffi-bindgen generate --no-format --library ../../target/debug/libalgo_models_ffi.dylib --language swift --out-dir ../../target/debug/swift/algo_models"
    )

    run_cmd = (
        "xcodebuild -create-xcframework -library target/debug/libalgo_models_ffi.dylib"
    )

    targets = [
        "aarch64-apple-ios-sim",
        "aarch64-apple-ios",
        "aarch64-apple-ios-macabi",
        # "x86_64-apple-ios",
        # "x86_64-apple-ios-macabi",
    ]
    for target in targets:
        run(f"rustup target add {target}")
        run(f"cargo --color always build --features ffi_uniffi --target {target}")
        run_cmd += f" -library target/{target}/debug/libalgo_models_ffi.dylib"

    run_cmd += " -headers target/debug/swift/algo_models/ -output target/debug/algo_models.xcframework"
    os.rename(
        "../../target/debug/swift/algo_models/algo_modelsFFI.modulemap",
        "../../target/debug/swift/algo_models/module.modulemap",
    )

    if os.path.exists("../../target/debug/algo_models.xcframework"):
        shutil.rmtree("../../target/debug")

    run(run_cmd, cwd="../../")
