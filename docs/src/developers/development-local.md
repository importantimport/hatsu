# Local Development

> You'll need to complete [prepare](./01-prepare.md) before you do this.

## Dependencies

To develop Hatsu, you should first [install Rust](https://www.rust-lang.org/tools/install) and some dependencies.

```bash
# Arch-based distro
sudo pacman -S git cargo openssl

# Debian-based distro
sudo apt install git cargo libssl-dev
```

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
cargo run
```

Hatsu now listen on `localhost:3939`, and in order for it to connect to Fediverse, you'll also need to set up a reverse proxy.
