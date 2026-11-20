# The website as a line map

The website grew up under the Platform identity (ADR 0007): a Swiss-railway page with a typeset departure board of Docker Hosts as its hero, then a grid of thirteen identical feature cards and five screenshot sections of long paragraphs. Two things had stopped being true. Its headline, "Every Docker Engine, on one board.", undersold a product that now reaches Podman, containerd, Incus, Swarm, Kubernetes and Nomad. And its structure answered "what are all the features" rather than the visitor's real questions: does it work with what I run, is it safe for my servers, what does it look like, how do I get it.

The website is now drawn as a **line map**, a transit diagram of the visitor's fleet. This machine is the terminus; each way of reaching an Engine (local socket, SSH, TLS, TCP, a cluster API) is a line of its own colour; each Engine is a station; the one station that needs you is red. The mark already drew this (a square, a line, a ring), so the identity keeps it and lets it run across the page.

- **The home page is rebuilt around the map**: the hero is an example fleet drawn as a map (labelled as an example), then the questions people open Homeport for (is it up, why did it die, what changed, then the change) each answered by a real screenshot on one line of stops, then the map's key written out as what each connection needs on the server, then an index of every Engine, the keys section and the download.
- **The download page, the changelog and the docs** use the same system: platform rows instead of cards, releases as stations on a line, and in the docs a rail with stations for the sidebar, "On this page" and numbered steps, where the page you are on is the filled station.
- **Type is Fira Sans and Fira Mono**, a humanist sans from Erik Spiekermann's signage lineage (Meta, the BVG and DB wayfinding), with Cyrillic and Greek for the Russian pages.
- **Colour**: map white or the night map, ink, five line colours that live only on lines and their key, and Signal red still only for "this needs you". No gradients, glows or shadows; 2 px corners.
- **No scroll fades.** The only motion is the map drawing itself once on load, not at all under reduced motion.
- **The tagline is now "One app for every container Engine."**

`docs/BRAND.md` § Website holds the values.

## Considered Options

- **Keep the railway board and rebuild only the structure and copy**: the least surprising path, but a board of Docker Hosts is the old, narrower claim, and the page would stay recognisably the same.
- **A permit-to-work pad and hang tags** (each Host a tag clipped on, never installed; red danger tag for what needs you): distinctive and true to "nothing moves in", but it tips into construction-site costume and its forms are a poor frame for long docs.

## Consequences

- **The website and the app no longer share a typeface or a tagline.** The app keeps Host Grotesk, Fragment Mono and "Every Docker Engine, on one board." (`src/shared/i18n/en/settings.ts`, `src-tauri/tauri.conf.json`). The screenshots on the site still show the app as it is. Bringing the app's tagline in line is a small follow-up; bringing its type in line is a decision for the app.
- The line colours are the first hues on the site beyond red and amber. They are kept off text and surfaces so that red still reads as the only alarm, and each is at least 3:1 against both grounds (`src/shared/styles/contrast.test.ts`).
- The home page's copy is new in all twelve languages; strings whose English did not change kept their translations.
