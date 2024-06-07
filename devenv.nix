# https://devenv.sh/basics/
{ pkgs, ... }: {
  name = "hatsu";

  # https://devenv.sh/languages/
  langauges.rust.enable = true;

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
