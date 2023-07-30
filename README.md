# Hatsu「初」

Bring your static site to Fediverse. [WIP]

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
touch hatsu.sqlite3
```

###### Cargo

```bash
# run
cargo run
# build (debug)
cargo build
# build (release)
cargo build --release
# install
cargo install
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

It will crawl the feed ([JSON Feed 1.1](https://jsonfeed.org/version/1.1) / [Atom 1.0](https://validator.w3.org/feed/docs/atom.html) / [RSS 2.0](https://www.rssboard.org/rss-specification)) to get the data instead of going through Webmention / Microformats, but the response from Fediverse will still be converted to a Webmention back to the source.

### Chinese comments?

I'm learning Rust through this project, so I need some Chinese comments to help me understand it. However, I usually add English comments as well, so if you haven't seen them, feel free to open a PR!

### Environments

| Environment          | Default (.env.example)   | Remarks |
| -------------------- | ------------------------ | ------- |
| `DATABASE_URL`       | `sqlite://hatsu.sqlite3` |         |
| `HATSU_DOMAIN`       | `hatsu.local`            |         |
| `HATSU_LISTEN`       | `localhost:3939`         |         |
| `HATSU_TEST_ACCOUNT` | `example.com`            |         |

### Fediverse compatibility

Hatsu uses the same library as [Lemmy](https://github.com/LemmyNet/lemmy), [Activitypub-Federation](https://github.com/LemmyNet/activitypub-federation-rust), so it should behave similarly in practice.

If you're not sure whether it's a Hatsu or Activitypub-Federation compatibility issue, you should open new issue in Hatsu first.

### Useful links

- https://github.com/LemmyNet/activitypub-federation-rust/tree/main/examples/
- https://github.com/LemmyNet/lemmy/tree/main/crates/apub/
- https://github.com/SeaQL/sea-orm/tree/master/examples/axum_example/
- https://blog.joinmastodon.org/2018/06/how-to-implement-a-basic-activitypub-server/

## License

Licensed under [AGPLv3](/LICENSE).

### Third Party Licenses

This project partially copies code from the following projects:

| Project                                                                                         | License                                                                               |
| ----------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------- |
| [LemmyNet/activitypub-federation-rust](https://github.com/LemmyNet/activitypub-federation-rust) | [AGPL-3.0](https://github.com/LemmyNet/activitypub-federation-rust/blob/main/LICENSE) |
| [LemmyNet/lemmy](https://github.com/LemmyNet/lemmy)                                             | [AGPL-3.0](https://github.com/LemmyNet/lemmy/blob/main/LICENSE)                       |
