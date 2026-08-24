# Contributing

Contributions to application code and original card sets are welcome.

## Add a card set

1. Choose a stable ID with two to five dotted segments. Each segment must be lowercase ASCII and
   may contain internal hyphens.
2. Create `cards/<segment>/<segment>/.../set.md` so the path mirrors the ID.
3. Copy the metadata and card syntax from `cards/README.md`.
4. Run `cargo run -p nixcards -- validate` and `just check`.
5. Open one focused pull request for the set.

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

