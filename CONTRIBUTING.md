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

The outbound license for a contribution depends on what it changes:

- Code contributions are licensed under FSL-1.1-ALv2 with Apache-2.0 as the Future Licence.
- Card and catalogue contributions are licensed under CC-BY-NC-SA-4.0.

Before submitting, read and agree to version 1.0 of the organization-wide
[Individual Contributor License Agreement][icla]. You retain ownership of your contribution, and
your attribution may be recorded in the set metadata and Git history. Do not contribute material
unless you have the right to grant the agreement's terms.

> I have read and agree to version 1.0 of the Individual Contributor License Agreement at https://github.com/corbet-labs/.github/blob/cla-v1.0/CLA.md.

[icla]: https://github.com/corbet-labs/.github/blob/cla-v1.0/CLA.md

## Code quality

- Keep product behavior in `nixcards-core`; UIs are adapters.
- Add behavior tests before changing a contract in `BEHAVIORS.md`.
- Do not add network access, analytics, or external assets.
- Keep browser and terminal behavior aligned.
