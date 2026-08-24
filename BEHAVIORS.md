# nixcards behaviors

These are product promises. Tests should name the behavior they prove.

## Catalogue

1. Every release contains a curated catalogue of Markdown card sets from `cards/`.
2. A set ID is a stable dotted hierarchy with two to five lowercase ASCII segments.
3. The set directory mirrors the ID and contains a `set.md` manifest plus one Markdown file per
   card.
4. A card's relative path mirrors its stable dotted ID; the canonical card ID is
   `<set-id>#<card-id>`.
5. A build fails on malformed metadata, duplicate IDs, empty questions or answers, path drift, raw
   HTML, or card sets without provenance.

## Learning

6. A user can browse every set and card freely without changing progress.
7. Search spans questions, answers, set titles, IDs, and tags.
8. Cram mode visits every card in a chosen set and repeats missed cards until they are known.
9. Reviews are stored as versioned events, not destructive counters, so future schedulers can
   replay the history.
10. Terminal and web surfaces use the same Rust behavior.

## Local-first operation

11. The web application works offline after its first successful load.
12. Browser progress stays in IndexedDB and can be exported and imported as versioned JSON.
13. Terminal progress stays in the platform state directory and uses the same JSON schema.
14. The application performs no analytics, tracking, authentication, or background network calls.

## Interfaces

15. The web interface is designed for one-handed use on a narrow phone before desktop layouts.
16. The Ratatui interface supports browsing, search, reveal, cram review, validation, and progress
    import/export.
17. Reduced-motion and keyboard users retain the complete product behavior.

## Content integrity

18. Certification material is original and based only on public objectives or documentation.
19. Exam dumps, NDA material, copied exam questions, vendor logos, and affiliation claims are
    rejected.
20. A card set records its language, licence, tags, attribution, and public sources.
