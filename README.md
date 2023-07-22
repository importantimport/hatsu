# 🎶 Hatsu

Bring your blog to Fediverse. [WIP]

## Development

Hatsu is in the early stages of development, PR welcome.

###### Setup

```bash
# clone project
git clone https://github.com/importantimport/hatsu.git
# change directory
cd hatsu
# copy env example
cp .env.example .env
# create database
touch db.sqlite3
```

###### Cargo

```bash
# run
cargo run
# build (debug)
cargo build
# build (release)
cargo build --release
```

###### Docker (possibly broken)

```bash
# docker build
docker build .
# docker compose up
docker-compose up -d
```

###### ORM (optional)

```bash
# install sea-orm-cli
cargo install sea-orm-cli
# migrate
sea-orm-cli migrate up
# generate entity
sea-orm-cli generate entity -o src/entities
```

The current goal is to refine the database and implement the basic functionality of the Fediverse server.

The ultimate goal of this project is to provide a self-hosted and easy-to-use alternative to [Bridgy Fed](https://github.com/snarfed/bridgy-fed).

It will crawl the feed ([JSON Feed](https://jsonfeed.org/version/1.1) / [Atom](https://validator.w3.org/feed/docs/atom.html) / [RSS](https://www.rssboard.org/rss-specification)) to get the data instead of going through Webmention / Microformats, but the response from Fediverse will still be converted to a Webmention back to the source.

### Chinese comments?

I'm learning Rust through this project, so I need some Chinese comments to help me understand it. However, I usually add English comments as well, so if you haven't seen them, feel free to open a PR!

### Environments

| Environment    | Default   | Remarks |
| -------------- | --------- | ------- |
| `HATSU_DOMAIN` | undefined |         |
| `DATABASE_URL` | undefined |         |

## License

Licensed under [AGPLv3](/LICENSE).
