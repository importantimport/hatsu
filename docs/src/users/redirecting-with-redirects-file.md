# Redirecting with Redirects file

Works with [Netlify](https://docs.netlify.com/routing/redirects/#syntax-for-the-redirects-file) and [Cloudflare Pages](https://developers.cloudflare.com/pages/platform/redirects).

## Well Known

Create a `_redirects` file in the SSG static files directory containing the following:

> Replace `hatsu.local` with your Hatsu instance.

```text
/.well-known/host-meta* https://hatsu.local/.well-known/host-meta:splat 307
/.well-known/nodeinfo* https://hatsu.local/.well-known/nodeinfo 307
/.well-known/webfinger* https://hatsu.local/.well-known/webfinger 307
```

## AS2

> Redirects file only applies to `.well-known`.
> for AS2 redirects, you need to use [AS2 Alternate](./redirecting-with-static-files-and-markup.md#as2-alternate).
