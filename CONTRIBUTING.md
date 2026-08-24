# Contributing

Contributions to application code and original card sets are welcome.

## Add a card set

1. Run `just cards`, then create a topic branch in the nested checkout with
   `git -C cards switch -c cards/<short-name>`.
2. Choose a stable ID with two to five dotted segments. Each segment must be lowercase ASCII and
   may contain internal hyphens.
3. Create `cards/<segment>/<segment>/.../set.md` so the set path mirrors the ID.
4. Add every card as its own Markdown file below that directory; its relative path is its ID.
5. Copy the metadata and card syntax from `cards/README.md`.
6. Regenerate the index with
   `cargo run -p nixcards -- catalog-index > cards/catalog.json`, then run `just check`.
7. Push the topic branch from `cards/` and open one focused pull request with `cards` as its base.

For a correction to an existing card, use the **Edit this card** link in the web app or edit the
file directly in a selected local catalogue checkout. That checkout follows the canonical `cards`
branch, so the change itself can be committed and proposed without translation. `catalog sync`
refuses to overwrite a dirty checkout.

Card text must be original. Public documentation and published certification objectives may inform
the material, but real exam questions, dumps, NDA material, vendor logos, and copied proprietary
explanations are not accepted. Name vendors only to identify the subject; do not imply affiliation
or endorsement.

## Contribution terms

By submitting a contribution, you certify that you have the right to submit it and agree to the
following inbound terms:

- Code contributions are licensed under FSL-1.1-ALv2 with Apache-2.0 as the Future Licence.
- Card and catalogue contributions are licensed under CC-BY-NC-SA-4.0.
- You grant the project maintainer a perpetual, worldwide, non-exclusive, royalty-free right to
  use, reproduce, modify, distribute, sublicense, and relicense the contribution, including under
  commercial terms. You retain ownership of your contribution.
- Your attribution may be recorded in the set metadata and Git history.

This grant preserves the public licence while allowing the project to offer separately licensed
services later. Do not contribute material you cannot license on these terms.

## Code quality

- Keep product behavior in `nixcards-core`; UIs are adapters.
- Add behavior tests before changing a contract in `BEHAVIORS.md`.
- Do not add network access, analytics, or external assets.
- Keep browser and terminal behavior aligned.
