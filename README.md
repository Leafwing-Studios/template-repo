# About

This is the Leafwing Studios' template repo, providing a quick, opinionated base for high-quality Bevy game projects (and libraries).
We've shaved the yaks for you!

The licenses here are provided for template purposes: this repository itself is provided under MIT-0.
Feel free to use, hack and adopt this freely: no attribution needed.

## Instructions

### Getting started

[Use this template](https://github.com/Leafwing-Studios/template-repo/generate) by pressing the big green "Use this template" button in the top right corner of [this repo](https://github.com/Leafwing-Studios/template-repo) to create a new repository.

This repository has dynamic linking enabled for much faster incremental compile times.
If you're on Windows, you'll need to use the `nightly` Rust compiler.
Swap by using `rustup default nightly`.

If you are making a game:

1. Enable the features you need from Bevy in `Cargo.toml`.
2. Delete the `examples` folder.
3. Start writing your game. Your logic should be stored in `lib.rs` (and other files that are pulled in from it).
Then, add all of the plugins and build your `App` in `main.rs`.
4. If you only care about your game working on `nightly`, remove `stable` from the `toolchain` field in `.github/workflows/ci.yml`.

If you are making a standalone library:

1. Delete `main.rs` and the `[[bin]]` section of the top-level `Cargo.toml`.
2. Disable `bevy` features: change `default-features` to `false` and disable the `dynamic` feature. This avoids unnecessarily pulling in extra features for your users.

Finally:

1. Rename the lib and bin in `Cargo.toml` (and all imports to ensure your code compiles).
2. Double check that the LICENSE matches your intent.
3. Update this README to match your project, modifying `About`, `Getting Started` and other sections as needed.
4. Consider cleaning up the issue and PR templates found in the `.github` folder to better match your needs.

### Running your game

Use `cargo run`.
This repo is set up to always build with full optimizations, so there's no need for a `--release` flag in most cases.
Dynamic linking is enabled to ensure build times stay snappy.

To run an example, use `cargo run --example_name`, where `example_name` is the file name of the example without the `.rs` extension.
