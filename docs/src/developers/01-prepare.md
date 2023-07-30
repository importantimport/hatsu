# Prepare

To develop Hatsu, you should first [install Rust](https://www.rust-lang.org/tools/install) and some dependencies.

Arch-based distro:

```bash
sudo pacman -S git cargo openssl
```

Debian-based distro:

```bash
sudo apt install git cargo libssl-dev
```

## Clone Repository

It will create a `hatsu` subfolder in the current path.

```bash
git clone https://github.com/importantimport/hatsu.git && cd hatsu
```

## Contributing

Go to the hatsu folder and you can see these:

- [`docs`](https://github.com/importantimport/hatsu/tree/main/docs) - The documentation you're looking at right now, uses [mdBook](https://github.com/rust-lang/mdBook) to build.
- [`migration`](https://github.com/importantimport/hatsu/tree/main/migration) - [SeaORM Migration](https://www.sea-ql.org/SeaORM/docs/migration/setting-up-migration/).
- [`src`](https://github.com/importantimport/hatsu/tree/main/src) - Main application.
