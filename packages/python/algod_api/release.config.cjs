const releaseUtils = require("../../../utils/semantic-release.cjs");

module.exports = releaseUtils.getConfig({
  language: "python",
  packageName: "algod_api",
  isNative: true,
  assets: ["../../../artifacts/algokit_algod_api*.whl"],
});
