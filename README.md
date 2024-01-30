# Hatsu「初」

[![MSRV](https://img.shields.io/badge/rust-1.75%2B-red)](.clippy.toml)
[![License](https://img.shields.io/github/license/importantimport/hatsu)](LICENSE)
[![Matrix](https://img.shields.io/matrix/importantimport%3Amatrix.org)](https://matrix.to/#/#importantimport:matrix.org)

Self-hosted & Fully-automated ActivityPub Bridge for Static Sites. [WIP]

## Development

Hatsu has the basic functionality of a bridge, PR welcome.

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
# install just (https://github.com/casey/just)
cargo install just
# list just recipes
just
```

The goal of this project is to provide a self-hosted and easy-to-use alternative to [Bridgy Fed](https://github.com/snarfed/bridgy-fed).

It will crawl the feed ([JSON Feed 1.1](https://jsonfeed.org/version/1.1) / [Atom 1.0](https://validator.w3.org/feed/docs/atom.html) / [RSS 2.0](https://www.rssboard.org/rss-specification)) to get the data instead of going through Webmention / Microformats, but the response from Fediverse will still be converted to a Webmention back to the source.

### Fediverse compatibility

Hatsu uses the same library as [Lemmy](https://github.com/LemmyNet/lemmy), [Activitypub-Federation](https://github.com/LemmyNet/activitypub-federation-rust), so it should behave similarly in practice.

In addition to this, Hatsu is also enabled:

- [signed_fetch_actor](https://docs.rs/activitypub_federation/latest/activitypub_federation/config/struct.FederationConfigBuilder.html#method.signed_fetch_actor)
  - Hatsu performs an HTTP signature on each request. This ensures compatibility with [Mastodon instances with secure mode enabled](https://docs.joinmastodon.org/admin/config/#authorized_fetch) and [GoToSocial](https://docs.gotosocial.org/en/latest/federation/federating_with_gotosocial/#access-control).
- [http_signature_compat](https://docs.rs/activitypub_federation/latest/activitypub_federation/config/struct.FederationConfigBuilder.html#method.http_signature_compat)
  - Like Lemmy, Hatsu enables this by default for [Pleroma](https://git.pleroma.social/pleroma/pleroma/-/issues/2939) compatibility.

If you're not sure whether it's a Hatsu or Activitypub-Federation compatibility issue, you should open new issue in Hatsu first.

### TODO

- Upgrade dependencies
  - axum 0.7
    - https://github.com/LemmyNet/activitypub-federation-rust/issues/87
    - utoipa-swagger-ui 5.0
  - axum-extra 0.9
    - use typed-routing (https://github.com/tokio-rs/axum/issues/2218, https://github.com/ibraheemdev/matchit/issues/13)
  - activitypub-federation 0.5
  - tokio-graceful-shutdown 0.14
    - https://github.com/tokio-rs/axum/pull/2398
    - https://github.com/hyperium/hyper-util/pull/66
  - Using AFIT / RPITIT instead of `async-trait`
- Performance improvements
  - https://github.com/TechEmpower/FrameworkBenchmarks/issues/8501#issuecomment-1780275745
  - sonic-rs (required nightly) / simd-json
  - uuid-simd

### Useful links

#### ActivityPub

- https://github.com/LemmyNet/activitypub-federation-rust/tree/main/examples/
- https://github.com/LemmyNet/lemmy/tree/main/crates/apub/
- https://blog.joinmastodon.org/2018/06/how-to-implement-a-basic-activitypub-server/

#### SeaORM

- https://www.sea-ql.org/SeaORM/docs/basic-crud/basic-schema/
- https://github.com/SeaQL/sea-orm/tree/master/examples/axum_example/
- https://github.com/SeaQL/sea-orm/blob/master/tests/basic.rs

#### OpenAPI

- https://github.com/juhaku/utoipa/tree/master/examples/todo-axum

#### Leptos

- https://github.com/leptos-rs/start-axum
- https://github.com/leptos-rs/start-axum-workspace

#### Shuttle

- https://docs.shuttle.rs/migration/migrating-to-shuttle
- https://github.com/shuttle-hq/shuttle/issues/179#issuecomment-1203536025

## License

Licensed under [AGPLv3](LICENSE).

### Third Party Licenses

This project partially copies code from the following projects:

| Project                                                                                         | License                                                                               |
| ----------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------- |
| [LemmyNet/activitypub-federation-rust](https://github.com/LemmyNet/activitypub-federation-rust) | [AGPL-3.0](https://github.com/LemmyNet/activitypub-federation-rust/blob/main/LICENSE) |
| [LemmyNet/lemmy](https://github.com/LemmyNet/lemmy)                                             | [AGPL-3.0](https://github.com/LemmyNet/lemmy/blob/main/LICENSE)                       |
