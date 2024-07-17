{
  description = "Description for the project";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    flake-parts.url = "github:hercules-ci/flake-parts";

    crane.url = "github:ipetkov/crane";
    crane.inputs.nixpkgs.follows = "nixpkgs";

    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = inputs@{ crane, flake-parts, nixpkgs, rust-overlay, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [ ];
      systems = [ "x86_64-linux" "aarch64-linux" ];

      perSystem = { config, self', inputs', lib, pkgs, system, ... }:
        let
          crossSystem =
            if system == "x86_64-linux" then
              "aarch64-linux"
            else "x86_64-linux";
          localSystem = system;
          overlays = [ rust-overlay.overlays.rust-overlay ];
          pkgs = import nixpkgs { inherit crossSystem localSystem overlays; };

          craneLib = (crane.mkLib pkgs).overrideToolchain (pkgs:
            pkgs.rust-bin.selectLatestNightlyWith (toolchain: toolchain.default.override {
              extensions = [
                "rust-src"
                # "rust-analyzer"
              ];
              targets = [
                "aarch64-unknown-linux-gnu"
                "aarch64-unknown-linux-musl"
                "x86_64-unknown-linux-gnu"
                "x86_64-unknown-linux-musl"
              ];
            })
          );

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

          craneArgs = {
            inherit src;
            strictDeps = true;
          };

          cargoArtifacts = craneLib.buildDepsOnly craneArgs;

          cargoFmt = craneLib.cargoFmt {
            inherit src;
          };

          cargoClippy = craneLib.cargoClippy (craneArgs // {
            inherit cargoArtifacts;
            cargoClippyExtraArgs = "--all-targets -- -W clippy::pedantic -W clippy::nursery -A clippy::missing-errors-doc -A clippy::module_name_repetitions";
          });

          hatsu = craneLib.buildPackage (craneArgs // {
            inherit cargoArtifacts;
          });

          # TODO: cross test
          # hatsu-aarch64-unknown-linux-gnu = craneLib.buildPackage
          #   (craneArgs // {
          #     CARGO_BUILD_TARGET = "aarch64-unknown-linux-gnu";
          #     CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER = "${pkgs.stdenv.cc.targetPrefix}cc";
          #     CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_RUNNER = "qemu-aarch64";

          #     HOST_CC = "${pkgs.stdenv.cc.nativePrefix}cc";
          #     TARGET_CC = "${pkgs.stdenv.cc.targetPrefix}cc";
          #   });
        in
        {
          checks = {
            inherit hatsu cargoFmt cargoClippy;
          };

          packages = {
            default = hatsu;
            # TODO: cross test
            # aarch64-unknown-linux-gnu = hatsu-aarch64-unknown-linux-gnu;
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
