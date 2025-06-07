import path from "path";
import { spawn } from "child_process";
import { resolve } from "path";
import { parseArgs } from "util";
import { buildPython } from "./languages/python.ts";
import { buildSwift } from "./languages/swift.ts";
import { buildTypescript } from "./languages/typescript.ts";

export const REPO_ROOT = path.resolve(__dirname, "../../");
process.chdir(REPO_ROOT);

export function toPascalCase(string: string): string {
  return string
    .replaceAll("_", " ")
    .split(" ")
    .map((word) => word.charAt(0).toUpperCase() + word.slice(1))
    .join("");
}

export function run(command: string, cwd: string | null = null, env: Record<string, string> = {}): Promise<void> {
  return new Promise<void>((resolvePromise) => {
    console.log(`Running '${command}'`);

    const args = command.split(" ");
    const subProcess = spawn(args[0], args.slice(1), {
      cwd: cwd || resolve(__dirname, "../../"),
      stdio: ["ignore", "pipe", "pipe"],
      env: { ...process.env, ...env },
      shell: true,
    });

    if (subProcess.stdout) {
      subProcess.stdout.on("data", (data) => {
        process.stdout?.write(data);
      });
    }

    if (subProcess.stderr) {
      subProcess.stderr.on("data", (data) => {
        process.stderr?.write(data);
      });
    }

    subProcess.on("close", (code) => {
      if (code !== 0) {
        process.exit(code || 1);
      }

      resolvePromise();
    });
  });
}

const languages = {
  python: buildPython,
  swift: buildSwift,
  typescript: buildTypescript,
};

const crates = ["algokit_transact", "algokit_utils"];

const { positionals } = parseArgs({
  allowPositionals: true,
});

if (positionals.length !== 2) {
  throw new Error("Usage: bun scripts/build <crate> <language>");
}

const [crate, language] = positionals;

if (language !== "all" && !Object.keys(languages).includes(language)) {
  throw new Error("Language must be one of: all, " + Object.keys(languages).join(", "));
}

if (!crates.includes(crate)) {
  throw new Error("Crate must be one of: " + crates.join(", "));
}

if (language === "all") {
  await Promise.all(
    Object.keys(languages).map(async (language) => {
      if (language === "swift") return; // temporarily remove swift from "all" while we focus on python and typescript
      await languages[language](crate);
    }),
  );
} else {
  await languages[language](crate);
}
