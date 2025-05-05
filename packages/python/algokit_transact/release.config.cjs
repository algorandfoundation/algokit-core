const releaseUtils = require("../../../utils/semantic-release.cjs");

module.exports = releaseUtils.getConfig({
  language: "python",
  package_name: "algokit_transact",
  assets: ["../../../artifacts/*.whl"],
});
