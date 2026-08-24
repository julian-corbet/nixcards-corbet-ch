# nixcards

Local-first flashcards in Markdown, delivered as a mobile PWA and a Ratatui terminal application.

nixcards keeps the behavior in one Rust core and ships a curated, community-maintained catalogue
with the application. Browse any card freely, run a focused cram session, and keep your progress on
your own device. There are no accounts, analytics, or backend services.

**Use it now:** [nixcards.corbet.ch](https://nixcards.corbet.ch)

## Status

The first release includes the mobile web application, terminal interface, catalogue validator,
progress export/import, and a German BearingPoint cloud interview set.

## Repository shape

```text
cards branch              canonical Markdown catalogue and contribution target
crates/nixcards-core/     parser, validation, search, cram, progress, WASM API
crates/nixcards-store/    Git-native partial clone and sparse selection
crates/nixcards-tui/      Ratatui interface, catalogue manager, local progress store
web/                      mobile-first Svelte PWA
```

The `cards` branch is the only editable source of card content. Builds check it out temporarily as
the ignored `cards/` directory and compile the validated catalogue into the terminal and web apps.
There is no generated contribution branch and no second card tree to keep in sync.

## Card set format

Each set is a directory whose path mirrors its dotted ID. `set.md` contains only set metadata and
an optional overview:

```markdown
---
id: cloud.example.certification.associate
title: Example Cloud Associate
language: en
license: CC-BY-NC-SA-4.0
attribution: Example contributors
tags: [cloud, certification]
sources: [https://example.com/public-objectives]
---
```

On the `cards` branch that file lives at `cloud/example/certification/associate/set.md`. Every card is a separate
Markdown file below the same directory:

```markdown
# What is a landing zone?

A governed starting point for cloud workloads.
```

Saving it as `platform/landing-zone.md` gives it the stable card ID `platform.landing-zone` and the
canonical ID `cloud.example.certification.associate#platform.landing-zone`. Set IDs contain two to
five dotted segments; card IDs contain one to five.

## Development

Requirements: Rust 1.96+, Node.js 24+, npm 11+, and wasm-pack 0.15+.

```sh
just cards
just check
just web-dev
cargo run -p nixcards
```

`just cards` clones the canonical data branch into the ignored local `cards/` build input. Use
`just cards-sync` to fast-forward a clean checkout. To contribute, create a topic branch inside
`cards/`, edit the Markdown you are learning from, push that branch, and open the pull request
against the repository's `cards` branch.

## Selective local catalogue

The terminal application can keep a large catalogue local without downloading every card. Press
`m` in the TUI, select any hierarchy branches or individual sets, and press Enter to apply. The
checkout uses Git's `blob:none` partial-clone filter and Git sparse checkout; the sparse paths are
the only local selection record.

The default standalone location is `$XDG_DATA_HOME/nixcards/knowledge/cards` (or
`~/.local/share/nixcards/knowledge/cards`). Override it with `NIXCARDS_STORE` or `--store`:

```sh
nixcards --store /path/to/brain/knowledge/cards catalog init
nixcards --store /path/to/brain/knowledge/cards catalog select cloud.bearingpoint.interview
nixcards --store /path/to/brain/knowledge/cards catalog status
```

The canonical `cards` branch has the catalogue at its root. Its small `catalog.json` is always
present, while unselected Markdown blobs remain remote. The sparse checkout is therefore also a
normal Git contribution checkout: corrections do not need to be copied into another tree.

## Licence

Application code is available under FSL-1.1-ALv2 and converts to Apache-2.0 two years after each
version is published. Card content and the catalogue are licensed separately under
CC-BY-NC-SA-4.0. See [`LICENSING.md`](LICENSING.md).

Contributions are welcome, especially focused card sets with stable IDs and authoritative sources.
See [`CONTRIBUTING.md`](CONTRIBUTING.md) before opening a pull request.

nixcards is an independent project and is not affiliated with or endorsed by certification
providers or employers named in contributed study material.
