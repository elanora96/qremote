_: {
  perSystem =
    { pkgs, ... }:
    {
      devShells.cross = pkgs.mkShell {
        name = "qremote-cross-shell";
        buildInputs = [
          pkgs.rustup
          pkgs.cargo-cross
          pkgs.gcc
          pkgs.podman
          pkgs.sccache
        ];

        shellHook = ''
          rustup toolchain install stable
        '';
      };
    };
}
