from .. import run, crate

run(f"maturin build -m crates/{crate}_ffi/Cargo.toml")
