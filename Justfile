set dotenv-load

# show available recipes.
list:
  @just --list

# detect before running cli and install it if it doesn't exist.
_cargo_ cli *args:
  #!/bin/sh
  if [ -z $(which {{cli}}) ]; then
    cargo install {{cli}}
  fi
  case {{cli}} in
    cargo-*) cargo $(echo {{cli}} | sed "s/^cargo-//") {{args}} ;;
    * ) {{cli}} {{args}} ;;
  esac

# running dev server.
run:
  just _cargo_ cargo-watch -x run

# building production.
build:
  cargo build --release

# building multi-arch production.
buildx:
  just _build x86_64-unknown-linux-gnu
  just _build x86_64-unknown-linux-musl
  just _zigbuild aarch64-unknown-linux-gnu
  just _zigbuild aarch64-unknown-linux-musl

_build target:
  rustup target add {{target}}
  cargo build --release --target {{target}}

_zigbuild target:
  rustup target add {{target}}
  just _cargo_ cargo-zigbuild --release --target {{target}}

# format code. (args example: just fmt --check)
fmt *args='':
  cargo fmt --all {{args}}

# check code. (args example: just check --quiet)
check *args='':
  cargo check --all {{args}}

# lint code. (args example: just lint --fix)
lint *args='':
  cargo clippy {{args}} -- -W clippy::pedantic -W clippy::nursery -A clippy::missing-errors-doc -A clippy::module_name_repetitions

# running tests.
test *args='':
  cargo test --all {{args}}

# docker-build version='nightly':
#   docker build . \
#   --tag "importantimport/hatsu:{{version}}"

# docker-buildx version='nightly':
#   docker buildx build . \
#   --platform "linux/amd64,linux/arm64" \
#   --tag "importantimport/hatsu:{{version}}"

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

# use db_* without underscores.
db *args='migration up':
  just db_{{args}}

# apply migrations to database.
db_migration *args='up':
  just _cargo_ sea-orm-cli migrate {{args}} -d crates/db_migration -u $HATSU_DATABASE_URL

# generate entities from database.
db_schema: (db_migration 'fresh')
  just _cargo_ sea-orm-cli generate entity -l -o crates/db_schema/src -u $HATSU_DATABASE_URL
