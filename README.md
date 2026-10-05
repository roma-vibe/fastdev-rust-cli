# Rust CLI

A [fastDev](https://github.com/roma-vibe/fastDev) project skeleton (`rust-cli`) for **fast command-line utilities in Rust that ship as one native executable**. fastDev lists it from a registry, downloads it when first used and creates named, ready-to-run projects from it.

Choose it for a tool that people or scripts run in a terminal: text and file processing, developer tooling, converters, automation helpers. It is not meant for servers, GUIs or libraries.

## Stack

| Part | Choice |
| --- | --- |
| Language | Rust 1.85+ (edition 2024), MSRV in `Cargo.toml`; the owner's machine and Docker use 1.98 |
| Arguments and help | clap 4 (derive), global `-v`/`-q`, `--version`, value enums |
| Errors | anyhow with context; exit codes 0 / 1 / 2 (usage) |
| Logging | tracing + tracing-subscriber on stderr (`-v`…`-vvv`, `RUST_LOG`) |
| Output | text tables or `--format json` (serde, serde_json); buffered stdout, broken pipes are not errors |
| Tests | built-in harness; assert_cmd 2, predicates 3, tempfile 3 for black-box tests of the binary |
| Quality gate | rustfmt (width 120), clippy `pedantic` with `-D warnings`, `unsafe` forbidden |
| Build | `build.rs` (dotenvy) compiles `APP_NAME` from `.env`; release profile with LTO, 1 codegen unit, stripped |
| Docker | build on `rust:1.98-slim-trixie`, runtime `gcr.io/distroless/cc-debian13:nonroot` (≈ 55 MB image) |

## What a new project gets

- A working binary named after the project (`package.name` = `APP_SLUG`; slugs that Cargo rejects, e.g. starting with a digit, get an `app-` prefix) whose `--help` shows `APP_NAME` from `.env`.
- The example command `stats` (lines, words, characters and bytes of files or standard input, text or JSON) through every layer: `cli.rs` → `commands/stats.rs` → `domain/stats.rs` → `output.rs`, with 13 unit tests and 8 black-box tests.
- `scripts/dev.sh`: a watch-and-run loop without extra tools; it rebuilds and runs the tool with `DEV_ARGS` from `.env` after every change to `src/`, `build.rs`, `Cargo.toml`, `Cargo.lock` or `.env`.
- `scripts/check.sh`: `cargo fmt --check`, `cargo clippy --all-targets --locked -- -D warnings`, `cargo test --locked`.
- `Cargo.lock` with the exact dependency versions, `.env.example`, `.gitignore`, `.editorconfig`, VS Code recommendations.
- `AGENTS.md` (stack, project map, architecture rules, commands per Docker mode, how to add commands/options/dependencies, testing, definition of done), `SPEC.md` (CLI-oriented specification to complete with the owner) and `README.md`.

## Options

### Docker (`docker`, default `run`)

| Option | What runs where |
| --- | --- |
| `none` — No Docker | Everything on the Mac with Rust. No Docker files. |
| `run` — Docker to run the app (default) | Development, tests, the native binary and **Install** on the Mac with Rust. `Dockerfile` + `docker-compose.yml` build a small Linux image of the tool (named after `COMPOSE_PROJECT_NAME`) and run it on the project folder (`docker compose run --rm --build app <args>`, mounted at `/work`). Commands **Docker build**, **Docker run**, **Docker clean**. |
| `full` — Everything in Docker | Only Docker on the Mac. A `dev` container (Rust 1.98 with rustfmt/clippy, user `dev`, `target/` and the crate cache in named volumes) runs the dev loop (`docker compose up dev`), tests and the quality gate. There is no macOS binary: **Docker build** builds the production image and the tool is used through it. |

There is no data-storage choice: the tool keeps no state and works on files in the folder it runs in.

## Commands of a new project

| Key | `none` / `run` | `full` |
| --- | --- | --- |
| `dev` (primary, long) | `sh scripts/dev.sh` | `docker compose up dev` |
| `build` | `cargo build --release` | — |
| `start` | `cargo run --release --quiet -- --help` | — |
| `install` | `cargo install --path . --locked` | — |
| `test` | `cargo test` | `docker compose run --rm dev cargo test` |
| `check` | `sh scripts/check.sh` | `docker compose run --rm dev sh scripts/check.sh` |
| `lint`, `format` | `cargo clippy …`, `cargo fmt --all` | the same in the dev container |
| `docker-build`, `docker-run`, `docker-clean` | `run` only | the same, plus `docker-down` |

Requirements: Rust (cargo) 1.85+ and the Xcode Command Line Tools (the linker); Docker 24+ for `run` and `full` (`full` needs only Docker).

`[[set]]` renames the package `app` in `Cargo.toml` and `Cargo.lock`, so `--locked` works even without setup. Setup: `cargo fetch` (or `docker compose run --rm dev cargo fetch`) downloads the crates and writes `Cargo.lock` in Cargo's order, so the first commit has the lockfile later cargo commands keep; dependency versions stay as locked.

## Preview

fastDev's preview runs **Run release** (`cargo run --release --quiet -- --help`) in a throwaway copy and shows `target/release`. The message tells you to `cd` into the preview folder and try `./target/release/<name> stats README.md Cargo.toml`, `--format json` and piping text into `stats`. With “Everything in Docker” it runs **Docker run** (the production image on the example command) instead.

## Verification

`[verify]` runs `check`, `build` and `docker-build` in a project with the default choices (`docker = run`: check and release build on the Mac, then the production image), with `docker = full` (check inside the dev container, then the production image; `build` does not exist there) and with `docker = none` (no Docker build). The options' `cleanup` removes the containers, volumes and images afterwards, and again when a preview is deleted.

## Repository layout

- `template.toml` — the manifest: requirements, choices, env, setup steps, commands, verify and preview.
- `files/` — the files of a new project (`*.tmpl` files are rendered with the project name and options).
- `CHANGELOG.md` — what changed in every version.

Versions are the `vX.Y.Z` tags of this repository and never change once published.
