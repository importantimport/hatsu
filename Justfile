set dotenv-load

default:
  just --list

# TODO: HATSU_LISTEN_PORT / HATSU_LISTEN_HOSTNAME
# examples:
# just account create example.com
# just account remove example.com
account method name:
  curl -X POST "http://localhost:3939/api/hatsu/v0/admin/{{method}}-account" \
  -H "Content-Type: application/json" \
  -d "{\"token\": \"${HATSU_ACCESS_TOKEN}\", \"name\": \"{{name}}\"}"
