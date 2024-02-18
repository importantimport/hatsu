# Redirecting with [FEP-612d](https://codeberg.org/fediverse/fep/src/branch/main/fep/612d/fep-612d.md)

There doesn't seem to be software currently implements FEP-612d, but that won't stop us from setting it up.

just add the following TXT record:

> Replace `hatsu.local` with your Hatsu instance and `example.com` with your site.

```
_apobjid.example.com https://hatsu.local/users/example.com
```

That's it!
