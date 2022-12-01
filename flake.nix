{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-22.11";
    flake-parts.url = "github:hercules-ci/flake-parts";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    nix-filter.url = "github:numtide/nix-filter";
  };

  outputs = inputs @ {
    self,
    nixpkgs,
    flake-parts,
    ...
  }:
    flake-parts.lib.mkFlake {inherit self;} {
      systems = [
        "aarch64-linux"
        "x86_64-linux"
      ];

      perSystem = {
        system,
        pkgs,
        config,
        ...
      }: {
        packages = {
          _toolchain_dev = with inputs.fenix.packages.${system}; (complete.withComponents [
            "rustc"
            "cargo"
            "rust-src"
            "clippy"
            "rustfmt"
            "rust-analyzer"
          ]);
        };

        devShells.default = with pkgs;
          mkShell {
            # Shell with CC
            name = "dev";
            RUST_SRC_PATH = "${config.packages._toolchain_dev}/lib/rustlib/src/rust/library";
            packages = [
              config.packages._toolchain_dev
            ];
          };
      };
    };
}
