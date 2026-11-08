---
title: Keyboard
description: Drive Homeport without the mouse — the command palette and search across every Host, shortcuts for every page and Host, and the help sheet that lists them.
---

Homeport can be used from the keyboard. Press <kbd>Mod</kbd> <kbd>K</kbd> to open the [command palette](#command-palette), and <kbd>?</kbd> on any page to see every shortcut that works there.

**Mod** below means <kbd>⌘</kbd> on macOS and <kbd>Ctrl</kbd> on Windows and Linux; on macOS, <kbd>Alt</kbd> is <kbd>⌥</kbd>.

## Command palette

<kbd>Mod</kbd> <kbd>K</kbd> opens the command palette from any page. Type what you want, move with <kbd>↑</kbd> and <kbd>↓</kbd>, and press <kbd>Enter</kbd>; <kbd>Esc</kbd> closes it and puts you back where you were. It lists:

- **Go to** — every page in the sidebar: type `vol` and press <kbd>Enter</kbd> to open Volumes.
- **Hosts** — every Host with its status: type part of a Host's name to switch to it.
- **Actions** — **Run a container**, **Pull an image**, **New Stack**, **Clean up the selected host**, **Add host**, **Manage hosts**, **Check for updates** and **Show keyboard shortcuts**.
- **Settings** — **Theme: Light**, **Theme: Dark** and **Theme: Match the system**.

Actions that need a Host — running a container, pulling, a new Stack, cleaning up — are listed only while the selected Host is connected.

### Searching every Host

Type two characters or more and the palette also searches the **containers**, **Stacks**, **images**, **volumes** and **networks** of every connected Host — "where does `postgres` run?" has an answer without switching Host. It matches names, short IDs and, for containers, their image and Stack; names that start with what you typed come first.

- Results are grouped by kind, five per kind until **Show all**. Each shows its Host, and for containers and Stacks whether they run: _prod-server · running_.
- Choosing one switches to its Host and opens it: a container's or a Stack's page, or the Images, Volumes or Networks page searched for it.
- Hosts that aren't connected are skipped, and the palette says how many: _2 hosts not searched (not connected)_.

Homeport loads the lists only once you've typed two characters, and keeps them fresh from each Host's live events after that. Logs, files and compose definitions aren't searched. Each item shows its shortcut, if it has one. With nothing typed, the items you used last come first, under **Recent**, until Homeport quits.

## Shortcuts

### Everywhere

| Keys                                       | Does                                                                              |
| ------------------------------------------ | --------------------------------------------------------------------------------- |
| <kbd>?</kbd>                               | Show the keyboard shortcuts                                                       |
| <kbd>Mod</kbd> <kbd>1</kbd> … <kbd>9</kbd> | Open the sidebar's pages in order: Overview, Containers, Images…                  |
| <kbd>Mod</kbd> <kbd>,</kbd>                | Open Settings                                                                     |
| <kbd>Alt</kbd> <kbd>↑</kbd> / <kbd>↓</kbd> | Select the previous or next Host, in the order the Hosts page lists them          |
| <kbd>Mod</kbd> <kbd>R</kbd>                | Refresh the selected Host's lists: containers, images, volumes, networks, Stacks… |

Hover a page in the sidebar to see its keys.

### On a page

| Keys         | Does                                                                                                                            |
| ------------ | ------------------------------------------------------------------------------------------------------------------------------- |
| <kbd>/</kbd> | Focus the page's search field. <kbd>Esc</kbd> in an empty field leaves it.                                                      |
| <kbd>N</kbd> | Start the page's main action: **Run container**, **Pull**, **Create volume**, **Create network**, **New Stack** or **Add host** |

The button of a page's main action shows <kbd>N</kbd> when you hover it.

### While typing

Single keys — <kbd>N</kbd>, <kbd>/</kbd>, <kbd>?</kbd> and the <kbd>Alt</kbd> arrows — never fire while you type in a field, a Stack's editor or a dialog: they type the character instead. Shortcuts with <kbd>Mod</kbd> work there too. In a container's **Terminal**, every key goes to the container and no shortcut fires.
