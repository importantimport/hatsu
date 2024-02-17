# Introduction

Hatsu is a self-hosted bridge that interacts with Fediverse on behalf of your static site.

Normally it can do all the:

- When a Fediverse user searches for a user of your site (`@catch-all@example.com`), [redirects](./users/redirecting-with-static-files-and-markup.md#well-known) to the corresponding user of the Hatsu instance.
- When a Fediverse user searches for your site URL (`https://example.com/hello-world`), [redirects](./users/redirecting-with-static-files-and-markup.md#as2-alternate) to the corresponding post on the Hatsu instance.
- Accepts follow requests and pushes new posts to the follower's homepage as they become available.
- Receive replies from Fediverse users and [backfeed](./users/backfeed.md) to your static site.

Best of all, these are fully automated! Just set it up once and you won't need to do anything else.

## Comparison

Hatsu is still a Work-In-Progress. It is similar to Bridgy Fed but different:

- Hatsu uses Feed (JSON / Atom / RSS) as a data source instead of HTML pages with microformats2.
- Hatsu doesn't require you to automatically or manually send Webmention reminders for create and update, it's all fully automated.
- Hatsu is ActivityPub only, which means it doesn't handle Nostr, AT Protocol (Bluesky) or other protocols.

If you don't want to self-host, you may still want to use Bridgy Fed or Bridgy in some cases:

### Bridgy Fed

- You don't mind compatibility with platforms other than Mastodon.
- Your site has good microformats2 markup.

### Bridgy

- You already have a Fediverse account ready to be used for this purpose.
- Your site has good microformats2 markup.
