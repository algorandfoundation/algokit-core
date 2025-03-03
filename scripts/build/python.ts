import { getCrateNanme, run } from "./index.ts";

const crate = getCrateNanme();
await run(`maturin build -m crates/${crate}_ffi/Cargo.toml`);
