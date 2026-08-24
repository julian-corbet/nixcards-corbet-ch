# Decision queue

nixcards is currently a local-first cram application with one Rust behavior core, a mobile PWA, a
Ratatui interface, browser-local progress, and a canonical Git-native card catalogue. This file is
not a promise of dates or scope. It separates contributions that can proceed now from product
changes that need an explicit decision.

## Open without another product decision

- Correct existing cards or add original card sets, including certification sets, through pull
  requests to the `cards` branch.
- Improve accessibility, mobile ergonomics, validation, documentation, tests, and performance
  without changing the promises in `BEHAVIORS.md`.
- Add cfetch-compatible card sets: selected Markdown already occupies `knowledge/cards` and is
  indexed under the brain's normal path-based trust rules.

## Waiting for an explicit go

1. **A long-term scheduler.** Replay the existing review-event history into spaced repetition or
   another retention model beyond the current cram loop.
2. **Web catalogue scaling.** Replace the all-cards application bundle with validated per-set static
   artifacts, explicit browser selection, and offline caching when catalogue size makes bundling
   everything unreasonable.
3. **Local AI assistance.** Generate draft cards, audit existing material against public sources,
   and propose corrections locally without making an AI service part of the core data path.
4. **Deeper human-agent learning semantics.** Model curricula, coverage, and a virtual
   “certificate passed” state; derive graph links or dedicated agent skills from selected cards.
   Today cfetch intentionally treats the Markdown as ordinary knowledge and does not infer mastery.
5. **Progress sync or accounts.** Any cross-device state, identity, collaboration, or server API.
   Export/import remains the only progress transfer today.
6. **In-app authoring beyond GitHub.** A native editor, review queue, or guided set builder. The web
   app currently links each card to its exact canonical Markdown file instead.

Each item should start with a behavior and privacy decision before implementation. The repository
shape, Markdown IDs, Rust core, and versioned review events are designed to support these additions
without committing to them prematurely.
