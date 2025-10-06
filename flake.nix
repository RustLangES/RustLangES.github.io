{
  description = "RustLangEs web page flake";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { nixpkgs, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        rustToolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            # Rust toolchain with wasm target
            rustToolchain
            
            # WASM tools
            tailwindcss_4
            wasm-bindgen-cli
            
            # Leptos specific tools
            leptosfmt
            cargo-leptos
            
            # Additional development tools
            pkg-config
            openssl
            nodePackages.npm
          ];
        };
      }
    );
}
