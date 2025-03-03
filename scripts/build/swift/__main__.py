# pyright: reportUnusedCallResult=false
from .. import run, crate, to_pascal_case

import itertools
import os
import shutil

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

cargo_build_cmd = f"cargo --color always build --manifest-path crates/{crate}_ffi/Cargo.toml --features ffi_uniffi"

all_targets = list(itertools.chain.from_iterable(fat_targets.values())) + targets
for target in all_targets:
    run(f"rustup target add {target}")
    cargo_build_cmd += f" --target {target}"

run(cargo_build_cmd)
run(
    f"cargo --color always run -p uniffi-bindgen generate --no-format --library target/aarch64-apple-darwin/debug/lib{crate}_ffi.dylib --language swift --out-dir target/debug/swift/{crate}"
)

create_xcf_cmd = "xcodebuild -create-xcframework"
for target in targets:
    create_xcf_cmd += f" -library target/{target}/debug/lib{crate}_ffi.dylib -headers target/debug/swift/{crate}/"

for fat_target_name in fat_targets:
    lib_paths: list[str] = []
    for target in fat_targets[fat_target_name]:
        lib_paths.append(f"target/{target}/debug/lib{crate}_ffi.dylib")

    run(
        f"lipo -create {' '.join(lib_paths)} -output target/debug/lib{crate}_ffi-{fat_target_name}.dylib",
    )
    create_xcf_cmd += f" -library target/debug/lib{crate}_ffi-{fat_target_name}.dylib -headers target/debug/swift/{crate}/"

swift_package = to_pascal_case(crate)
create_xcf_cmd += f" -output packages/swift/{swift_package}/Frameworks/{crate}.xcframework"

if os.path.exists(f"packages/swift/{swift_package}/Frameworks/{crate}.xcframework"):
    shutil.rmtree(f"packages/swift/{swift_package}/Frameworks/{crate}.xcframework")

# xcframework needs the modulemap to be named module.modulemap
os.rename(
    f"target/debug/swift/{crate}/{crate}FFI.modulemap",
    f"target/debug/swift/{crate}/module.modulemap",
)

# replace var with let to resolve swift concurrency issues
# I believe this is fixed in https://github.com/mozilla/uniffi-rs/pull/2294
# The above PR is available in uniffi-rs 0.29.0, but we won't be updating until
# Nord generators (i.e. Golang) are updated to use 0.29.0
with open(f"target/debug/swift/{crate}/{crate}.swift", "r") as file:
    content = file.read()

content = content.replace(
    "private var initializationResult", "private let initializationResult"
)

with open(f"target/debug/swift/{crate}/{crate}.swift", "w") as file:
    file.write(content)


run(create_xcf_cmd)

shutil.move(
    f"target/debug/swift/{crate}/{crate}.swift",
    f"packages/swift/{swift_package}/Sources/{swift_package}/{swift_package}.swift",
)
print(f"Updated {swift_package} in packages/swift/{swift_package}")
