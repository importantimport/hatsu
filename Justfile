set dotenv-load

# show available recipes
list:
  @just --list

dev:
  cargo watch -x run

build:
  cargo build --release

# method: create/remove name: example.com
account method name:
  #!/bin/sh
  if [ -z ${HATSU_ACCESS_TOKEN+x} ]; then
    echo "env HATSU_ACCESS_TOKEN must be set"
  else
    just _account {{method}} {{name}}
  fi

_account method name:
  curl -X POST "http://localhost:${HATSU_LISTEN_PORT}/api/hatsu/v0/admin/{{method}}-account?token=${HATSU_ACCESS_TOKEN}" \
  -H "Content-Type: application/json" \
  -d "{\"name\": \"{{name}}\"}"

# setup dev environment for arch linux
setup-arch:
  sudo pacman -S mold rustup
  just _setup-rustup
  just _setup-cargo

# setup dev environment for debian sid
setup-debian:
  sudo apt install mold rustup
  just _setup-rustup
  just _setup-cargo

# setup dev environment for docker (debian:sid-slim)
setup-docker:
  just setup-debian
  cargo install cargo-chef

# rustup install nightly
# rustup override set nightly
# rustup component add rustc-codegen-cranelift-preview --toolchain nightly
# TODO: cargo-pgo
# rustup component add llvm-tools-preview
_setup-rustup:
  @echo "TODO"

# cargo install sccache
# TODO: cargo-pgo
# cargo install cargo-pgo
_setup-cargo:
  cargo install cargo-watch

# cargo install cross --git https://github.com/cross-rs/cross
_setup-cross:
  @echo "TODO"
