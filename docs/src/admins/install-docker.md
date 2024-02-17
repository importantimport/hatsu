# Docker Installation

## Stable

Doesn't have a stable version yet.

## Nightly

Hatsu provides nightly Docker image builds.

Please note that it is unstable and may contain breaking changes.

```bash
docker pull ghcr.io/importantimport/hatsu:nightly
```

Run with `docker run`:

```bash
docker run -d \
  --name hatsu \
  --restart unless-stopped \
  -p 3939:3939 \
  -v /opt/hatsu/hatsu.sqlite3:/app/hatsu.sqlite3 \
  -e HATSU_DATABASE_URL=sqlite://hatsu.sqlite3 \
  -e HATSU_DOMAIN={{hatsu-instance-domain}} \
  -e HATSU_LISTEN_HOST=0.0.0.0 \
  -e HATSU_PRIMARY_ACCOUNT={{your-static-site}} \
  -e HATSU_ACCESS_TOKEN=edc7e5a7-f2ce-44a6-a57e-b7706503f642 \
  ghcr.io/importantimport/hatsu:nightly
```

You need to specify all environment variables at once. For more information, see [Environments](./environments.md).

<!-- You can also use the docker-compose example in examples. (TODO) -->
