const releaseUtils = require("../../../utils/semantic-release.cjs");

module.exports = releaseUtils.getConfig({
  language: "python",
  package_name: "algod_api",
  assets: ["../../../artifacts/algokit_algod_api*.whl"],
});
