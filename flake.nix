{
  description = "github:importantimport/hatsu";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    flake-parts.url = "github:hercules-ci/flake-parts";

    fenix.url = "github:nix-community/fenix/monthly";
    fenix.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs =
    inputs@{
      fenix,
      flake-parts,
      nixpkgs,
      ...
    }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [ ];
      systems = [
        "x86_64-linux"
        "aarch64-linux"
      ];

      perSystem =
        {
          config,
          self',
          inputs',
          lib,
          pkgs,
          system,
          ...
        }:
        let
          toolchain =
            with fenix.packages.${system};
            combine [
              complete.toolchain
              targets.aarch64-unknown-linux-gnu.latest.rust-std
              targets.aarch64-unknown-linux-musl.latest.rust-std
              targets.x86_64-unknown-linux-gnu.latest.rust-std
              targets.x86_64-unknown-linux-musl.latest.rust-std
            ];
        in
        {
          devShells.default = pkgs.mkShell {
            packages =
              [ toolchain ]
              ++ (with pkgs; [
                mdbook # ./docs/

                # cargo-*
                cargo-watch
                cargo-zigbuild

                # sea-orm
                sea-orm-cli

                # just
                # mold
                # sccache
              ])
              ++ (with fenix.packages.${system}; [
                rust-analyzer
              ]);
          };

          packages.default =
            let
              version = (builtins.fromTOML (builtins.readFile ./Cargo.toml)).workspace.package.version;
              rustPlatform = pkgs.makeRustPlatform {
                cargo = toolchain;
                rustc = toolchain;
              };
            in
            rustPlatform.buildRustPackage {
              inherit version;
              pname = "hatsu";
              src = ./.;
              cargoLock.lockFile = ./Cargo.lock;
            };
        };
    };
}
