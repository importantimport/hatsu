# Backfeed based on KKna

Written by the same authors as Hatsu, KKna provides the simplest integration for Hatsu.

## Examples

> Replace `hatsu.local` with your Hatsu instance.

```html
<script type="module">
  import { defineConfig } from 'https://esm.sh/@kkna/context'
  import { hatsu } from 'https://esm.sh/@kkna/preset-hatsu'

  defineConfig({
    presets: [hatsu({ instance: 'https://hatsu.local' })],
  })
</script>
<script type="module" src="https://esm.sh/@kkna/component-material"></script>
<kkna-material></kkna-material>
```

You can use it with other presets or write your own components, see the [KKna Documentation](https://kkna.js.org/) for details.
