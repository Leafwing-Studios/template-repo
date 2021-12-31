{
  description = "template";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
    nix-fmt.url = "github:nix-community/nixpkgs-fmt";
  };


  outputs = { self, nixpkgs, rust-overlay, flake-utils, nix-fmt, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      with pkgs;
      {
        devShell = mkShell {
          shellHook = ''export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${pkgs.lib.makeLibraryPath [
            pkgs.alsaLib
            pkgs.udev
            pkgs.vulkan-loader
          ]}"'';

          buildInputs = [
            (rust-bin.selectLatestNightlyWith (toolchain: toolchain.default.override {
              extensions = [ "rust-src" ];
            }))
            cargo
            pkgconfig
            udev
            alsaLib
            x11
            xorg.libXcursor
            xorg.libXrandr
            xorg.libXi
            vulkan-tools
            vulkan-headers
            vulkan-loader
            vulkan-validation-layers
            clang
            lld
            nixpkgs-fmt
          ];
        };
      }
    );
}
