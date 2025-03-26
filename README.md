# Dijkstra's Algorithm in Rust

This implementation of Dijkstra's algorithm is inspired by [TheAlgorithms' Rust version](https://github.com/TheAlgorithms/Rust/blob/master/src/graph/dijkstra.rs) and fully optimized for WebAssembly (WASM) compatibility.

## Usage 🛠️

Ensure that you have `wasm-pack` installed:

```bash
cargo install wasm-pack
```

Next, build the project with the following command:

```bash
wasm-pack build --target web --release
```

Once the build process is complete, you can include the generated `pkg` folder in your project.

> [!IMPORTANT]
> This implementation is in super alpha state and must be improved etc etc.

> [!CAUTION]
> The `examples/` folder is outdated!
