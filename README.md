![Nix flake check](https://github.com/marijanp/rust-game/actions/workflows/check.yaml/badge.svg)

# Rust Game Template Project

This project is an exploration of the Rust game development ecosystem, structured as a reusable template. The goal is to showcase how various crates can be integrated to build a game, hopefully making it easier for others to get started with Rust-based game development. If you're new to Rust or game development, this template can serve as a useful starting point.

## Features

- **Game Engine**: Built on top of [Bevy](https://bevyengine.org/), a data-driven game engine that emphasizes simplicity and modularity.
- **Input Management**: Handled with [leafwing-input-manager](https://github.com/Leafwing-Studios/leafwing-input-manager) for flexible input control.
- **Physics**: Powered by [bevy_rapier](https://github.com/dimforge/bevy_rapier) for 2D/3D physics simulations.
- **Level Editing**: Support for [LDtk](https://github.com/Trouv/bevy_ecs_ldtk), allowing one to design levels in a user-friendly editor and easily integrate them into your game.

## Build & Platform Support

The game is packaged with [Nix](https://nixos.org/), this ensures reproducible builds and complete deployments across different platforms:

- **Linux**: Supports x86_64 and aarch64 architectures.
- **macOS**: Works on both Intel and Apple Silicon.
- **WebAssembly (WASM)**: The game can be compiled to run in browsers.

You can try the game online here: [Play Now](https://game.marijan.pro).

## Automated Testing

An automated end-to-end (e2e) system test ensures the game runs correctly by verifying that the main menu appears on startup. The NixOS e2e test framework:

1. Spawns a virtual machine with an X server enabled.
2. Launches the game.
3. Uses OCR (Optical Character Recognition) to detect the main menu text, confirming successful startup.

## Development

The following sections explain how to obtain a development shell, what tools to use during development, and lastly what to run before committing.

### Development Shell

There are two to obtain a development shell: you can configure [direnv](https://direnv.net/) to be dropped in a development shell automatically when you enter the directory (recommended) or do it manually.

#### Automatic Development Shell using `direnv`

First, you will have to [install direnv](https://direnv.net/docs/installation.html), by adding it to your Nix/NixOS configuration or using your package manager.

Afterward, add a `.envrc` file to the root of the project:

```sh
touch .envrc
echo "use flake" >> .envrc
```

Next, enable direnv for this project:

```sh
direnv allow
```

#### Obtaining a Development Shell Manually

Run:

```sh
nix develop
```

### Inside a Development Shell

Inside the development shell, you can use `cargo` as usual during development.

### Before you Commit

Because Nix gives us gives us a high degree of reproducibility, by building our project and running the checks locally and making them succeed, we can be very certain it will pass the pipeline too.

#### Build

You can explore the buildable outputs of any flake project by running:

```sh
nix flake show
```

To build e.g. `game` you can then run:

```sh
nix build .#game
```

#### Run the Checks

To run all the "checks" of this project, like formatting, lint, audit, etc. checks, run:

```sh
nix flake check
```

To run a single check e.g. the format check, run:

```sh
nix build .#checks.<system>.treefmt
```

### Format

Code for the whole project tree can be formatted by running `nix fmt` from the project's root or anywhere in the tree, but be warned that it will only format code inside the sub-tree.

The `nix fmt` command currently formats all the `Rust` and `Nix` code in the tree. To add support for more languages you'll have to adjust the `treefmt` attribute-set in the `flake.nix` accordingly. A list of already supported formatters can be found [here](https://numtide.github.io/treefmt/formatters/).
