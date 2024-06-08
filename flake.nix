{
  description = "Description for the project";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    devenv-root.url = "file+file:///dev/null";
    devenv-root.flake = false;
    devenv.url = "github:cachix/devenv";

    flake-parts.url = "github:hercules-ci/flake-parts";

    crane.url = "github:ipetkov/crane";
    crane.inputs.nixpkgs.follows = "nixpkgs";

    fenix.url = "github:nix-community/fenix/monthly";
    fenix.inputs.nixpkgs.follows = "nixpkgs";

    # nix2container.url = "github:nlewo/nix2container";
    # nix2container.inputs.nixpkgs.follows = "nixpkgs";

    # mk-shell-bin.url = "github:rrbutani/nix-mk-shell-bin";
  };

  nixConfig = {
    extra-trusted-public-keys = "devenv.cachix.org-1:w1cLUi8dv3hnoSPGAuibQv+f9TZLr6cv/Hm9XgU50cw=";
    extra-substituters = "https://devenv.cachix.org";
  };

  outputs = inputs@{ crane, devenv-root, fenix, flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [
        inputs.devenv.flakeModule
      ];
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

            buildInputs = with pkgs; [
              openssl
              pkg-config
            ];
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
          packages.default = crate;
          checks = {
            inherit crate crateClippy;
          };

          devenv.shells.default = {
            name = "hatsu";
            imports = [
              ./devenv.nix
            ];

            # temporary fix
            # https://github.com/cachix/devenv/issues/528
            # https://github.com/cachix/devenv/issues/760
            containers = pkgs.lib.mkForce { };

            devenv.root =
              let
                devenvRootFileContent = builtins.readFile devenv-root.outPath;
              in
              pkgs.lib.mkIf (devenvRootFileContent != "") devenvRootFileContent;
          };
        };
      flake = {
        # The usual flake attributes can be defined here, including system-
        # agnostic ones like nixosModule and system-enumerating ones, although
        # those are more easily expressed in perSystem.
      };
    };
}
