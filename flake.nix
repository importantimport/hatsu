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
          toolchain = with fenix.packages.${system}; combine [
            (fromToolchainFile {
              file = ./rust-toolchain.toml;
              sha256 = "bx0H6uahYI+z2i+dMWDH/GzH9mm298NMsUF0eR5kmc4=";
            })
            # minimal.toolchain
            # targets.aarch64-unknown-linux-gnu.latest.rust-std
            # targets.aarch64-unknown-linux-musl.latest.rust-std
            # targets.x86_64-unknown-linux-gnu.latest.rust-std
            # targets.x86_64-unknown-linux-musl.latest.rust-std
          ];

          craneLib = (crane.mkLib pkgs).overrideToolchain toolchain;

          # https://github.com/ipetkov/crane/blob/master/docs/source-filtering.md
          src = lib.cleanSourceWith {
            name = "source";
            src = ./.;
            filter =
              path: type:
              (lib.hasInfix "/crates/backend/assets/" path)
              || (craneLib.filterCargoSources path type);
          };

          commonArgs = {
            inherit src;
            strictDeps = true;

            nativeBuildInputs = with pkgs; [ cmake pkg-config ];
            buildInputs = with pkgs; [ openssl ];

            # fix openssl
            OPENSSL_NO_VENDOR = true;
            OPENSSL_DIR = "${pkgs.openssl.dev}";
            OPENSSL_LIB_DIR = "${pkgs.openssl.out}/lib";
            OPENSSL_INCLUDE_DIR = "${pkgs.openssl.dev}/include/";
          };

          cargoArtifacts = craneLib.buildDepsOnly commonArgs;

          crateClippy = craneLib.cargoClippy (commonArgs // {
            inherit cargoArtifacts;
            cargoClippyExtraArgs = "--all-targets -- -W clippy::pedantic -W clippy::nursery -A clippy::missing-errors-doc -A clippy::module_name_repetitions";
          });

          crateFmt = craneLib.cargoFmt {
            inherit src;
          };

          crate = craneLib.buildPackage (commonArgs // {
            inherit cargoArtifacts;
          });

        in
        {
          checks = {
            inherit crate crateClippy crateFmt;
          };

          packages.default = crate;
          devShells.default = craneLib.devShell {
            # checks = self'.checks.${system};
            inputsFrom = [ crate ];
            packages = with pkgs; [
              mdbook # ./docs/

              # cargo-*
              cargo-watch

              just
              # mold
              # sccache
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
