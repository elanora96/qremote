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
          ...
        }:
        {
          rust-project.crates."qremote".crane.args = {
            nativeBuildInputs = [
              pkgs.xdotool
              pkgs.libxkbcommon
            ];
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
