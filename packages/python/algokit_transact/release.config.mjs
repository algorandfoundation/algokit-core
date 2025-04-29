export default {
  branches: [{ name: "main", prerelease: "alpha" }],
  repositoryUrl: "https://github.com/algorandfoundation/algokit-core",
  tagFormat: "python/algokit_transact@${version}",
  plugins: [
    [
      "semantic-release-scope-filter",
      {
        scopes: [
          "python", // A change to the python build process that impacts this package
          "python/algokit_transact", // A change that only affects this python package
          "algokit_transact", // A change made to algokit_transact crate
          "algokit_transact_ffi", // A change made to algokit_transact_ffi crate
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
        assets: ["../../../artifacts/*-wheel/**/*"],
      },
    ],
    "semantic-release-gha-output",
  ],
};
