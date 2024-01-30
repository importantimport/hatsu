# Create Account

Ensure you set `HATSU_ACCESS_TOKEN` correctly in the [previous section](./environments.md#hatsu_access_token-optional) first, otherwise you will not be able to use the Hatsu Admin API.

## just

The easiest way to create an account is the [`just`](https://github.com/casey/just) command line tool:

```bash
just account create example.com
```

If you are using docker, you need to exec to the container first.

```bash
docker exec -it hatsu /bin/bash
```

## curl

You can also access the API via curl, as `Justfile` does.

```bash
curl -X POST "http://localhost:$(echo $HATSU_LISTEN_PORT)/api/v0/admin/create-account?token=$(echo $HATSU_ACCESS_TOKEN)" \
  -H "Content-Type: application/json" \
  -d "{\"name\": \"example.com\"}"
```
