# BSim2

BSim2 is a simulator for logical circuits, written in Rust using [Bevy](https://bevyengine.org/).
This is based on [BSim](https://github.com/bnjmn21/bsim).

## Building

For compiling and running a dev build, use the standard `cargo` cli, with the `dev` feature enabled:

```bash
cargo run
```


For compiling a release build, use:

```bash
cargo build --release --no-default-features
```

By default the `dev` feature is enabled, which is used to enable Bevys `dynamic-linking` and `file-watcher` feature.
For releases the `dynamic-linking` is not recommended as it require shipping an additional library,
and the `file-watcher` feature is useless on release builds, and infact won't even compile for WASM-builds.

For building the web version, use:

```bash
trunk build --release --no-default-features
```

or

```bash
trunk build --release --no-default-features
```

to build it and start a webserver running it.
Make sure you have trunk installed, which is available through

```bash
cargo install --locked trunk
```

## Structure

- `./assets` contains images and gltf scene files (`.glb`) used within the program.
- `./build` contains extra files for compiling the project to the different targets.
- `./dist` is the target folder for the web build.
- `./gates.blend` is a blender file containing the meshes for the logic gates.

The rest of the files are like the usual cargo project structure.