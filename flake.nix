{
  description = "Rust Lang Esp flake for NixOs users";
  inputs = {
    flake-parts.url = "github:hercules-ci/flake-parts";
    flake-utils.url = "github:numtide/flake-utils";
    fenix.url = "github:nix-community/fenix/monthly";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    crane.url = "github:ipetkov/crane";
    nix-npm-buildpackage = { url = "github:serokell/nix-npm-buildpackage"; };
  };
  outputs =
    inputs @ { flake-parts
    , fenix
    , nixpkgs
    , flake-utils
    , crane
    , nix-npm-buildpackage
    , self
    , ...
    }:
    inputs.flake-parts.lib.mkFlake
      {
        inherit inputs;
      }
      {
        systems = [ "x86_64-linux" ];
        perSystem =
          { config
          , pkgs
          , system
          , ...
          }:
          let
            inherit (pkgs) lib;
            # Toolchain with wasm32 target
            toolchain = with fenix.packages.${system};
              combine [
                complete.cargo
                complete.clippy
                complete.rust-src
                complete.rustc
                complete.rustfmt
                targets.wasm32-unknown-unknown.latest.rust-std
              ];
            # craneLib with wasm32 toolchain
            craneLib = crane.lib.${system}.overrideToolchain toolchain;
            # Node modules installer
            bp = pkgs.callPackage nix-npm-buildpackage { };
            # Filtering all sources need for build and check
            src = lib.cleanSourceWith {
              src = craneLib.path ./.;
              filter = path: type:
                (lib.hasSuffix "html" path)
                || (lib.hasSuffix "scss" path)
                || (lib.hasSuffix "css" path)
                || (lib.hasSuffix "toml" path)
                || (lib.hasSuffix "tailwind.config.js" path)
                || (lib.hasInfix "/assets" path)
                || (lib.hasInfix "/public" path)
                || (craneLib.filterCargoSources path type);
            };
            # Args for base mkCargoDerivation
            commonArgs = {
              inherit src;
              CARGO_BUILD_TARGET = "wasm32-unknown-unknown";
            };
            # Compile all artifacts with wasm32
            cargoArtifacts = craneLib.buildDepsOnly (commonArgs
              // {
              doCheck = false;
            });
            # Node modules installer
            node_modules = bp.mkNodeModules {
              src = ./.;
              pname = "npm";
              version = "2";
            };
            # extras repo for trunk build
            extras = pkgs.fetchFromGitHub {
              owner = "RustLangES";
              repo = "proyectos-comunitarios";
              rev = "master";
              hash = "sha256-X6VHTxVymZ76DLqDw6ZF59PRCEc7R5SVdcJ6qmrsXY4=";
            };
            # Build trunk derivation and resolve result folder with all output files
            web-app = craneLib.buildTrunkPackage (commonArgs
              // {
              inherit cargoArtifacts;
              # Adding tailwind for build all styles pre trunk build and hashing files
              buildInputs = with pkgs; [
                tailwindcss
                git
              ];
              # Pre build script, copy all node_modules, extras and run tailwindcss cli
              preBuild = ''
                echo > Trunk.toml
                cp -r ${extras} extras
                mkdir gen_assets
                cp -r ${node_modules}/node_modules node_modules
                tailwindcss -i  "$(dirname ./input.css)/input.css" -o "$(dirname ./index.html)/style/output.css"
              '';
            });
            # Serve via python3 http.server result folder in buildTrunkPackage derivation
            serve-app = pkgs.writeShellScriptBin "serve-app" ''
              ${pkgs.python3Minimal}/bin/python3 -m http.server -d ${web-app} 8000
            '';
            # Custom derivation using source filtering and run leptosfmt check
            leptosFmtCheck = {}:
              craneLib.mkCargoDerivation (commonArgs
                // {
                inherit cargoArtifacts;
                pnameSuffix = "-leptosfmt";
                buildPhaseCargoCommand = "leptosfmt --check .";
                nativeBuildInputs = [ pkgs.leptosfmt ];
              });
          in
          rec
          {
            # nix flake check
            checks = {
              inherit web-app;

              cargo-clippy = craneLib.cargoClippy (commonArgs
                // {
                inherit cargoArtifacts;
                cargoClippyExtraArgs = "-- -D warnings";
              });
              cargo-fmt = craneLib.cargoFmt {
                inherit src;
                cargoExtraArgs = "--all";
              };
              web-app-leptosfmt = leptosFmtCheck { };
            };

            # nix build
            packages.default = web-app;

            # nix run
            apps.default = flake-utils.lib.mkApp {
              drv = serve-app;
            };

            # nix develop
            devShells.default = craneLib.devShell {
              packages = with pkgs; [
                nodejs
                tailwindcss
                nodePackages.tailwindcss
                toolchain
                trunk
                cargo-leptos
                cargo-leptosfmt
                cargo-make
                binaryen
              ];
            };
          };
      };
}
