// swift-tools-version: 6.0
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
    name: "SwiftTests",
    targets: [
        // Targets are the basic building blocks of a package, defining a module or a test suite.
        // Targets can depend on other targets in this package and products from dependencies.
        .binaryTarget(
            name: "algo_modelsFFI",
            path: "../../../../target/debug/algo_models.xcframework"
        ),
        .target(
            name: "algo_models",
            dependencies: ["algo_modelsFFI"],
            path: "Sources/algo_models"
        ),
        .executableTarget(
            name: "SwiftTests",
            dependencies: ["algo_models"]),
    ]
)
