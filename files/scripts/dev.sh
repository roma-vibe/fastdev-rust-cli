#!/bin/sh
# Development loop: builds and runs the tool, then again after every change to src/, build.rs,
# Cargo.toml, Cargo.lock or .env. The tool gets this script's arguments, or DEV_ARGS from .env
# (read again before every run) when there are none. Stop it with Ctrl+C.
set -u
cd "$(dirname "$0")/.." || exit 1

# DEV_ARGS from .env, read by the shell so quoting works: DEV_ARGS="stats 'my notes.txt'".
read_dev_args() {
  [ -f .env ] || return 0
  (
    set +u -a
    . ./.env >/dev/null 2>&1
    printf '%s' "${DEV_ARGS:-}"
  )
}

stamp=target/.dev-stamp
mkdir -p target
explicit_args=$#

run() {
  touch "$stamp"
  if [ "$explicit_args" -eq 0 ]; then
    eval "set -- $(read_dev_args)"
  fi
  echo "[dev] cargo run -- $*"
  cargo run --quiet -- "$@"
  echo "[dev] exit code $?. Watching for changes (Ctrl+C to stop)..."
}

changed() {
  [ -n "$(find src build.rs Cargo.toml Cargo.lock .env -newer "$stamp" 2>/dev/null | head -n 1)" ]
}

run "$@"
while sleep 1; do
  if changed; then
    echo
    run "$@"
  fi
done
