#!/usr/bin/env bash
set -euo pipefail

repository="${NIXCARDS_CARDS_REPOSITORY:-https://github.com/julian-corbet/nixcards-corbet-ch.git}"
workspace=$(git rev-parse --show-toplevel)
checkout="$workspace/cards"

if [[ ! -e "$checkout/.git" ]]; then
  if [[ -e "$checkout" ]]; then
    echo "$checkout exists but is not a Git checkout" >&2
    exit 1
  fi
  git clone --filter=blob:none --single-branch --branch cards "$repository" "$checkout"
fi

if [[ ! -f "$checkout/catalog.json" ]]; then
  echo "$checkout is not a nixcards catalogue checkout" >&2
  exit 1
fi

if [[ "${1:-}" == "--sync" ]]; then
  if [[ -n "$(git -C "$checkout" status --porcelain)" ]]; then
    echo "$checkout has local changes; commit them on a topic branch or restore them before syncing" >&2
    exit 1
  fi
  branch=$(git -C "$checkout" branch --show-current)
  if [[ "$branch" != "cards" ]]; then
    echo "$checkout is on topic branch $branch; switch to cards before syncing" >&2
    exit 1
  fi
  git -C "$checkout" fetch --filter=blob:none origin cards
  git -C "$checkout" merge --ff-only FETCH_HEAD
fi

echo "cards: $checkout ($(git -C "$checkout" rev-parse --short HEAD))"
