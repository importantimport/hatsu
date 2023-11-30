# Docker Development

> You'll need to complete [prepare](./01-prepare.md) before you do this.

## Dependencies

To use Docker, you only need to install Docker and Docker Compose.

```bash
# Arch-based distro
sudo pacman -S docker docker-compose

# Debian-based distro
sudo apt install docker.io docker-compose
```

<!-- ## Building -->

## Running

First copy the variables,

Set `HATSU_DOMAIN` to your prepared domain
(e.g. `hatsu.example.com` without `https://`)

and `HATSU_PRIMARY_ACCOUNT` to your desired user domain
(e.g. `blog.example.com` without `https://`)

```bash
# copy env example
cp .env.example .env
# edit env
nano .env
```

Then create the database file and run:

```bash
# create database
touch hatsu.sqlite3
# run hatsu
docker-compose up -d
```

If there is no build image, it will be built automatically at execution time.
Hatsu uses [cargo-chef](https://crates.io/crates/cargo-chef) in the [Dockerfile](https://github.com/importantimport/hatsu/blob/main/Dockerfile),
which caches dependencies to avoid duplicate build dependencies.

If you need to rebuild, add the `--build` flag:

```bash
docker-compose up -d --build
```
