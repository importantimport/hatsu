set dotenv-load

default:
  just --list

# TODO: HATSU_LISTEN_PORT / HATSU_LISTEN_HOSTNAME
# method: create/remove name: example.com
account method name:
  #!/bin/sh
  if [ -z ${HATSU_ACCESS_TOKEN+x} ]; then
    echo "env HATSU_ACCESS_TOKEN must be set"
  else
    just _account {{method}} {{name}}
  fi

# method: create/remove name: example.com
_account method name:
  curl -X POST "http://localhost:3939/api/hatsu/v0/admin/{{method}}-account" \
  -H "Content-Type: application/json" \
  -d "{\"token\": \"${HATSU_ACCESS_TOKEN}\", \"name\": \"{{name}}\"}"
