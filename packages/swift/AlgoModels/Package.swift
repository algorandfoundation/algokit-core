// swift-tools-version: 6.0
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
    name: "AlgoModels",
    products: [
        // Products define the executables and libraries a package produces, making them visible to other packages.
        .library(
            name: "AlgoModels",
            targets: ["AlgoModels"])
    ],
    targets: [
        // Targets are the basic building blocks of a package, defining a module or a test suite.
        // Targets can depend on other targets in this package and products from dependencies.
        .binaryTarget(
            name: "algo_modelsFFI",
            path: "Frameworks/algo_models.xcframework"
        ),
        .target(
            name: "AlgoModels",
            dependencies: ["algo_modelsFFI"],
            path: "Sources/AlgoModels"
        ),
        .testTarget(
            name: "AlgoModelsTests",
            dependencies: ["AlgoModels"]
        ),
    ]
)
