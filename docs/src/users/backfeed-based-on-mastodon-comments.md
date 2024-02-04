# Backfeed based on Mastodon Comments

## Examples

"Mastodon Comments" refers to the [`@oom/mastodon-components`](https://github.com/oom-components/mastodon-comments) library.

```html
<script type="module">
  import Comments from 'https://esm.run/@oom/mastodon-comments'
  customElements.define('oom-comments', Comments)
</script>
<link
  rel="stylesheet"
  href="https://cdn.jsdelivr.net/npm/@oom/mastodon-comments/src/styles.css"
/>
<oom-comments src="https://mastodon.gal/@misteroom/110810445656343599">
  No comments yet
</oom-comments>
```

The basic example should look something like the above, where [https://mastodon.gal/@misteroom/110810445656343599](https://mastodon.gal/@misteroom/110810445656343599) is the link to the post in Fediverse.

Hatsu uses predictable URLs, you just need to change the src:

```js
// trim url
// input:
// https://example.com/foo/bar#baz
// https://example.com/foo/bar?baz=qux
// output:
// https://example.com/foo/bar
const { origin, pathname } = new URL(window.location.href)
const url = new URL(pathname, origin).href

// get id (base64url encode)
// aHR0cHM6Ly9leGFtcGxlLmNvbS9mb28vYmFy
const id = btoa(url).replaceAll('+', '-').replaceAll('/', '_')

// oom-comments src
// https://hatsu.local/notice/aHR0cHM6Ly9leGFtcGxlLmNvbS9mb28vYmFy
const src = new URL(`/notice/${id}`, 'https://hatsu.local').href
```

So eventually it will look like this:

```html
<script type="module">
  import Comments from 'https://esm.run/@oom/mastodon-comments'
  customElements.define('oom-comments', Comments)
</script>
<link
  rel="stylesheet"
  href="https://cdn.jsdelivr.net/npm/@oom/mastodon-comments/src/styles.css"
/>
<oom-comments
  src="https://hatsu.local/notice/aHR0cHM6Ly9leGFtcGxlLmNvbS9mb28vYmFy"
>
  No comments yet
</oom-comments>
```

It's a real pain in the ass, but you can try to automate it.

### Lume

If you're using [Lume](https://github.com/lumeland/lume) and [Theme Simple Blog](https://github.com/lumeland/theme-simple-blog), it will read `data.comments.src`.

So you can do this:

```ts
// _config.ts
import lume from 'lume/mod.ts'
import blog from 'https://deno.land/x/lume_theme_simple_blog@v0.14.0/mod.ts'

const site = lume()

site.use(blog())

// add this:
site.preprocess(['.md'], (pages) =>
  pages
    .filter((page) => page.type === 'post')
    .forEach((page) => {
      page.data.comments = {
        src: new URL(
          `/notice/${btoa(site.url(page.data.url, true))
            .replaceAll('+', '-')
            .replaceAll('/', '_')}`,
          'https://hatsu.local' // your hatsu instance
        ).href,
      }
    })
)

export default site
```

## How it works?

Hatsu mimics Pleroma's URL format.

`@oom/mastodon-components` will extract the ID from the URL and query the corresponding API for the data.

```js
// oom-comments src
const src = 'https://hatsu.local/notice/aHR0cHM6Ly9leGFtcGxlLmNvbS9mb28vYmFy'
// origin: 'https://hatsu.local
// pathname: '/notice/aHR0cHM6Ly9leGFtcGxlLmNvbS9mb28vYmFy'
const { origin, pathname } = new URL(src)
// id: 'aHR0cHM6Ly9leGFtcGxlLmNvbS9mb28vYmFy'
const [, id] = pathname.match(/^\/notice\/([^\/?#]+)/)
// api url: https://hatsu.local/api/v1/statuses/aHR0cHM6Ly9leGFtcGxlLmNvbS9mb28vYmFy/context
const url = new URL(`/api/v1/statuses/${id}/context`, origin)
```

Upon receiving a request, Hatsu's corresponding API will attempt to decode the base64url ID and return the data.

If you're interested in the code, you can also take a look at [routes/statuses/status_context.rs](https://github.com/importantimport/hatsu/blob/main/crates/api_mastodon/src/routes/statuses/status_context.rs) and [entities/context.rs](https://github.com/importantimport/hatsu/blob/main/crates/api_mastodon/src/entities/context.rs).
