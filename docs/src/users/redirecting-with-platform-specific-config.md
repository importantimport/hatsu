# Redirecting with Platform-Specific Configuration

Works with [Netlify](https://docs.netlify.com/routing/redirects/#syntax-for-the-netlify-configuration-file) and [Vercel](https://vercel.com/docs/projects/project-configuration#redirects).

## Well Known

### Netlify (`netlify.toml`)

Create a `netlify.toml` file in the root directory containing the following:

> Replace `hatsu.local` with your Hatsu instance.

```toml
[[redirects]]
  from = "/.well-known/host-meta*"
  to = "https://hatsu.local/.well-known/host-meta:splat"
  status = 307
[[redirects]]
  from = "/.well-known/nodeinfo*"
  to = "https://hatsu.local/.well-known/nodeinfo"
  status = 307
[[redirects]]
  from = "/.well-known/webfinger*"
  to = "https://hatsu.local/.well-known/webfinger"
  status = 307
```

### Vercel (`vercel.json`)

Create a `vercel.json` file in the root directory containing the following:

> Replace `hatsu.local` with your Hatsu instance.

```json
{
  "redirects": [
    {
      "source": "/.well-known/host-meta",
      "destination": "https://hatsu.local/.well-known/host-meta"
    },
    {
      "source": "/.well-known/host-meta.json",
      "destination": "https://hatsu.local/.well-known/host-meta.json"
    },
    {
      "source": "/.well-known/nodeinfo",
      "destination": "https://hatsu.local/.well-known/nodeinfo"
    },
    {
      "source": "/.well-known/webfinger",
      "destination": "https://hatsu.local/.well-known/webfinger"
    }
  ]
}
```

## AS2

> Redirects file only applies to `.well-known`.
> for AS2 redirects, you need to use [AS2 Alternate](./redirecting-with-static-files-and-markup.md#as2-alternate).
