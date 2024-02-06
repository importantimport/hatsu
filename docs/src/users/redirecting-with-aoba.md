# Redirecting with [Aoba](https://github.com/fedikit/aoba) (Lume & Hono)

SSG plugin for Lume and Server Middleware for Deno Deploy and Netlify.

Aoba provides some plugins and server middleware for Lume and Hono,
including Hatsu integration.

## [Lume](https://github.com/fedikit/aoba/blob/main/lume/plugins/hatsu.ts)

The Lume plugin will do what you did in [Redirecting with Static files and Markup](./redirecting-with-static-files-and-markup.md) for you.

> Replace `hatsu.local` with your Hatsu instance and `example.com` with your site.

```ts
import lume from 'lume/mod.ts'
import { hatsuPlugin } from 'aoba/lume/plugins/hatsu.ts'

export default lume({ location: new URL('https://example.com') })
    .use(hatsuPlugin({
        // Hatsu instance
        instance: new URL('https://hatsu.local'),
        // match /posts/*
        match: [/^\/posts\/(.+)$/],
    }))
```

### [Lume Server](https://github.com/fedikit/aoba/blob/main/lume/middlewares/hatsu.ts)

On top of that, the Lume server middleware can redirect `.well-known/*` and AS2 request.

> Replace `hatsu.local` with your Hatsu instance.

```ts
import Server from 'lume/core/server.ts'
import site from './_config.ts'
import { hatsuMiddleware } from 'aoba/lume/middlewares/hatsu.ts'

const server = new Server()

server.use(hatsuMiddleware({
    // Hatsu instance
    instance: new URL('https://hatsu.local'),
    // site location
    location: site.options.location,
}))

server.start()
```

## [Hono](https://github.com/fedikit/aoba/blob/main/hono/middlewares/hatsu.ts)

It's not published to npm, so feel free to copy and paste it if you need to use it in a Node.js.

> Replace `hatsu.local` with your Hatsu instance.

```ts
import { Hono } from 'hono'
import { hatsuWellKnown, hatsuObject } from 'aoba/hono/middlewares/hatsu.ts'

const app = new Hono()
const instance = new URL('https://hatsu.local')

// https://example.com/.well-known/* => https://hatsu.local/.well-known/*
app.use('/.well-known/*'， hatsuWellKnown({ instance }))
// https://example.com/posts/foo => https://hatsu.local/posts/https://example.com/posts/foo
app.use('/posts/*', hatsuObject({ instance }))
```

<!-- ### Deno Deploy

### Netlify -->

<!-- ## Well Known

## AS2 -->
