import { run } from "..";

export async function buildPython(crate: string) {
  await run(`maturin build`, `packages/python/${crate}`);
}
