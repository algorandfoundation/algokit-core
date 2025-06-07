# build.py
import shutil
from pathlib import Path


def build():
    # Copy dynamic libraries to package directory
    lib_dir = Path("../../../target/release")
    target_dir = Path("algokit_utils")

    for dylib in lib_dir.glob("libalgokit_utils_ffi.dylib"):
        _ = shutil.copy2(dylib, target_dir)

    uniffi_file = target_dir / "algokit_utils_ffi.py"
    with uniffi_file.open("r") as file:
        content = file.read()

    content = content.replace("from .algokit_transact_ffi", "from algokit_transact")
    with uniffi_file.open("w") as file:
        _ = file.write(content)


if __name__ == "__main__":
    build()
