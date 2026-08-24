# nixcards agent instructions

Read `BEHAVIORS.md` before changing product behavior.

## Premise

nixcards is a local-first flashcard application. The curated card catalogue is part of the
repository and ships with every release. The application has no account, backend, or cloud sync.
Personal progress stays on the user's device and can be exported and imported.

## Architecture

- `crates/nixcards-core`: the only owner of parsing, validation, search, card rendering, cram
  ordering, and progress reduction. It must compile natively and to WebAssembly.
- `crates/nixcards-tui`: Ratatui adapter. It may own terminal interaction and filesystem storage,
  but no domain behavior.
- `web`: mobile-first Svelte adapter. It may own browser interaction and IndexedDB storage, but no
  domain behavior.
- `cards`: bundled community content. Directory hierarchy mirrors each set's dotted ID; `set.md`
  owns set metadata and every card has its own Markdown file.

Do not add a second parser or scheduler in TypeScript. Do not add compatibility shims during active
development. Keep AI, accounts, sync, server APIs, and in-app authoring out until product behavior
explicitly changes.

## Verification

Run `just check` for Rust, WebAssembly, Svelte, and card validation. Browser changes also require a
real mobile-sized browser smoke test. GitHub Actions is the public CI authority.

## Licensing

Code is FSL-1.1-ALv2. Card content and the catalogue are CC-BY-NC-SA-4.0. New contributions must
retain the boundary and satisfy `CONTRIBUTING.md`.
