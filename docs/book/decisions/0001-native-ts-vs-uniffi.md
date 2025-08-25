---
status: accepted
date: 2025-08-15
decision-makers: David Rojas
consulted: MakerX & Algorand Foundation AlgoKit engineering team
---

# Native TypeScript vs Uniffi Bindings for AlgoKit Core Packages

## Context and Problem Statement

One of the languages that AlgoKit Core should support is TypeScript. The browser is one of the most popular environments for using AlgoKit, thus it is important that we have first-class support for it. For every language other than TypeScript, we use Uniffi, which uses C ABI bindings for each language via Foreign Function Interfaces (FFIs). C bindings, however, are not available for use in the browser because browsers cannot execute native code. This requires a separate runtime (and potentially ABI) for browsers. The only option for executing *near*-native code in browser is WebAssembly. Rust does support compiling to WebAssembly and there are a lot of popular tools available for Rust and WASM. That being said, supporting WASM requires a separate set of tools for TypeScript bindings and comes with a some implications that are not applicable to other languages.

## Decision Drivers

- The chosen approach should enable API consistency with other languages
- The chosen approach should be reasonable to implement and maintain long term
- The chosen approach should deliver a clear developer experience for TypeScript developers regardless of their environment
- The chosen approach should not negatively impact downstream dependencies

## Considered Options

- Uniffi bindings via uniffi-bindgen-react-native
- Native TypeScript implementation

## Decision Outcome

- Native TypeScript implementation

## Pros and Cons of the Options

### Native TypeScript Implementation

- **Good**: Seamless support for all JavaScript/TypeScript environments
- **Good**: Straightforward packaging and distribution via standard TypeScript tooling
- **Neutral**: If APIs across languages are kept in sync, this may slow down overall velocity of new features
- **Bad**: Potential for API and/or feature divergence from other languages
- **Bad**: Potential for bugs to be introduced unique to TypeScript implementation

### Uniffi Bindings

- **Good**: Ensures consistent API across all languages, including TypeScript
- **Good**: Leverages existing Rust implementation, reducing duplication of effort
- **Bad**: Complex build process for all environments
- **Bad**: React Native and WASM bindings might have different behavior or performance characteristics
- **Bad**: uniffi-bindgen-react-native is relatively immature
- **Bad**: May introduce significant migration (or glue code) from the existing TypeScript AlgoKit Utils library
- **Bad**: WASM boundary crosses are expensive (orders of magnitude more than C FFI)
- **Bad**: WASM importing will impact all downstream dependents
- **Bad**: Mixed support across popular runtimes (i.e Bun)
- **Bad**: A separate package would be required for React Native
- **Bad**: WASM binaries can get quite large and are not tree-shakeable

## More Information

### Uniffi Bindgen React Native

Historically, we have used [wasm_bindgen](https://github.com/wasm-bindgen/wasm-bindgen/) and [wasm-pack](https://github.com/drager/wasm-pack) to generate TypeScript bindings and package core TypeScript libraries. React Native, however, does not currently support WASM out of the box. It also does not support [WeakRefs TC39 proposal](https://github.com/tc39/proposal-weakrefs), which is needed for proper garbage collection of Rust-owned objects. The [uniffi-bindgen-react-native](https://jhugman.github.io/uniffi-bindgen-react-native/) project aims to solve these issues by providing a way to generate React Native bindings via UniFFI and recently added support for WASM as well. Early spikes have demonstrated that this works, but it adds a significant amount of complexity to the build process (see React Native Support below). Additionally, this library is relatively immature compared to the other first and third-party uniffi binding generators we will be using for core.

### WASM Runtime Support

It is possible to get WASM modules working in Node.js and other runtimes such as bun, but how they must be imported and the level of support varies. For example, bun has ran into issues with WASM-based packages that have not been replicated in Node.js. It's also unclear how well other runtimes outside of Node.js support WASM.

### WASM "Coloring"

Due to the fact that WASM modules are binary blobs that can be rather large, they are typically fetched and instantiated asynchronously. This means one of the following must be true:

1. All users of the library must support top-level await (ESM only)
1. The library must have a single entry point that is async (and automatically does the instantiation)
1. All library functions that call into WASM must be async (and each one ensures the WASM module is instantiated)

A common fix to this problem is to inline the WASM binary as a base64 string and decode it at runtime. This works, but it can significantly increase the size of the library.

Furthermore, WASM support in popular bundlers is mixed and any user of the library may need to add additional configuration to their bundler to ensure the WASM module is properly included.

All three of these problems are problems that are inherited down to all dependents of the library. For example, if the entry point to our library is async, all dependents must also use an async entry point. Similarly, bundling decisions made upstream will impact all dependents.

Further reading: https://nickb.dev/blog/recommendations-when-publishing-a-wasm-library/

### WASM Bundle Size

WASM binaries can get quite large, especially when they include multiple features or dependencies. This can lead to significantly larger bundle sizes for applications that depend on the library. Additionally, WASM binaries are not tree-shakeable, meaning that even if only a small portion of the library is used, the entire WASM binary must still be included in the bundle. The initial bundle size can be reduced by async fetching the WASM binary, but that comes with the downsides mentioned above (and a large size is still not ideal for users with slow or metered connections). To properly reduce the WASM bundle size, extra thought needs to be put into essentially every aspect of the Rust implementation such as dependency selection, feature flags, and memory allocators.

Further reading: https://nickb.dev/blog/the-dark-side-of-inlining-and-monomorphization/

### WASM Performance

When using Uniffi or wasm-bindgen, there will always be a small performance hit because these libraries do not use direct memory access and instead serialize data going over the FFI/WASM boundary. For native languages, the impact is small (microseconds), but for WASM, the impact is much larger (milliseconds).

Benchmarks for both WASM and uniffi (python) can be found here: https://github.com/joe-p/uniffi-wasm-playground/blob/main/FINDINGS.md

### React Native Support

Because React Native does not support WASM or WeakRefs, the uniffi-bindgen-react-native library must be used to generate bindings. This library generates TypeScript bindings to Rust bindings to JSI. This means the code path for React Native and other JavaScript runtimes (Node.js, browser, etc) are completely different. This also adds a significant amount of complexity to the build process because turbo modules must be built for each platform. This also requires a completely new set of dependencies for React Native, thus necessitating a separate package.

### Design Impact

All the above issues only apply to WASM and not C ABI FFIs. With Core, however, we are re-using as much code as possible across languages. This means that if we were to use WASM for TypeScript, we would need to ensure that all code paths are compatible with WASM and that the API is consistent across languages. Alternatively, we would need to have seperate code paths just for WASM. Regardless of the approach, it is inevitable that the implications of WASM will impact the overall design of AlgoKit core, including other languages.

### Native TypeScript in Rust Monorepo

It should be noted that native implementation of the logic in TypeScript does not preclude us from using Rust-based tooling for the testing and development of the TypeScript packages. For example, we can use Rust to generate the types for the TypeScript package to ensure consistency with the Rust implementation. We can also leverage WASM bindings for testing to ensure that all languages undergo the same tests.
