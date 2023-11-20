# Feed

For Hatsu to work, your site needs to have one of the valid [JSON](https://jsonfeed.org/version/1.1/) / [Atom](https://en.wikipedia.org/wiki/Atom_(web_standard)) / [RSS](https://en.wikipedia.org/wiki/RSS) feeds.

Hatsu uses serde direct parsing of JSON feeds internally and parses Atom / RSS via [`feed-rs`](https://github.com/feed-rs/feed-rs), so JSON Feed are supported first-class.

These feeds should be auto-discoverable on the homepage:

```html
<html>
    <head>
        ...
        <link rel="alternate" type="application/feed+json" href="https://example.com/feed.json" />
        <link rel="alternate" type="application/atom+xml" href="https://example.com/atom.xml" />
        <link rel="alternate" type="application/rss+xml" href="https://example.com/rss.xml" />
    </head>
    <body>
        ...
    </body>
</html>
```

Hatsu detects all available feeds and prioritizes them in order of `JSON > Atom > RSS`.

If you can customize your site's JSON Feed,
you might also want to check out the [Hatsu JSON Feed Extension](../others/json-feed-extension.md).
