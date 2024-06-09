{
  description = "Description for the project";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    flake-parts.url = "github:hercules-ci/flake-parts";

    crane.url = "github:ipetkov/crane";
    crane.inputs.nixpkgs.follows = "nixpkgs";

    fenix.url = "github:nix-community/fenix/monthly";
    fenix.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = inputs@{ crane, fenix, flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [ ];
      systems = [ "x86_64-linux" "aarch64-linux" ];

      perSystem = { config, self', inputs', lib, pkgs, system, ... }:
        let
          toolchain = fenix.packages.${system}.fromToolchainFile
            {
              file = ./rust-toolchain.toml;
              sha256 = "PLeBx9Gqpcr0/IzchOJ9g+a0MzOvRFfaoqSx1BXoDFM=";
            };

          craneLib = (crane.mkLib pkgs).overrideToolchain toolchain;
          src = craneLib.cleanCargoSource (craneLib.path ./.);

          commonArgs = {
            inherit src;
            strictDeps = true;

            nativeBuildInputs = with pkgs; [ cmake pkg-config ];
            buildInputs = with pkgs; [ openssl ];

            # try fix openssl
            OPENSSL_DIR = "${pkgs.openssl.dev}";
            OPENSSL_LIB_DIR = "${pkgs.openssl.out}/lib";
            OPENSSL_INCLUDE_DIR = "${pkgs.openssl.dev}/include/";
          };

          cargoArtifacts = craneLib.buildDepsOnly commonArgs;

          crateClippy = craneLib.cargoClippy (commonArgs // {
            inherit cargoArtifacts;
            cargoClippyExtraArgs = "--all-target -- -W clippy::pedantic -W clippy::nursery -A clippy::missing-errors-doc -A clippy::module_name_repetitions";
          });

          crate = craneLib.buildPackage (commonArgs // {
            inherit cargoArtifacts;
          });

        in
        {
          checks = {
            inherit crate crateClippy;
          };

          packages.default = crate;
          devShells.default = craneLib.devShell {
            # checks = self'.checks.${system};
            inputsFrom = [ crate ];
            packages = with pkgs; [
              # rust toolchain
              # toolchain

              # cargo-*
              cargo-watch

              just
              mold
              sccache
            ];
          };
        };
      flake = {
        # The usual flake attributes can be defined here, including system-
        # agnostic ones like nixosModule and system-enumerating ones, although
        # those are more easily expressed in perSystem.
      };
    };
}
