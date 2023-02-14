# About

This is the Leafwing Studios' template repo, providing a quick, opinionated base for high-quality Bevy game projects (library template coming soon!).
We've shaved the yaks for you!

The licenses here are provided for template purposes: this repository itself is provided under MIT-0.
Feel free to use, hack and adopt this freely: no attribution needed.

## Instructions

### Getting started

[Use this template](https://github.com/Leafwing-Studios/template-repo/generate) by pressing the big green "Use this template" button in the top right corner of [this repo](https://github.com/Leafwing-Studios/template-repo) to create a new repository.

This repository uses a nightly toolchain and has fast compiles enabled through the [`/.cargo/config.toml`](./.cargo/config.toml) for much faster incremental compile times. This repository also uses the [beta crates.io indexing protocol](https://blog.rust-lang.org/inside-rust/2023/01/30/cargo-sparse-protocol.html). 
Swap from nightly to stable by changing the [`rust-toolchain`](./rust-toolchain.toml) from `"nightly"` to `"stable"` and comment out the `"-Zshare-generics=y/n` statements of [`/.cargo/config.toml](./.cargo/config.toml). 

You may want to run `rustup update` afterwards to get your tools up to date.

1. Search for `ToDo:` comments to find where you need to change the project name from `template_...` to `your_game_name_...`. 

2. Set your dependency's features in [`Cargo.toml`](./template_lib/Cargo.toml) of the game library. If you need features that only your game `main.rs` or other logics you add to [`template_game/src`](./template_game/src/) will use, you will need to change the features on dependencies in [`Cargo.toml`](./template_game/Cargo.toml) from `template_game`.

3. Start writing your game. Your logic should be stored in [`template_lib/src/lib.rs`](./template_lib/src/lib.rs) or in files and folders in [`template_lib/src`](./template_lib/src/) that act as [modules](https://doc.rust-lang.org/reference/items/modules.html).

- Then, add all of your plugins and build your `App` in [`main.rs`](./template_game/src/main.rs).

4. Double check that the LICENSE matches your intent, edit it if it doesn't.

5. Update this README to match your project, modifying any sections as needed.

6. Consider cleaning up the issue and PR templates found in the `.github` folder to better match your needs.

7. Make sure all of your documentation is properly organized and filled out. 

### Running your game

Use `cargo run` to run in debug mode or use `cargo run --release` to test the release build.
This repo is set up to always build with full optimizations, so there's no need for a `--release` flag in most cases where you aren't testing a release.

### Publishing your game

A build will be produced for Windows, MacOS and Linux each time a [tag](https://docs.github.com/en/desktop/contributing-and-collaborating-using-github-desktop/managing-commits/managing-tags) is pushed to GitHub.

These can be found under the [Releases](https://docs.github.com/en/rest/reference/releases) tab of your project.

## Contributing

See [CONTRIBUTING.md](https://github.com/Leafwing-Studios/template-repo/blob/main/CONTRIBUTING.md)!

### Testing

1. Use doc tests aggressively to show how APIs should be used.
You can use `#` to hide a setup line from the doc tests.
2. Unit test belong near the code they are testing. Use `#[cfg(test)]` on the test module to ignore it during builds, and `#[test]` on the test functions to ensure they are run.
3. Integration tests should be stored in the top level `tests` folder, importing functions from `lib.rs`.

Use `cargo test` to run all tests.

### CI

The CI will:

1. Ensure the code is formatted with `cargo fmt`.
2. Ensure that the code compiles.
3. Ensure that (almost) all `clippy` lints pass.
4. Ensure all tests pass on Windows, MacOS and Ubuntu.

Check this locally with:

1. `cargo run -p ci`
2. `cargo test --workspace`

To manually rerun CI on Github:

1. Navigate to the `Actions` tab.
2. Use the dropdown menu in the CI run of interest and select "View workflow file".
3. In the top-right corner, select "Rerun workflow".

### Documentation

Reference documentation is handled with standard Rust doc strings.
Use `cargo doc --open` to build and then open the docs.

Design docs (or other book-format documentation) is handled with [mdBook](https://rust-lang.github.io/mdBook/index.html).
Install it with `cargo install mdbook`, then use `mdbook serve --open` to launch the docs.

### Benchmarking (ToDo)

To run the benchmarks, use `cargo bench`.

For more documentation on making your own benchmarks, check out [criterion's docs](https://bheisler.github.io/criterion.rs/book/index.html).
