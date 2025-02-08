#! /usr/bin/env python3

import subprocess
import select
import sys
import os


def run(command: str):
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
elif build_mode == "py":
    run("maturin build")
else:
    run("cargo --color always build --features ffi_uniffi")
