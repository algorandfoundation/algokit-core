import { run } from "..";
import * as fs from "fs";
import { $ } from "bun";
export async function buildGo(crate: string) {
  const target = (await run("rustc -vV")).split("host: ")[1].split("\n")[0];
  await run(`cargo build --target ${target} --package ${crate}_ffi`);
  await run(
    `cargo bin uniffi-bindgen-go --no-format --out-dir packages/go/${crate}/ --library target/debug/lib${crate}_ffi.a`,
  );

  const newDir = `packages/go/${crate}/${target}`;
  await $`rm -rf ${newDir}`;

  await $`mv packages/go/${crate}/${crate}_ffi ${newDir}`;

  const content = fs.readFileSync(`${newDir}/${crate}_ffi.go`, "utf-8");

  const newContent = content.replace(
    `package ${crate}_ffi`,
    `package ${target}`,
  );

  fs.writeFileSync(`${newDir}/${crate}_ffi.go`, newContent, "utf-8");
}
