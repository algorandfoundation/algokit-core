const releaseUtils = require("../../../utils/semantic-release.cjs");

const config = releaseUtils.getConfig({
  language: "typescript",
  package_name: "algokit_transact",
});

config.plugins = [
  ...config.plugins,
  ["@semantic-release/npm", { npmPublish: false }],
];

module.exports = config;
