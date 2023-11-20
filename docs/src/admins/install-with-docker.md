# Install with Docker

Hatsu has not yet released an docker image (this will be done when v0.1.0 is released)

So for now you need to follow the steps in [Prepare](../developers/prepare.md) and [Docker Development](../developers/development-docker.md).

In order to add other accounts, you also need to set the `HATSU_ACCESS_TOKEN` variable.

This can be any string, but I recommend generating a random uuid v4.

```bash
echo "HATSU_ACCESS_TOKEN = \"$(cat /proc/sys/kernel/random/uuid)\"" >> .env
```
