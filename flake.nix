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
            minimal.toolchain
            targets.aarch64-unknown-linux-gnu.latest.rust-std
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
              (lib.hasInfix "/contrib/" path)
              || (lib.hasInfix "/crates/backend/assets/" path)
              || (craneLib.filterCargoSources path type);
          };

          commonArgs = {
            inherit src;
            strictDeps = true;
          };

          cargoArtifacts = craneLib.buildDepsOnly commonArgs;

          cargoClippy = craneLib.cargoClippy (commonArgs // {
            inherit cargoArtifacts;
            cargoClippyExtraArgs = "--all-targets -- -W clippy::pedantic -W clippy::nursery -A clippy::missing-errors-doc -A clippy::module_name_repetitions";
          });

          cargoFmt = craneLib.cargoFmt {
            inherit src;
          };

          buildHatsu = args:
            craneLib.buildPackage (commonArgs // {
              inherit cargoArtifacts;
            } // lib.optionalAttrs (!isNull args) args);

          hatsu = buildHatsu { };
        in
        {
          checks = {
            inherit cargoFmt cargoClippy hatsu;
          };

          packages = {
            default = hatsu;
            aarch64-unknown-linux-gnu = buildHatsu {
              CARGO_BUILD_TARGET = "aarch64-unknown-linux-gnu";
              CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER =
                let
                  inherit (pkgs.pkgsCross.aarch64-multiplatform.stdenv) cc;
                in
                "${cc}/bin/${cc.targetPrefix}cc";
            };
          };
          devShells.default = craneLib.devShell {
            # checks = self'.checks.${system};
            inputsFrom = [ hatsu ];
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
