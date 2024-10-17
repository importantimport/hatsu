# Nix/NixOS Installation

[![nixpkgs unstable package](https://repology.org/badge/version-for-repo/nix_unstable/hatsu.svg)](https://repology.org/project/hatsu/versions)

Hatsu is available in Nixpkgs, NUR and Flakes.

macOS (Darwin) is not supported.

## Nixpkgs

Nixpkgs only has a stable version, you need nixos-24.11 or nixos-unstable.

```nix
{ pkgs, ... }: {
  environment.systemPackages = with pkgs; [
    hatsu
  ];
}
```

## NUR (SN0WM1X)

The SN0WM1X NUR may contain beta versions, but there may be a delay.

You need to [follow the instructions to set up NUR](https://github.com/nix-community/nur#installation) first.

```nix
{ pkgs, ... }: {
  environment.systemPackages = with pkgs; [
    nur.repos.sn0wm1x.hatsu
  ];
}
```

## Flakes

> This is untested.

Add the hatsu repository directly to your flake inputs, up to date but unstable.

```nix
{
  inputs: {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    # ...
    hatsu.url = "github:importantimport/hatsu";
    hatsu.inputs.nixpkgs.follows = "nixpkgs";
    # ...
  };
}
```

```nix
{ inputs, pkgs, ... }: {
  environment.systemPackages = [
    inputs.hatsu.packages.${pkgs.system}.default;
  ];
}
```
