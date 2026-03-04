// swift-tools-version: 6.0
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
  name: "AlgoKitAlgo25",
  products: [
    // Products define the executables and libraries a package produces, making them visible to other packages.
    .library(
      name: "AlgoKitAlgo25",
      targets: ["AlgoKitAlgo25"])
  ],
  targets: [
    // Targets are the basic building blocks of a package, defining a module or a test suite.
    // Targets can depend on other targets in this package and products from dependencies.
    .binaryTarget(
      name: "algokit_algo25FFI",
      path: "Frameworks/algokit_algo25.xcframework"
    ),
    .target(
      name: "AlgoKitAlgo25",
      dependencies: ["algokit_algo25FFI"],
      path: "Sources/AlgoKitAlgo25"
    ),
    .testTarget(
      name: "AlgoKitAlgo25Tests",
      dependencies: [
        "AlgoKitAlgo25"
      ],
    ),
  ]
)
