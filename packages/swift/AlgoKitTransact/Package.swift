// swift-tools-version: 6.0
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
  name: "AlgoKitTransact",
  products: [
    // Products define the executables and libraries a package produces, making them visible to other packages.
    .library(
      name: "AlgoKitTransact",
      targets: ["AlgoKitTransact"])
  ],
  dependencies: [
    .package(url: "https://github.com/pebble8888/ed25519swift.git", from: "1.2.7")
  ],
  targets: [
    // Targets are the basic building blocks of a package, defining a module or a test suite.
    // Targets can depend on other targets in this package and products from dependencies.
    .binaryTarget(
      name: "algokit_transactFFI",
      path: "Frameworks/algokit_transact.xcframework"
    ),
    .target(
      name: "AlgoKitTransact",
      dependencies: ["algokit_transactFFI"],
      path: "Sources/AlgoKitTransact"
    ),
    .testTarget(
      name: "AlgoKitTransactTests",
      dependencies: [
        "AlgoKitTransact",
        "ed25519swift",
      ],
      resources: [
        .process("Resources/test_data.json")
      ]
    ),
  ]
)
