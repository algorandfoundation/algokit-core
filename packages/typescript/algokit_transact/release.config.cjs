const releaseUtils = require("../../../utils/semantic-release.cjs");

const config = releaseUtils.getConfig({
  language: "typescript",
  packageName: "algokit_transact",
});

config.plugins = [...config.plugins, ["@semantic-release/npm", { npmPublish: true }]];

module.exports = config;
