// swift-tools-version: 6.0
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
  name: "AlgoKitUtils",
  platforms: [.iOS(.v15), .macOS(.v12), .macCatalyst(.v15)],
  products: [
    .library(name: "AlgoKitTransact", targets: ["AlgoKitTransact"]),
    .library(name: "AlgoKitCrypto", targets: ["AlgoKitCrypto"]),
    .library(name: "AlgoKitUtils", targets: ["AlgoKitUtils"]),
  ],
  dependencies: [
    .package(url: "https://github.com/pebble8888/ed25519swift.git", from: "1.2.7")
  ],
  targets: [
    .binaryTarget(
      name: "algokit_transactFFI",
      url: "https://github.com/algorandecosystem/algokit-core/releases/download/swift-v0.0.0-test/algokit_transact.xcframework.zip",
      checksum: "b5d7d4aa6f53c274eb58a20d2a775e0b0ca1af3cd07c313899eda557e65ef118"
    ),
    .binaryTarget(
      name: "algokit_cryptoFFI",
      url: "https://github.com/algorandecosystem/algokit-core/releases/download/swift-v0.0.0-test/algokit_crypto.xcframework.zip",
      checksum: "c1505d388e92b421f94492116a0e97319ac374a95f41e23c4ece580e1866a541"
    ),

    .target(
      name: "AlgoKitTransactFFI",
      dependencies: ["algokit_transactFFI"],
      path: "Sources/AlgoKitTransactFFI"
    ),
    .target(
      name: "AlgoKitCryptoFFI",
      dependencies: ["algokit_cryptoFFI"],
      path: "Sources/AlgoKitCryptoFFI"
    ),

    .target(
      name: "AlgoKitTransact",
      dependencies: ["AlgoKitTransactFFI"],
      path: "Sources/AlgoKitTransact"
    ),
    .target(
      name: "AlgoKitCrypto",
      dependencies: ["AlgoKitCryptoFFI"],
      path: "Sources/AlgoKitCrypto"
    ),
    .target(
      name: "AlgoKitUtils",
      dependencies: ["AlgoKitTransact", "AlgoKitCrypto"],
      path: "Sources/AlgoKitUtils"
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
    .testTarget(
      name: "AlgoKitCryptoTests",
      dependencies: ["AlgoKitCrypto"]
    ),
  ]
)