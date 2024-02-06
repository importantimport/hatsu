# Redirecting

There are two types of redirects required by Hatsu:

1. Well Known files, redirecting them to make your username searchable.

    - before: `https://example.com/.well-known/webfinger?resource=acct:carol@example.com`
    - after: `https://hatsu.local/.well-known/webfinger?resource=acct:carol@example.com`

2. Requests accept of type `application/activity+json`, redirecting them to make your page searchable.
    - before: `https://example.com/foo/bar`
    - after: `https://hatsu.local/posts/https://example.com/foo/bar`

There are many ways to redirect them and you can pick one you like:

## [with Static files and Markup](./redirecting-with-static-files-and-markup.md)

This should apply to most hosting services and SSG.

## [with Redirects file](./redirecting-with-redirects-file.md)

Works with Netlify and Cloudflare Pages.

## [with Platform-Specific Configuration](./redirecting-with-platform-specific-config.md)

Works with Netlify and Vercel.

## [with Aoba (Lume & Hono)](./redirecting-with-aoba.md)

SSG plugin for Lume and Server Middleware for Deno Deploy and Netlify.
