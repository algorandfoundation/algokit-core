module.exports = {
  /**
   * @param opts {{
   *    language: string;
   *    package_name: string;
   *    assets?: string[];
   * }}
   *
   * @returns {import('semantic-release').GlobalConfig}
   */
  getConfig: (opts) => {
    const { language, package_name } = opts;
    const assets = opts.assets || [];
    return {
      branches: ["release", { name: "main", prerelease: "alpha" }],
      repositoryUrl: "https://github.com/algorandfoundation/algokit-core",
      tagFormat: `${language}/${package_name}` + "@${version}",
      plugins: [
        [
          "semantic-release-scope-filter",
          {
            scopes: [
              language, // A change to the language's build process
              `${language}/${package_name}`, // A change that only affects a specific package for a specific langauge
              package_name, // A change made to non-ffi crate
              `${package_name}_ffi`, // A change made to ffi crate
            ],
            filterOutMissingScope: false, // Assume any commit without a scope affects this package
          },
        ],
        [
          "@semantic-release/commit-analyzer",
          {
            preset: "angular",
          },
        ],
        "@semantic-release/release-notes-generator",
        [
          "@semantic-release/github",
          {
            assets,
          },
        ],
        "semantic-release-gha-output",
      ],
    };
  },
};
