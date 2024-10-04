{
  description = "nix-rust-template";

  nixConfig = {
    extra-substituters = [
      "https://crane.cachix.org"
      "https://nix-community.cachix.org"
    ];
    extra-trusted-public-keys = [
      "crane.cachix.org-1:8Scfpmn9w+hGdXH/Q9tTLiYAE/2dnJYRJP7kl80GuRk="
      "nix-community.cachix.org-1:mB9FSh9qf2dCimDSUo8Zy7bkq5CX+/rkCWyvRCYg3Fs="
    ];
  };

  inputs = {
    flake-parts.url = "github:hercules-ci/flake-parts";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    treefmt-nix.url = "github:numtide/treefmt-nix";
    treefmt-nix.inputs.nixpkgs.follows = "nixpkgs";
    fenix.url = "github:nix-community/fenix";
    fenix.inputs.nixpkgs.follows = "nixpkgs";
    crane.url = "github:ipetkov/crane";
    advisory-db.url = "github:rustsec/advisory-db";
    advisory-db.flake = false;
  };

  outputs =
    inputs@{
      flake-parts,
      treefmt-nix,
      ...
    }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];
      imports = [
        treefmt-nix.flakeModule
        ./nixos
      ];
      perSystem =
        {
          self',
          inputs',
          pkgs,
          lib,
          ...
        }:
        let
          rustToolchain =
            with inputs'.fenix.packages;
            combine [
              stable.toolchain
              targets.wasm32-unknown-unknown.stable.rust-std
            ];
          craneLib = (inputs.crane.mkLib pkgs).overrideToolchain rustToolchain;

          commonAttrs = {
            pname = "game";

            src = lib.fileset.toSource {
              root = ./.;
              fileset = lib.fileset.unions [
                ./Cargo.toml
                ./Cargo.lock
                ./game
              ];
            };

            # https://github.com/bevyengine/bevy/blob/latest/docs/linux_dependencies.md
            nativeBuildInputs = with pkgs; [
              pkg-config
            ];
            buildInputs =
              with pkgs;
              [
              ]
              ++ lib.optionals stdenv.hostPlatform.isLinux [
                alsa-lib
                vulkan-loader
                udev
                # To use the x11 feature
                xorg.libX11
                xorg.libXcursor
                xorg.libXi
                xorg.libXrandr
                # To use the wayland feature
                libxkbcommon
                wayland
              ]
              ++ lib.optionals stdenv.isDarwin [
                libiconv
                darwin.apple_sdk.frameworks.Cocoa
                darwin.apple_sdk.frameworks.CoreAudio
                darwin.apple_sdk.frameworks.AudioUnit
              ];

            # the coverage report will run the tests
            doCheck = false;
          };
          commonAttrsWasm = commonAttrs // {
            cargoExtraArgs = "--target wasm32-unknown-unknown";
            nativeBuildInputs = commonAttrs.nativeBuildInputs ++ [ pkgs.lld ];
            CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_LINKER = "lld";
          };
        in
        {
          devShells.default = pkgs.mkShell {
            inputsFrom = [ self'.packages.default ];
            WINIT_X11_SCALE_FACTOR = "2";
            LD_LIBRARY_PATH = "$LD_LIBRARY_PATH:${lib.makeLibraryPath commonAttrs.buildInputs}";
            # wasm
            CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_LINKER = "lld";
            packages = [
              pkgs.lld
              pkgs.ldtk
            ];
          };

          packages = {
            game-deps-wasm = craneLib.buildDepsOnly commonAttrsWasm;

            game-deps = craneLib.buildDepsOnly commonAttrs;

            game-docs = craneLib.cargoDoc (
              commonAttrs
              // {
                cargoArtifacts = self'.packages.game-deps;
              }
            );

            game-wasm = craneLib.buildPackage (
              commonAttrsWasm
              // {
                cargoArtifacts = self'.packages.game-deps;
              }
            );

            game-dist = pkgs.runCommand "test" { nativeBuildInputs = [ pkgs.wasm-bindgen-cli ]; } ''
              wasm-bindgen --no-typescript --target web \
                --out-dir $out \
                --out-name "game" \
                ${self'.packages.game-wasm}/bin/game.wasm
              ln -s ${./game/assets}/ $out/assets
              ln -s ${./dist}/index.html $out/index.html
              ln -s ${./dist}/js $out/js
            '';

            game = craneLib.buildPackage (
              commonAttrs
              // {
                cargoArtifacts = self'.packages.game-deps;
                meta.mainProgram = "game";
                postInstall = ''
                  ln -s ${./game/assets}/ $out/bin/assets
                '';
                postFixup = lib.optionalString pkgs.stdenv.hostPlatform.isLinux ''
                  patchelf $out/bin/game \
                    --add-rpath ${
                      lib.makeLibraryPath [
                        pkgs.vulkan-loader
                        pkgs.libxkbcommon
                      ]
                    }
                '';

              }
            );

            default = self'.packages.game;
          };

          checks = {
            inherit (self'.packages) game-docs game;

            lint = craneLib.cargoClippy (
              commonAttrs
              // {
                cargoArtifacts = self'.packages.game-deps;
                cargoClippyExtraArgs = "--all-targets -- --deny warnings";
              }
            );

            coverage-report = craneLib.cargoTarpaulin (
              commonAttrs
              // {
                cargoArtifacts = self'.packages.game-deps;
              }
            );
          };

          treefmt = {
            projectRootFile = ".git/config";
            programs.nixfmt.enable = true;
            programs.rustfmt.enable = true;
            programs.rustfmt.package = craneLib.rustfmt;
            settings.formatter = { };
          };
        };
    };
}
