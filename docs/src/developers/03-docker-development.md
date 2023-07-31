# Docker Development

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

Simply run:

```bash
docker-compose up -d
```

If there is no build image, it will be built automatically at execution time.
Hatsu uses [cargo-chef](https://crates.io/crates/cargo-chef) in the [Dockerfile](https://github.com/importantimport/hatsu/blob/main/Dockerfile),
which caches dependencies to avoid duplicate builds
(i.e. you only need to build Hatsu itself afterwards)
