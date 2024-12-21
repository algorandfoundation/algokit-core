#! /usr/bin/env python3

import subprocess
import select
import sys
import shutil
import os


def run(command):
    """
    Run a subprocess and print stdout and stderr in real-time

    Args:
        command (list): Command and arguments as a list (e.g., ["ls", "-la"])
    """
    print(f"Running '{command}'")
    process = subprocess.Popen(
        command.split(" "),
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
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
else:
    run("cargo --color always build --release --features ffi_uniffi")

if build_mode == "py":
    run(
        "cargo --color always run -p uniffi-bindgen generate --library ../../target/release/libalgo_models_ffi.dylib --language python --out-dir tests/py"
    )

    extension = None

    # Determine what the extension of the library is
    extensions = ("dylib", "so", "dll")
    for ext in extensions:
        if os.path.exists(f"../../target/release/libalgo_models_ffi.{ext}"):
            extension = ext
            break

    copy_args = [
        f"../../target/release/libalgo_models_ffi.{extension}",
        f"tests/py/libalgo_models_ffi.{extension}",
    ]

    # Copy the library file using Python's shutil
    shutil.copy2(*copy_args)

    print(f"Copied {copy_args[0]} to {copy_args[1]}")
