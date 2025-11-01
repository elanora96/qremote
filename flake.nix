{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    git-hooks-nix = {
      url = "github:cachix/git-hooks.nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };
    systems.url = "github:nix-systems/default";
    rust-flake = {
      url = "github:elanora96/rust-flake";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    inputs:
    inputs.flake-parts.lib.mkFlake { inherit inputs; } {
      systems = import inputs.systems;
      imports = [
        inputs.rust-flake.flakeModules.default
        inputs.rust-flake.flakeModules.nixpkgs
        inputs.treefmt-nix.flakeModule
        inputs.git-hooks-nix.flakeModule
      ];
      perSystem =
        {
          self',
          pkgs,
          lib,
          config,
          ...
        }:
        let
          name = "qremote";
        in
        {

          rust-project = {
            src =
              let
                filterCargoSources =
                  path: type:
                  config.rust-project.crane-lib.filterCargoSources path type
                  && !(lib.hasSuffix ".toml" path && !lib.hasSuffix "Cargo.toml" path);
              in
              lib.cleanSourceWith {
                src = inputs.self;
                filter =
                  path: type:
                  filterCargoSources path type || lib.hasInfix "templates/" path || lib.hasInfix "assets/" path;
              };

            crates.${name}.crane.args = {
              strictDeps = true;
              buildInputs = with pkgs; [
                pkg-config
              ];
              nativeBuildInputs = with pkgs; [
                pkg-config
                (libxkbcommon.override { withWaylandTools = true; })
                xdotool
              ];
            };
          };
          packages.default = self'.packages.qremote;
          devShells.default = pkgs.mkShell {
            name = "${name}-shell";
            inputsFrom = [
              self'.devShells.rust
            ];
          };
          pre-commit.settings.hooks = {
            cargo-check.enable = true;
            clippy.enable = true;
            treefmt.enable = true;
          };

          treefmt = {
            projectRootFile = "rust-toolchain.toml"; # Used to find the project root
            programs = {
              rustfmt = {
                enable = true;
                edition = "2024";
              };
              biome.enable = true;
              mdformat.enable = true;
              nixfmt.enable = true;
              statix.enable = true;
              taplo.enable = true;
            };
          };
        };
    };
}
