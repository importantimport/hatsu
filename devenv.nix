# https://devenv.sh/basics/
{ lib, pkgs, ... }:
let rust_toolchain = lib.importTOML ./rust-toolchain.toml;
in {
  name = "hatsu";

  # https://devenv.sh/languages/
  languages.rust = {
    enable = true;
    channel = rust_toolchain.toolchain.channel;
    components = rust_toolchain.toolchain.components;
  };

  imports = [
    # This is just like the imports in devenv.nix.
    # See https://devenv.sh/guides/using-with-flake-parts/#import-a-devenv-module
    # ./devenv-foo.nix
  ];

  # https://devenv.sh/packages/
  packages = with pkgs; [
    # cargo-*
    cargo-watch

    mold
    sccache
  ];

  # enterShell = ''
  #   hello
  # '';
}
