# Feed

For Hatsu to work, your site needs to have one of the valid [JSON](https://jsonfeed.org/version/1.1/) / [Atom](https://en.wikipedia.org/wiki/Atom_(web_standard)) / [RSS](https://en.wikipedia.org/wiki/RSS) feeds.

These feeds should be auto-discoverable on the homepage:

```html
<!-- https://example.com -->
<!DOCTYPE html>
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

## JSON Feed

Hatsu uses `serde` to parse JSON Feed directly, so you can expect it to have first-class support.

Please make sure your feed is valid in [JSON Feed Validator](https://validator.jsonfeed.org/) first.

### JSON Feed Items

Hatsu infers object id from `item.url` and `item.id`.

It will use the `item.url` first, and if it doesn't exist, it will try to convert the `item.id` to an absolute url.

```text
https://example.com/foo/bar => https://example.com/foo/bar
/foo/bar => https://example.com/foo/bar 
foo/bar => https://example.com/foo/bar
```

Ideally, your `item.id` and `item.url` should be consistent absolute links:

```json
{
	"id": "https://example.com/foo/bar",
    "url": "https://example.com/foo/bar",
	"title": "...",
	"content_html": "...",
	"date_published": "..."
}
```

### JSON Feed Extension

If you can customize your site's JSON Feed,
you might also want to check out the [Hatsu JSON Feed Extension](../others/json-feed-extension.md).

## Atom / RSS

Hatsu uses `feed-rs` to parse XML feeds and convert them manually.

Please make sure your feed is valid in [W3C Feed Validation Service](https://validator.w3.org/feed/) first.

This section is currently lacking testing, so feel free to report bugs.
