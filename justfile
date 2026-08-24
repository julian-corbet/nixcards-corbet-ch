set shell := ["bash", "-euo", "pipefail", "-c"]

default:
    @just --list

cards:
    ./scripts/checkout-cards.sh

cards-sync:
    ./scripts/checkout-cards.sh --sync

fmt:
    cargo fmt --all --check

rust-check:
    cargo clippy --workspace --all-targets --all-features -- -D warnings
    cargo test --workspace --all-features
    cargo run -p nixcards -- validate

wasm:
    wasm-pack build crates/nixcards-core --target web --out-dir ../../web/src/lib/wasm --features wasm

web-install:
    cd web && npm ci

web-check: wasm
    cd web && npm run check
    cd web && npm run build

check: cards fmt rust-check web-check

web-dev: wasm
    cd web && npm run dev -- --host 0.0.0.0

deploy: check
    cd web && npx wrangler deploy --config ../wrangler.jsonc
