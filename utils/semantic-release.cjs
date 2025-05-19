module.exports = {
  /**
   * @param opts {{
   *    language: string;
   *    packageName: string;
   *    assets?: string[];
   *    isNative?: boolean;
   * }}
   *
   * @returns {import('semantic-release').GlobalConfig}
   */
  getConfig: (opts) => {
    const { language, packageName } = opts;
    const assets = opts.assets || [];
    const isNative = opts.isNative || false;

    return {
      branches: ["release", { name: "main", prerelease: "alpha" }],
      repositoryUrl: "https://github.com/algorandfoundation/algokit-core",
      tagFormat: `${language}/${packageName}` + "@${version}",
      plugins: [
        [
          "semantic-release-scope-filter",
          {
            scopes: [
              language, // A change to the language's build process
              `${language}/${packageName}`, // A change that only affects a specific package for a specific language
              packageName, // A change made to the package regardless of language
              ...(!isNative ? [`${packageName}_ffi`] : []), // A change made to ffi crate
            ],
            filterOutMissingScope: false, // Assume any commit without a scope affects this package
          },
        ],
        [
          "@semantic-release/commit-analyzer",
          {
            preset: "conventionalcommits",
            releaseRules: [
              {
                type: "build",
                release: "patch",
              },
              {
                type: "chore",
                release: "patch",
              },
            ],
          },
        ],
        [
          "@semantic-release/release-notes-generator",
          {
            preset: "conventionalcommits",
            presetConfig: {
              types: [
                {
                  type: "feat",
                  section: "Features",
                },
                {
                  type: "fix",
                  section: "Bug Fixes",
                },
                {
                  type: "build",
                  section: "Dependencies and Other Build Updates",
                  hidden: false,
                },
              ],
            },
          },
        ],
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
