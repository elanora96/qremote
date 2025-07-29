{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };
    systems.url = "github:nix-systems/default";
    rust-flake = {
      url = "github:juspay/rust-flake";
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
      ];
      perSystem =
        {
          self',
          pkgs,
          lib,
          config,
          ...
        }:
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
                  filterCargoSources path type
                  || lib.hasSuffix "templates/index.hbs" path
                  || lib.hasSuffix "assets/styles.css" path
                  || lib.hasSuffix "assets/app.js" path;
              };

            crates."qremote".crane.args = {
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
            name = "qremote-shell";
            inputsFrom = [
              self'.devShells.rust
            ];
          };
        };
    };
}
