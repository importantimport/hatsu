# Docker Installation

> Hatsu uses the `x86-64-v3` target architecture for optimal performance.
>
> If you are using an older processor, you currently need to build locally and change the corresponding values in `.cargo/config.toml`.

You can find images on GitHub: [https://github.com/importantimport/hatsu/pkgs/container/hatsu](https://github.com/importantimport/hatsu/pkgs/container/hatsu)

Hatsu uses three primary tags: `latest` (stable), `beta` and `nightly`, literally.

## docker run

> Replace `{{version}}` with the version you want to use.

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
  -e HATSU_ACCESS_TOKEN=123e4567-e89b-12d3-a456-426614174000 \
  ghcr.io/importantimport/hatsu:{{version}}
```

You need to specify all environment variables at once. For more information, see [Environments](./environments.md).

## docker compose

The [examples](https://github.com/importantimport/hatsu/tree/main/examples) folder contains some sample docker compose configurations,

You can make your own modifications based on them.
