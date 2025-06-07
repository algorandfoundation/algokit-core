import { run } from "..";

export async function buildPython(crate: string) {
  await run(`cargo --color always build --release --manifest-path crates/${crate}_ffi/Cargo.toml`);
  await run(
    `cargo --color always run -p uniffi-bindgen generate --no-format --library target/release/lib${crate}_ffi.dylib --language python --out-dir packages/python/${crate}/${crate}`,
  );

  await run(`poetry build --format wheel`, `packages/python/${crate}`);
}
