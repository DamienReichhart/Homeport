# Product

<!-- impeccable:product-schema 1 -->

## Platform

web

A Tauri 2 desktop app: the interface is a web view, laid out for desktop windows from 800 × 520 (the minimum) to full screen. There is no phone or tablet layout.

## Users

A self-hosting developer or a small team managing 2 to 10 Docker Hosts, most of them over SSH, sometimes with one Kubernetes or Nomad Cluster beside them. They switch between Hosts several times a day, usually to answer one question (is it up, why did it die, what changed) or to do one action (redeploy, restart, read logs, open a shell), and then leave.

## Product Purpose

One desktop app for every container Engine and orchestrator the user reaches: Docker Engines and Podman, containerd, Incus, the Swarms those Hosts form, and Kubernetes and Nomad Clusters. Success is the user finding what needs them and acting on it without opening a terminal or SSH session.

## Positioning

Homeport stays on the user's machine and visits their servers: nothing is installed on a Host or a Cluster, there is no agent, no web panel, no account and no telemetry. Every Engine kind and orchestrator is reached from the same app, with the same vocabulary.

## Operating Context

Used alongside a terminal and an editor, often at odd hours when something broke. Keyboard-driven use matters: a command palette (Mod+K), shortcuts for every page and Host, listed with `?`. Compose files, kubeconfigs, `~/.ssh/config` and Docker contexts are the materials it imports from.

## Capabilities and Constraints

- Full capability list in `README.md` § Features; vocabulary in `GLOSSARY.md`; main decisions in `docs/adr/`.
- 12 languages, including Arabic (right to left); every string lives in the catalogues under `src/shared/i18n/` and every language must carry every key.
- WCAG 2.2 AA in both themes, checked by `src/shared/styles/contrast.test.ts` and axe on every page.
- Version 1.0.1 is released and in use: keyboard shortcuts and routes that still make sense are kept, and glossary terms only change with the owner's approval.

## Brand Commitments

`docs/BRAND.md` is binding: Ink, Paper and one Signal red that only means "this needs you"; healthy gets no loud colour; flat surfaces, hairline borders, 4 px radius, no gradients, glows or depth shadows; Host Grotesk for the interface and Fragment Mono for machine text; the mark and lockup as drawn. Voice: plain, specific, calm, British spelling, no exclamation marks, named by the glossary. Status marks may be made easier to tell apart at a glance, including the muted green for `--success` that `BRAND.md` already allows.

## Evidence on Hand

Real screenshots of the app with demo data: `website/src/assets/screenshots/`, produced by `bun run screenshots` from `scripts/screenshots/`. No customer quotes, usage numbers or logos exist; none may be invented.

## Product Principles

1. What needs you comes first; what is healthy stays quiet.
2. One question, one place: the answer to "is it up / why did it die / what changed" is never more than one step away.
3. Nothing surprising happens to a server: every destructive or deploying action says what it will do, to which Host, before it does it.
4. Machine text looks like machine text; the interface speaks the glossary.
5. The keyboard reaches everything the mouse does.

## Accessibility & Inclusion

WCAG 2.2 AA in light and dark; every status is a shape as well as a colour; reduced motion is respected; full right-to-left layout for Arabic.
