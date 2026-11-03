# Package repositories on GitHub Pages, rebuilt from Releases, at a permanent URL

The APT and RPM repositories (and the in-app updater's manifest) are served from the website's GitHub Pages site, under `https://damienreichhart.github.io/Homeport/`. Pages deploys a fresh artifact each time and can't redirect, so the repositories can't point at release downloads and can't keep state between deploys: every Pages build downloads the `.deb` and `.rpm` files of the latest 5 stable releases from GitHub Releases, regenerates the metadata and signs it. GitHub Releases stay the single source of truth, and nothing is committed to git. Prereleases are left out.

The URL is permanent. It is written into every user's package sources, and Pages can't redirect, so moving to a custom domain would silently stop updates for everyone already installed. No custom domain is planned. If one is ever needed, the old URL has to keep serving the repositories alongside it.

## Considered Options

- **A `gh-pages` branch holding the packages**: keeps old versions without re-downloading them, but puts binaries in git history and needs a second way of deploying Pages.
- **A hosted repository service (Cloudsmith, packagecloud, Gemfury)**: less work, but another account, another trust root and another URL to keep alive.

## Consequences

- Each Pages build gets longer, and the site gets bigger, with each release kept (about 20 MB per release; Pages allows 1 GB).
- Signing the repositories needs the private key as a secret in the Pages workflow, not just the Release workflow.
