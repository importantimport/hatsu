set dotenv-load

default:
  just --list

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
