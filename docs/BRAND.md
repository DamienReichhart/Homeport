# Brand

Homeport is the harbour your containers come home to. The identity says that once, plainly: a house that is also a shipping container, docked on the tide.

Tagline: **Every Docker Engine, docked in one place.**

## Mark

<img src="../assets/brand/mark.svg" alt="" width="96" /> &nbsp; <img src="../assets/brand/glyph.svg" alt="" height="96" />

- **House-container** in Beacon orange: a gable roof over four door bars, like a container's.
- **Tide** wave beneath it: the port.
- **Harbour tile**: a vertical gradient from deep sea blue to night navy, with a faint rim so it holds its edge on dark backgrounds.

Use the **mark** (tile) wherever the product is represented: the app icon, the sidebar, the welcome screen, About. Use the **glyph** (no tile) only on light surfaces where a tile would be too heavy. Don't recolour, rotate, outline or add effects to either; don't put the glyph on dark or busy backgrounds — use the mark.

Minimum sizes: mark 16 px, lockup 24 px high. Keep clear space of at least a quarter of the mark's width around it.

## Lockup

The mark beside the word **Homeport** in Geist SemiBold, tracking −2 %. Capital H, as in all copy.

| File                                                                | Use                                               |
| ------------------------------------------------------------------- | ------------------------------------------------- |
| [`assets/brand/lockup-light.svg`](../assets/brand/lockup-light.svg) | Light backgrounds (wordmark in Ink)               |
| [`assets/brand/lockup-dark.svg`](../assets/brand/lockup-dark.svg)   | Dark backgrounds (wordmark in Foam)               |
| [`assets/brand/mark.svg`](../assets/brand/mark.svg)                 | The mark alone; also `public/favicon.svg`         |
| [`assets/brand/glyph.svg`](../assets/brand/glyph.svg)               | Tile-less mark for light surfaces                 |
| [`assets/app-icon.svg`](../assets/app-icon.svg)                     | Source of every app icon (1024 canvas, 64 margin) |

In the app, use `<LogoMark />` and `<Logo />` from `@/shared/ui`; they draw the same SVG inline.

The wordmark in the SVG lockups is outlined, so it renders without the font. Regenerate the platform icons after changing the mark:

```sh
bunx tauri icon assets/app-icon.svg   # then delete src-tauri/icons/{android,ios}
```

## Colour

Brand colours are fixed; the UI colours below are their theme-aware tokens in `src/shared/styles/global.css`.

| Name    | Value                             | Role                                                                 |
| ------- | --------------------------------- | -------------------------------------------------------------------- |
| Harbour | `oklch(0.50 0.13 240)` `#006aa5`  | Primary actions, links, focus, selection — `--primary`               |
| Beacon  | `oklch(0.73 0.17 52)` `#f8842d`   | The brand's own moments: the mark, cursors, search hits — `--beacon` |
| Tide    | `oklch(0.80 0.10 222)` `#6eccec`  | Water in the mark; dark-theme primary is its sibling                 |
| Ink     | `oklch(0.21 0.03 256)` `#0f1926`  | Text and dark surfaces; neutrals are Ink-tinted (hue 245–256)        |
| Foam    | `oklch(0.96 0.008 240)` `#edf3f7` | Text on dark                                                         |
| Tile    | `#004c76` → `#091a34`             | The mark's gradient only                                             |

Beacon is an accent, not an action colour: keep it to the mark and small signals so it stays loud. Buttons are Harbour.

Status tones are tokens too — `--success`, `--warning`, `--info`, `--destructive` — chosen to pass WCAG AA as text on the background in both themes. Use them (`bg-success`, `text-warning`, `StatusDot`) rather than Tailwind's palette, so a status reads the same everywhere. Chart series are `--chart-1` … `--chart-5`: Harbour, Beacon, sea green, violet, coral.

Libraries that can't read CSS variables (xterm.js, CodeMirror syntax) carry hex copies of the same hues: `src/shared/ui/terminal/XTerm.tsx` and `src/shared/ui/editor/editorTheme.ts`.

## Type

- **Geist** for the interface and the wordmark.
- **Geist Mono** for anything a machine wrote or will read: IDs, paths, ports, commands, logs, terminals, compose files (`font-mono`).

## Voice

Plain, specific, calm. Say what happened and what to do next; name things by the [glossary](../GLOSSARY.md). British spelling (colour, harbour).
