{
  description = "Description for the project";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    devenv-root.url = "file+file:///dev/null";
    devenv-root.flake = false;
    devenv.url = "github:cachix/devenv";

    flake-parts.url = "github:hercules-ci/flake-parts";

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

  outputs = inputs@{ flake-parts, devenv-root, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [
        inputs.devenv.flakeModule
      ];
      systems = [ "x86_64-linux" "aarch64-linux" ];

      perSystem = { config, self', inputs', lib, pkgs, system, ... }: {
        # Per-system attributes can be defined here. The self' and inputs'
        # module parameters provide easy access to attributes of the same
        # system.

        # Equivalent to  inputs'.nixpkgs.legacyPackages.hello;
        # packages.default = pkgs.hello;

        devenv.shells.default = {
          name = "hatsu";
          imports = [
            ./devenv.nix
          ];

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
