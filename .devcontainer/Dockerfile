FROM nixos/nix:latest

COPY . /app
WORKDIR /app

RUN nix-env -iA nixpkgs.bashInteractive nixpkgs.direnv nixpkgs.git && \
    eval "$(direnv hook bash)" && \
    direnv allow
