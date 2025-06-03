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

            # crane-lib = (inputs.rust-flake.inputs.crane.mkLib pkgs).overrideToolchain (
            #   p:
            #   p.rust-bin.stable.latest.default.override {
            #     targets = [ "x86_64-unknown-linux-musl" ];
            #   }
            # );

            crates."qremote".crane.args = {
              strictDeps = true;
              buildInputs = with pkgs; [
                pkg-config
              ];
              nativeBuildInputs = with pkgs; [
                pkg-config
              #   makeWrapper
              # ] ++ (with pkgsStatic; [
              #   bashInteractive
              #   cmake
              #   makeWrapper
              #   makeShellWrapper
              #   ninja
                (libxkbcommon.override { withWaylandTools = true; })
                xdotool
              ];

              # CARGO_BUILD_TARGET = "x86_64-unknown-linux-musl";
              # CARGO_BUILD_RUSTFLAGS = "-C target-feature=+crt-static";
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
