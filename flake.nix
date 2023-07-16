{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.05";
    flake-parts.url = "github:hercules-ci/flake-parts";
  };

  outputs = inputs @ {
    self,
    nixpkgs,
    flake-parts,
    ...
  }:
    flake-parts.lib.mkFlake {inherit inputs;} {
      systems = [
        "aarch64-linux"
        "x86_64-linux"
      ];

      perSystem = {pkgs, ...}: {
        devShells.default = with pkgs;
          mkShell {
            name = "dev";
            RUST_SRC_PATH = "${rustPlatform.rustLibSrc}";
            RUST_BACKTRACE = "1";
            packages = [
              cargo
              rustc
              rust-analyzer-unwrapped
              rustfmt
            ];
          };
      };
    };
}
