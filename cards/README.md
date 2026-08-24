# Card catalogue

Every leaf directory contains one `set.md`. The directory path mirrors the set's dotted ID.

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

Each level-two heading starts a card and ends with its stable local ID:

```markdown
## Question text {#stable-card-id}

Answer text in Markdown.
```

Use two to five set-ID segments. Do not encode language, author, difficulty, or version in the ID.
Within metadata lists, separate entries with commas and do not use nested YAML structures.

