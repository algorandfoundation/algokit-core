import { run } from "..";

export async function buildTypescript(crate: string) {
  await run("npm i", `packages/typescript/${crate}`);
  await run("npm run build", `packages/typescript/${crate}`);
}
