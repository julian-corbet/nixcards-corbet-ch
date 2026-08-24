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
cards/                    bundled Markdown catalogue
crates/nixcards-core/     parser, validation, search, cram, progress, WASM API
crates/nixcards-tui/      Ratatui interface and local progress store
web/                      mobile-first Svelte PWA
```

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

That file lives at `cards/cloud/example/certification/associate/set.md`. Every card is a separate
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
just check
just web-dev
cargo run -p nixcards
```

## Licence

Application code is available under FSL-1.1-ALv2 and converts to Apache-2.0 two years after each
version is published. Card content and the catalogue are licensed separately under
CC-BY-NC-SA-4.0. See [`LICENSING.md`](LICENSING.md).

Contributions are welcome, especially focused card sets with stable IDs and authoritative sources.
See [`CONTRIBUTING.md`](CONTRIBUTING.md) before opening a pull request.

nixcards is an independent project and is not affiliated with or endorsed by certification
providers or employers named in contributed study material.
