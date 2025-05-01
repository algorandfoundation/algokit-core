import { run } from "..";

export async function buildPython(crate: string) {
  await run(
    `maturin build --no-default-features --features ffi_uniffi`,
    `packages/python/${crate}`
  );
}
