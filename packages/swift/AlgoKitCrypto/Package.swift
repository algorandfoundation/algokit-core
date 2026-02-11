// swift-tools-version: 6.0
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
  name: "AlgoKitCrypto",
  products: [
    // Products define the executables and libraries a package produces, making them visible to other packages.
    .library(
      name: "AlgoKitCrypto",
      targets: ["AlgoKitCrypto"])
  ],
  targets: [
    // Targets are the basic building blocks of a package, defining a module or a test suite.
    // Targets can depend on other targets in this package and products from dependencies.
    .binaryTarget(
      name: "algokit_cryptoFFI",
      path: "Frameworks/algokit_crypto.xcframework"
    ),
    .target(
      name: "AlgoKitCrypto",
      dependencies: ["algokit_cryptoFFI"],
      path: "Sources/AlgoKitCrypto"
    ),
    .testTarget(
      name: "AlgoKitCryptoTests",
      dependencies: [
        "AlgoKitCrypto"
      ],
    ),
  ]
)
