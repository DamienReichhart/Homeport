---
title: Keyboard and accessibility
description: Using Homeport with the keyboard, reduced motion and a screen reader, the contrast it meets, the limits known today, and how to report a barrier.
---

Homeport aims to be usable with the keyboard alone, with a screen reader and with reduced motion, in every one of its [languages](/docs/guides/settings/#appearance). This page says what works, what doesn't yet, and how to tell us when something stands in your way.

## Moving around

Everything in Homeport can be reached with <kbd>Tab</kbd> and <kbd>Shift</kbd> <kbd>Tab</kbd>, and used with <kbd>Enter</kbd> or <kbd>Space</kbd>. A visible ring shows where the focus is.

- **The sidebar** lists every page; each is a link. <kbd>Mod</kbd> <kbd>1</kbd> to <kbd>9</kbd> open them in order, and the [command palette](/docs/guides/keyboard/#command-palette) (<kbd>Mod</kbd> <kbd>K</kbd>) reaches every page, Host and action by typing.
- **Lists** (containers, images, volumes, networks, Stacks, Hosts, Pods…): each row's name is a button or a link. <kbd>Tab</kbd> to it and press <kbd>Enter</kbd> to open the item, as a click on the row does. The checkboxes and the row's menu come before and after it.
- **Sorting**: a sortable column's header is a button. Press <kbd>Enter</kbd> or <kbd>Space</kbd> to sort by it, again to reverse; screen readers announce the order.
- **Stacks in the containers list** open and close with the button before their name, which says whether it is expanded.
- **Dialogs** take the focus when they open, keep it inside while open, and give it back to what opened them when they close. <kbd>Esc</kbd> closes a dialog without doing anything.
- **Menus, selects and tabs** move with the arrow keys, <kbd>Home</kbd> and <kbd>End</kbd>; <kbd>Enter</kbd> chooses, <kbd>Esc</kbd> closes.
- **Shortcuts**: <kbd>?</kbd> on any page lists the ones that work there; see [Keyboard](/docs/guides/keyboard/).

## Terminals, logs and the editor

A container's **Terminal**, and the logs viewer, take every key while they have the focus, so the program inside gets <kbd>Tab</kbd>, <kbd>Esc</kbd> and <kbd>Ctrl</kbd> keys too. Leave them with the mouse, or by moving the focus with your screen reader's own commands.

The compose, YAML, job and unit editors are CodeMirror editors named for screen readers. In them, <kbd>Tab</kbd> indents; press <kbd>Esc</kbd> and then <kbd>Tab</kbd> to move the focus out of the editor.

## Reduced motion

When your operating system asks for reduced motion (**Reduce motion** on macOS, **Animation effects** off on Windows, **Reduce animation** on GNOME and KDE), Homeport only fades: dialogs, menus, selects and tooltips don't zoom or slide, nothing pulses, buttons don't move when pressed and the terminal's cursor stops blinking. Spinners keep turning, slower, because they say that work is in progress. The setting is followed as it changes, without a restart.

## Screen readers

Every page and the main dialogs are checked with [axe](https://github.com/dequelabs/axe-core) on each change: buttons, fields and regions have names, headings and landmarks are in order, and nothing misuses ARIA. Homeport sets the page's language, so a screen reader reads each language with its own voice, and Arabic is shown right to left.

Homeport's window is a web view: WebKitGTK on Linux, WKWebView on macOS and WebView2 on Windows. A hands-on audit with Orca, VoiceOver and NVDA on each is under way ([#184](https://github.com/DamienReichhart/Homeport/issues/184)); until it's done, treat the screen-reader experience as tested by tools only.

### Known limits

- **Terminals and logs**: the text inside a container's terminal and the logs viewer isn't read by screen readers yet. Copy the output, or open the logs in a terminal with `docker logs`.
- **Live lists**: lists refresh in place as the Engine reports changes, without announcing each change. Toasts announce what you did.
- **Charts** draw their values; the latest value of each is also in the chart's title, as text.

## Contrast

Text meets WCAG 2.2 AA (4.5:1) in both the light and the dark theme, and so do the borders of fields, the focus ring and the status shapes (3:1). Status never relies on colour alone: each has its own shape. The ratios are listed in the project's brand guide.

## Reporting a barrier

If something stops you from using Homeport with your keyboard, screen reader, magnifier or other assistive technology, [open an issue](https://github.com/DamienReichhart/Homeport/issues/new?labels=accessibility) with the `accessibility` label. Say:

- what you were trying to do, and where (page, dialog, Host kind);
- your operating system and its version, and the assistive technology and its version;
- what you heard or saw, and what you expected.

Barriers are treated as bugs, not feature requests.
