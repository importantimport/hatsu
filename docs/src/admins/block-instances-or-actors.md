# Block Instances or Actors

> Ensure you set `HATSU_ACCESS_TOKEN` correctly in the [previous section](./environments.md#hatsu_access_token-optional) first, otherwise you will not be able to use the Hatsu Admin API.

## Block URL

Block URL. if path is `/`, it is recognized as an instance.

Each time an activity is received an origin match is performed on blocked instances and an exact match is performed on blocked actors.

```bash
BLOCK_URL="https://example.com" curl -X POST "http://localhost:$(echo $HATSU_LISTEN_PORT)/api/v0/admin/block-url?url=$(echo $BLOCK_URL)&token=$(echo $HATSU_ACCESS_TOKEN)"
```

### Get the Actors URL for a Fediverse user

In Fediverse, we see user IDs typically as `@foo@example.com`. so how do we get the corresponding URL? it's simple. here's an example of a JavaScript environment where you can run it in your browser:

```js
const id = '@Gargron@mastodon.social'

// split id by @ symbol
// ['', 'Gargron', 'mastodon.social']
const [_, user, instance] = id.split('@')

// get webfinger json
const webfinger = await fetch(
  `https://${instance}/.well-known/webfinger?resource=acct:${user}@${instance}`,
  { headers: { accept: 'application/jrd+json' }}
).then(res => res.json())

// find rel=self
const url = webfinger.links.find(({ rel }) => rel === 'self').href

// https://mastodon.social/users/Gargron
console.log(url)
```

That's it! you may also need to open the console on the web page of the instance the account belongs to, given cross-origin issues and such.

## Unblock URL

The unblocked version of the above API, simply replaces the path `/api/v0/admin/block-url` with `/api/v0/admin/unblock-url`.
