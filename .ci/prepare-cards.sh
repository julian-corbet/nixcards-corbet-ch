#!/usr/bin/env bash
# SPDX-License-Identifier: FSL-1.1-ALv2
set -euo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")/.."

cards_revision=$(python3 -c 'import pathlib,tomllib; print(tomllib.loads(pathlib.Path(".ci/archives.toml").read_text())["archives"]["cards"]["revision"])')
[[ $cards_revision =~ ^[0-9a-f]{40}$ ]] || {
  printf 'CI cards source requires an exact pinned Git revision.\n' >&2
  exit 2
}
if [[ -e cards || -L cards ]]; then
  printf 'CI cards destination already exists; refusing to replace it.\n' >&2
  exit 2
fi
if [[ -n ${CARDS_SOURCE_ARCHIVE:-}${CARDS_SOURCE_SHA256:-} ]]; then
  : "${CARDS_SOURCE_ARCHIVE:?A staged cards archive is required with its digest}"
  : "${CARDS_SOURCE_SHA256:?A staged cards digest is required with its archive}"
  : "${CI_TOOL_BINARY:?Use the verified shared runner to extract staged cards}"
  mkdir cards
  "$CI_TOOL_BINARY" verify-source --archive "$CARDS_SOURCE_ARCHIVE" \
    --sha256 "$CARDS_SOURCE_SHA256" --commit "$cards_revision" --destination cards
else
  git init -q cards
  git -C cards fetch --depth=1 \
    https://github.com/julian-corbet/nixcards-corbet-ch.git "$cards_revision"
  git -C cards checkout --detach "$cards_revision"
fi
test -f cards/catalog.json
printf 'CI cards source verified at %s\n' "$cards_revision"
