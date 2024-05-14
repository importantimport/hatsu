# Hatsu JSON Feed Extension

To allow you to customize your postings, Hatsu defines a JSON Feed extension that uses the `_hatsu` key.

All extension keys for the Hatsu JSON Feed Extension are optional.

> Note: everything here is experimental. It is always subject to breaking changes and does not follow semver.

## Top-level

The following applies to the [Top-level JSON Feed](https://www.jsonfeed.org/version/1.1/#top-level-a-name-top-level-a).

- `about` (optional but strongly recommended, string) is the URL used to introduce this extension to humans. should be [https://github.com/importantimport/hatsu/issues/1](https://github.com/importantimport/hatsu/issues/1) .
- `aliases` (optional, string) is the customized username used for [FEP-4adb](https://codeberg.org/fediverse/fep/src/branch/main/fep/4adb/fep-4adb.md) and [FEP-2c59](https://codeberg.org/fediverse/fep/src/branch/main/fep/2c59/fep-2c59.md).
- `banner_image` (optional, string) is the URL of the banner image for the website in hatsu.

## Items

The following applies to the [JSON Feed Item](https://www.jsonfeed.org/version/1.1/#items-a-name-items-a).

- `about` (optional, string) is the URL used to introduce this extension to humans. should be [https://github.com/importantimport/hatsu/issues/1](https://github.com/importantimport/hatsu/issues/1) .
