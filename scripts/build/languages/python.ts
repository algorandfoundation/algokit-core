import { getCrateNanme, run } from "..";

export async function buildPython(crate: string) {
  await run(`maturin build -m crates/${crate}_ffi/Cargo.toml`);
}
