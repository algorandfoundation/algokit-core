import path from "path";
import { spawn } from "child_process";
import { resolve } from "path";

export const REPO_ROOT = path.resolve(__dirname, "../../");

process.chdir(REPO_ROOT);
export function toPascalCase(string: string): string {
  return string
    .replaceAll("_", " ")
    .split(" ")
    .map((word) => word.charAt(0).toUpperCase() + word.slice(1))
    .join("");
}

export function getCrateNanme(): string {
  if (process.argv[2] == undefined) {
    throw new Error("Crate must be specified as the argument");
  }

  return process.argv[2].replace("_ffi", "");
}

export function run(command: string, cwd: string | null = null): Promise<void> {
  return new Promise<void>((resolvePromise) => {
    console.log(`Running '${command}'`);

    const args = command.split(" ");
    const subProcess = spawn(args[0], args.slice(1), {
      cwd: cwd || resolve(__dirname, "../../"),
      stdio: ["ignore", "pipe", "pipe"],
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
