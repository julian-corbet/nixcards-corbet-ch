# Card catalogue

This branch is the canonical, directly editable source for every card shipped by nixcards. The web
app's **Edit this card** link and the terminal's sparse checkout both point here. Corrections and new
sets are proposed as pull requests whose base branch is `cards`; there is no generated catalogue
branch and no second content tree.

Every leaf directory contains one `set.md` manifest and one Markdown file per card. The set
directory path mirrors the set's dotted ID.

Required metadata:

```yaml
---
id: domain.subject.context
title: Human-readable title
language: en
license: CC-BY-NC-SA-4.0
attribution: Original authors or project contributors
tags: [one, two]
sources: [https://example.com/public-source]
---
```

Each card file starts with one level-one question followed by its answer:

```markdown
# Question text

Answer text in Markdown.
```

The card's relative path below the set is its dotted ID: `platform/landing-zone.md` becomes
`platform.landing-zone`. Use two to five set-ID segments and one to five card-ID segments. Do not
encode language, author, difficulty, or version in the set ID. Within metadata lists, separate
entries with commas and do not use nested YAML structures.

`catalog.json` is the committed, content-free index used to render the complete selection tree
before card blobs are downloaded. From a checkout of the application's `main` branch, run
`cargo run -p nixcards -- catalog-index > cards/catalog.json` after changing a set or card count;
CI rejects drift.

Card text must be original. Public documentation and published certification objectives may inform
it, but exam dumps, NDA material, copied proprietary explanations, vendor logos, and affiliation
claims are rejected. Contributions use CC-BY-NC-SA-4.0 plus the inbound grant described in the
project's [contribution terms](https://github.com/julian-corbet/nixcards-corbet-ch/blob/main/CONTRIBUTING.md).

For the complete local workflow, clone `main`, run `just cards`, create a topic branch inside the
resulting `cards/` checkout, and run `just check` before pushing. A selected standalone catalogue at
`knowledge/cards` is already a normal checkout of this branch and can be used directly.
