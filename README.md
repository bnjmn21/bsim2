# BSim2

BSim2 is a simulator for logical circuits, written in Rust using [Bevy](https://bevyengine.org/).
This is based on [BSim](https://github.com/bnjmn21/bsim).

## Building

For compiling and running a dev build, use the standard `cargo` cli:

```shell
cargo run
```

For compiling a release build, use:

```shell
cargo build --release --no-default-features
```

The `--no-default-features` is used to disable Bevys `dynamic-linking` feature,
which is not recommended for release builds as it requires shipping a `bevy-dylib` dll.

## Structure

- `./assets` contains images and gltf scene files (`.glb`) used within the program.
- `./build` contains extra files for compiling the project to the different targets.
- `./dist` is the target folder for the web build.
- `./gates.blend` is a blender file containing the meshes for the logic gates.

The rest of the files should be familiar to [rustaceans](https://www.rustacean.net/).
