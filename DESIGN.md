# Design

How Homeport's interface is built: the rules every page follows. `docs/BRAND.md` owns the identity (colour, type, mark, voice); `PRODUCT.md` owns who it is for. This file is about the screens.

## Principles

1. **What needs you comes first; what is healthy stays quiet.** Colour is spent only on state: red for "this needs you", amber for "this needs a look", a muted green disc for running. Healthy rows carry no badge that shouts.
2. **One primary action per page.** The page header holds at most one solid button (the thing people come to do: Run container, New stack, Pull image), one or two outline buttons for frequent secondary actions, and an overflow menu (`⋯`, "More actions") for the rest. Rows never carry a solid button.
3. **Lists open items beside them.** A list whose items have pages declares them with `splitRoutes()`; from 1280 px the item opens next to the list (DataTable keeps the primary column, plus any column marked `meta: { compact: true }`), and Up and Down move through the list.
4. **Danger is set apart and proportionate.** Stop and restart ask nothing. Removing something that can be recreated asks once and names the Host. Losing data for good (a volume, an Instance's disk, a purge) asks for the name to be typed (`typeToConfirm`). Forgetting or deleting what a page is about sits in a `DangerZone` at its foot.
5. **Machine text looks like machine text.** IDs, paths, ports, image references and commands are in Fragment Mono; everything else in Host Grotesk.
6. **Secrets are hidden until asked for.** Environments use `EnvTextarea`; secret values are revealed per action, never stored in view.

## Layout

- The shell is a 240 px sidebar and the page. The sidebar holds search (the command palette), the Board, the fleet (every Host, Swarm and Cluster with its status shape; the selected one opens onto its pages), and the app's own links at its foot.
- Pages are padded 24 px. A page starts with `PageHeader`: title (20 px, semibold), an optional one-line description, actions on the end side. Inside a split view's narrow pane the header shows the title only.
- Sections inside a page are separated by 32 px; related controls by 8–12 px. A section heading is 16 px semibold, sentence case, never an eyebrow above it.
- Cards are for a group of facts about one thing (Engine, Disk usage). Never a card inside a card, never a grid of identical icon cards as a page.

## Components

- **Buttons**: `default` (solid ink) once per page; `outline` for secondary; `ghost` for toolbar and row icons (always with an `aria-label` and a tooltip); `destructive` (tinted) for a remove in a menu or a danger zone; `danger` (solid red) only as the final confirm of deleting data.
- **Row actions**: at most two icon buttons visible (the most frequent, e.g. Restart and Stop/Start), the rest in the row's `⋯` menu. Stop is `CircleStopIcon`, start `CirclePlayIcon`, never a bare square.
- **Tables**: `DataTable`; header text 12 px muted; rows 40 px, one line; numbers right-aligned and tabular; secondary text muted; the identifying column first. No column may make the table overflow its container at 1280 px: truncate with a `title`.
- **Badges**: 4 px corners, outline by default. `success` (green outline) and `warning` (amber outline) for states worth naming; `destructive` for what needs you. Never solid black for a neutral state like "In use".
- **Status**: `StatusDot` shapes (green disc running, grey ring stopped, amber diamond warning, red disc failing, ink ring connecting), always with a text label or a tooltip.
- **Empty states**: say what the page is for and offer the one action that fills it.
- **Tabs**: underlined (`line`) for the sections of a page; the segmented `default` variant only to choose what a list shows.

## Motion

Colour and opacity only, 150 ms. Nothing slides, zooms, lifts or bounces; spinners keep turning (slower under reduced motion).

## Words

The glossary's terms (Host, Engine, Stack, Board, Overview, Needs you…). Buttons say their verb ("Deploy", "Pull image"); confirmations name what goes and where. Every string is in all 12 catalogues; new terms get a row in each `docs/i18n/terms-*.md`.
