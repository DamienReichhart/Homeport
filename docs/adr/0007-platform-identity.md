# A neutral identity with one red signal

Homeport's first identity was a harbour: a house that was also a shipping container, in orange, over a tide wave, on a deep-blue gradient tile, with Harbour blue for actions and Geist type. It said "home" and "container", but it pulled towards nautical kitsch, its blues sat close to Docker's own, and its orange accent appeared wherever the brand wanted a moment (cursors, search hits, badges), so it meant nothing in particular.

The project's position is that Homeport stays on your machine and visits your servers: no agent, no web panel, no account. The feeling it should give in three seconds is **steady**. The new identity ("Platform", after the railway station clock it borrows from) is built for that:

- **Two neutrals and one signal.** Ink and Paper do all the work. Signal red has one meaning, "this needs you", and is used for nothing else. There is no accent colour for links or selection.
- **Healthy is plain.** A running container is an ink disc, not a green one. Status is a shape as well as a colour (disc, ring, diamond), so it reads in greyscale and for colour-blind users.
- **A mark that draws the claim**: a square (this machine), a line (the connection) and a hollow red ring (a Host with nothing installed on it).
- **Host Grotesk and Fragment Mono**, both under the SIL OFL. Fragment Mono's single weight is used as a rule: machine text is never bold.
- **Flat, square, still.** Radius 4 px, hairline borders, no gradients, shadows, blur or hover motion.

`docs/BRAND.md` holds the rules and values.

## Considered Options

- **Keep the house-container mark and warm the palette** (cream, walnut, a milk-paint orange): the cheapest change and keeps recognition, but a house fights the "never moves in" position, and cream with clay orange is the default warm look of many current products.
- **Status as a flag code** (flat yellow and black, flag-shaped status pips): the clearest status language, but yellow and black is the universal hazard code, the opposite of steady, and flags next to the name Homeport invite the nautical kitsch the identity is leaving behind. Its shape-coded status was kept.
- **A dark-first surveying identity** with a magenta signal: its mark (here, a line, a hollow circle) was the best of the candidates and was kept, but magenta is energetic rather than steady and sits close to error red.

## Consequences

- There is no green. That is the main risk: tools have trained users to scan for green dots. If testing shows "all ink" reads as "nothing is running", `--success` becomes a muted green and red stays the only loud colour.
- `--beacon` is gone; `--signal` replaces it and is only for what needs the user. Cursors, search hits and chart series use neutrals (search's current match uses amber).
- Charts can no longer rely on hue to tell series apart: a second series in a chart is dashed.
- The website's hero is a typeset board of Hosts, not a screenshot. Screenshots are still captured from the real UI with mocked IPC (`bun run screenshots`, with Firefox or `CHROMIUM=…`).
- Host Grotesk's hyphen is wide, so hyphenated names read a little like dashes in the sans. Setting machine identifiers in Fragment Mono is the follow-up.
- The tagline changes from "docked in one place" to "on one board", dropping the last nautical word except the name itself.
