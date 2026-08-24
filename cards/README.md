# Card catalogue

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
before card blobs are downloaded. Regenerate it with `nixcards catalog-index`; CI rejects drift.
