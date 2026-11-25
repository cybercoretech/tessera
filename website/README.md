# Tessera project website

This is the editable source for the Tessera project website. It is built with the Tessera CLI and its CSS-first Kit.

The navigation includes Cybercore's shared theme chooser, which loads its
stylesheet, browser module, and theme catalog from `cybercoretech.github.io/kit`.
The page maps those shared theme colors onto Tessera's site tokens. If the shared
assets are unavailable, the page keeps its local storm-blue default theme.

The footer uses Cybercore's shared `<cybercore-contact repo="tessera">`
component from `cybercoretech.github.io/kit/contact.js`. It reads contact,
social, and support links from the shared contact data, including the donation
options, and adds a direct link to Tessera's issue tracker. A direct contact
email remains as a fallback if the shared component cannot load.

From this directory, build the static site with:

```sh
cargo run --manifest-path ../Cargo.toml -- build
```

The generated files go into `website/dist/`. The site is deployed manually to [tessera-8gd.pages.dev](https://tessera-8gd.pages.dev/). Cloudflare assigned this hostname because Pages subdomains cannot be changed after project creation.

`wrangler.jsonc` binds the Pages project name to this directory's `dist/` output. Deploy manually from this directory with:

```sh
wrangler pages deploy dist --project-name=tessera --branch=main
```

CI does not publish the site automatically.
