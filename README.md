# 🎶 Hatsu

Bring your blog to Fediverse. [WIP]

## Development

Hatsu is in the early stages of development, PR welcome.

```bash
git clone https://github.com/importantimport/hatsu.git
cd hatsu
cargo run
```

The current goal is to refine the database and implement the basic functionality of the Fediverse server.

The ultimate goal of this project is to provide a self-hosted and easy-to-use alternative to [Bridgy Fed](https://github.com/snarfed/bridgy-fed).

It will crawl the feed to get the data instead of going through Webmention / Microformats, but the response from Fediverse will still be converted to a Webmention back to the source.

### Chinese comments?

I'm learning Rust through this project, so I need some Chinese comments to help me understand it. However, I usually add English comments as well, so if you haven't seen them, feel free to open a PR!

## License

Licensed under [AGPLv3](/LICENSE).