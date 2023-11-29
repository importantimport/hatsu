set dotenv-load

default:
  just --list

# TODO: HATSU_LISTEN_PORT / HATSU_LISTEN_HOSTNAME
create-account name:
  curl -X POST "http://localhost:3939/api/hatsu/v0/admin/create-account" \
  -H "Content-Type: application/json" \
  -d "{\"token\": \"${HATSU_ACCESS_TOKEN}\", \"name\": \"{{name}}\"}"
