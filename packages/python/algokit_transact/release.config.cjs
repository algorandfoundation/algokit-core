const releaseUtils = require("../../../utils/semantic-release.cjs");

module.exports = releaseUtils.getConfig({
  language: "python",
  packageName: "algokit_transact",
  assets: ["../../../artifacts/*.whl"],
});
