# Redirecting with Static files and Markup

This should apply to most hosting services and SSG.

## Well Known

For the `.well-known/*` files, you need to get the corresponding contents from the hatsu instance and output them as a static file.

> Replace `hatsu.local` with your Hatsu instance and `example.com` with your site.

Open your Hatsu instance home page in a browser and F12 -> Console to run:

```js
// .well-known/webfinger
await fetch('https://hatsu.local/.well-known/webfinger?resource=acct:example.com@hatsu.local').then(res => res.text())
// .well-known/nodeinfo
await fetch('https://hatsu.local/.well-known/nodeinfo').then(res => res.text())
// .well-known/host-meta
await fetch('https://hatsu.local/.well-known/host-meta').then(res => res.text()).then(text => text.split('\n').map(v => v.trim()).join(''))
// .well-known/host-meta.json
await fetch('https://hatsu.local/.well-known/host-meta.json').then(res => res.text())
```

This will fetch their text contents,
which you need to save to the SSG equivalent of the static files directory and make sure they are output to the `.well-known` folder.

## AS2 Alternate

> Only Mastodon and Misskey (and their forks) is known to support auto-discovery, other software requires redirection to search correctly.
> [w3c/activitypub#310](https://github.com/w3c/activitypub/issues/310)

Make your posts searchable on Fediverse by setting up auto-discovery.

Since Hatsu's object URLs are predictable, you just need to make sure:

- The page you want to set up for auto-discovery is in the Feed.
- The actual URL of the page is the same as in the Feed. (see [./feed](./feed.md))

That's it! For `https://example.com/foo/bar`, just add the following tag to the `document.head`:

> Replace `hatsu.local` with your Hatsu instance.

```html
<link rel="alternate" type="application/activity+json" href="https://hatsu.local/posts/https://example.com/foo/bar" />
```
