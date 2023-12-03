set dotenv-load

# show available recipes
list:
  @just --list

# starting dev server.
dev:
  cargo watch -x run

# building production.
build:
  cargo build --release

# cross-build:
#   cross build --release --target aarch64-unknown-linux-gnu
#   cross build --release --target aarch64-unknown-linux-musl
#   cross build --release --target x86_64-unknown-linux-gnu
#   cross build --release --target x86_64-unknown-linux-musl

# docker-build name='importantimport/hatsu' version='latest':
#   docker build . \
#   --tag "{{name}}:{{version}}"

# docker-buildx name='importantimport/hatsu' version='latest':
#   docker buildx build . \
#   --tag "{{name}}:{{version}}"
#   --platform "linux/amd64,linux/arm64"

# create and remove account (method: create/remove) (name: example.com)
account method name:
  #!/bin/sh
  if [ -z ${HATSU_ACCESS_TOKEN+x} ]; then
    echo "env HATSU_ACCESS_TOKEN must be set"
  else
    just _account {{method}} {{name}}
  fi

_account method name:
  curl -X POST "http://localhost:${HATSU_LISTEN_PORT}/api/v0/admin/{{method}}-account?token=${HATSU_ACCESS_TOKEN}" \
  -H "Content-Type: application/json" \
  -d "{\"name\": \"{{name}}\"}"

# apply migrations to database.
db_migration *args='up':
  just _sea-orm-cli migrate {{args}} -d crates/db_migration

# generate entities from database.
db_schema: (db_migration 'fresh')
  just _sea-orm-cli generate entity -o crates/db_schema/src/entities

# detect before running sea-orm-cli and install it if it doesn't exist.
_sea-orm-cli *args:
  #!/bin/sh
  if [ -z $(which sea-orm-cli) ]; then
    cargo install sea-orm-cli
  fi
  sea-orm-cli {{args}}

# setup dev environment for arch linux (target-arch: amd64/arm64)
setup-arch target-arch='amd64':
  sudo pacman -S mold rustup
  just _setup-rustup {{target-arch}}
  just _setup-cargo arch

# setup dev environment for debian sid (target-arch: amd64/arm64)
setup-debian target-arch='amd64':
  sudo apt install mold rustup
  just _setup-rustup {{target-arch}}
  just _setup-cargo debian

# setup dev environment for docker (target-arch: amd64/arm64)
setup-docker target-arch='amd64':
  just setup-debian
  cargo install cargo-chef

# rustup install nightly
# rustup override set nightly
# rustup component add rustc-codegen-cranelift-preview --toolchain nightly
# TODO: cargo-pgo
# rustup component add llvm-tools-preview
_setup-rustup target-arch='amd64':
  @echo "TODO"

# TODO: cargo-pgo
# cargo install cargo-pgo
# (distro: undefined/arch/debian)
_setup-cargo distro='undefined':
  {{ if distro == 'arch' { "sudo pacman -S cargo-watch" } else { "cargo install cargo-watch" } }}

# cargo install cross --git https://github.com/cross-rs/cross
_setup-cross:
  @echo "TODO"
