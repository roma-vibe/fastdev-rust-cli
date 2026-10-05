#!/bin/sh
# Quality gate: formatting, lints and tests. Warnings are errors; everything must pass before a
# commit. --locked fails when Cargo.lock is out of date with Cargo.toml.
set -eu
cd "$(dirname "$0")/.."

echo "==> cargo fmt --check"
cargo fmt --all --check

echo "==> cargo clippy"
cargo clippy --all-targets --locked -- -D warnings

echo "==> cargo test"
cargo test --locked

echo "All checks passed."
