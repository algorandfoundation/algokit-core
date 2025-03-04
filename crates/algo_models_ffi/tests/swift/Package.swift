// swift-tools-version: 6.0
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
    name: "AlgoModelsTests",
    dependencies: [
        // Add the local package dependency by specifying its path
        .package(path: "../../../../packages/swift/AlgoModels")

    ],
    targets: [
        .testTarget(
            name: "AlgoModelsTests",
            dependencies: ["AlgoModels"]
        )
    ]
)
