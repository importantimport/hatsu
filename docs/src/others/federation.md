# Federation in Hatsu

## Supported federation protocols and standards

- [ActivityPub](https://www.w3.org/TR/activitypub/) (Server-to-Server)
- [Http Signatures](https://datatracker.ietf.org/doc/html/draft-cavage-http-signatures)
- [WebFinger](https://webfinger.net/)
- [NodeInfo](https://nodeinfo.diaspora.software/)
- [Web Host Metadata](https://datatracker.ietf.org/doc/html/rfc6415)

## Supported FEPs

- [FEP-67ff: FEDERATION.md](https://codeberg.org/fediverse/fep/src/branch/main/fep/67ff/fep-67ff.md)
- [FEP-f1d5: NodeInfo in Fediverse Software](https://codeberg.org/fediverse/fep/src/branch/main/fep/f1d5/fep-f1d5.md)
- [FEP-fffd: Proxy Objects](https://codeberg.org/fediverse/fep/src/branch/main/fep/fffd/fep-fffd.md)

## ActivityPub

The following activities and object types are supported:

- `Follow(Actor)`, `Undo(Follow)`
- `Create(Note)`
<!-- - `Create(Note)`, `Update(Note)`, `Delete(Note)` -->
<!-- - `Like(Note)`, `Undo(Like)` -->
<!-- - `Announce(Note)`, `Undo(Announce)` -->

Activities are implemented in way that is compatible with Mastodon and other popular ActivityPub servers.

### Notable differences

- No shared inbox.

## Additional documentation

- [Hatsu Documentation](https://hatsu.cli.rs)
