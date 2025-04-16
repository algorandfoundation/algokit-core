import * as path from "path";

const jsonString = await Bun.file(
  path.join(__dirname, "../../../../crates/algokit_transact_ffi/test_data.json"),
).text();

export const testData = JSON.parse(jsonString, (_, value) => {
  if (Array.isArray(value) && value.every((n) => typeof n === "number")) {
    return new Uint8Array(value);
  }

  if (
    typeof value === "number" &&
    ["fee", "amount", "firstValid", "lastValid"].includes(_)
  ) {
    return BigInt(value);
  }

  return value;
});
