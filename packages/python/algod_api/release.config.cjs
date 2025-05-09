const releaseUtils = require("../../../utils/semantic-release.cjs");

const config = releaseUtils.getConfig({
  language: "python",
  package_name: "algod_api",
});

config.plugins = [...config.plugins, ["@semantic-release/npm", { npmPublish: false }]];

module.exports = config;
