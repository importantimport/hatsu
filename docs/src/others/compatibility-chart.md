# Compatibility Chart

Hatsu is primarily geared towards the micro-blogging platform in the Fediverse.

Currently I've created a chart for all the platforms I expect to be compatible with, and hopefully it will be filled in later:

## Send

|              | [Create] (Note) | [Accept] (Follow) |
| ------------ | --------------- | ----------------- |
| [Mastodon]   | ✅              | ✅                |
| [GoToSocial] |                 |                   |
| [Misskey]    |                 |                   |
| [Pleroma]    | ✅              |                   |

## Receive

|              | [Follow] |     |
| ------------ | -------- | --- |
| [Mastodon]   | ✅       |     |
| [GoToSocial] |          |     |
| [Misskey]    |          |     |
| [Pleroma]    |          |     |

[Akkoma], [Sharkey], etc. forks should be compatible with upstream, so they are not listed separately.

[Create]: https://www.w3.org/ns/activitystreams#Note
[Accept]: https://www.w3.org/ns/activitystreams#Accept
[Follow]: https://www.w3.org/ns/activitystreams#Follow

[Mastodon]: https://github.com/mastodon/mastodon
[GoToSocial]: https://github.com/superseriousbusiness/gotosocial
[Misskey]: https://github.com/misskey-dev/misskey
[Pleroma]: https://git.pleroma.social/pleroma/pleroma/
[Akkoma]: https://akkoma.dev/AkkomaGang/akkoma/
[Sharkey]: https://activitypub.software/TransFem-org/Sharkey
