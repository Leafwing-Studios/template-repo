# About

This is the standard template repository for Leafwing Studios!
It's intended to provide a quick, minimally opinionated base to build high-quality Bevy game projects with.

The licenses here are provided for template purposes: this repository itself is provided under MIT-0.
Feel free to use, hack and adopt this freely: no attribution needed.

## Instructions

### Getting started

[Use this template](https://github.com/Leafwing-Studios/template-repo/generate) by pressing the big green "Use this template" button in the top right corner of [this repo](https://github.com/Leafwing-Studios/template-repo) to create a new repository.

### Running your game

Use `cargo run --release`.

### CI

The CI will:

1. Ensure the code is formatted with `cargo fmt`.
2. Ensure that the code compiles.
3. Ensure that (almost) all `clippy` lints pass.
4. Ensure all tests pass on Windows, MacOS and Ubuntu.

Check this locally with:

1. `cargo run ci`
2. `cargo test --workspace`

### Documentation

Reference documentation is handled with standard Rust doc strings.
Use `cargo doc --open` to build and then open the docs.

Design docs (or other book-format documentation) is handled with [mdBook](https://rust-lang.github.io/mdBook/index.html).
Install it with `cargo install mdbook`, then use `mdbook serve --open` to launch the docs.

Note that "```rust" formatted code in your book will be [automatically tested](https://rust-lang.github.io/mdBook/cli/test.html).

Use "```rust,ignore" instead to bypass this.
