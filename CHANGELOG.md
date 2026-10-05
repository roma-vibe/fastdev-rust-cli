# Changelog

## 1.0.0 — 2026-10-05

- Initial version: single-binary Rust command-line tool (edition 2024, MSRV 1.85) with clap 4.6 (derive), anyhow 1.0, tracing 0.1 + tracing-subscriber 0.3 and serde/serde_json 1.
- Example command stats through every layer (cli.rs → commands/stats.rs → domain/stats.rs → output.rs): lines, words, characters and bytes of files or standard input, as a text table or with --format json.
- APP_NAME from .env is compiled into the binary by build.rs (dotenvy) and shown in --help; the executable is named after APP_SLUG, with an app- prefix for slugs Cargo rejects (leading digit, build, deps, examples, incremental).
- Cargo.toml and Cargo.lock are renamed by [[set]], so --locked works even without setup; the setup step cargo fetch downloads the crates and keeps Cargo.lock in Cargo's order.
- CLI conventions: results on stdout, logs and errors on stderr; -v/-vv/-vvv/-q and RUST_LOG; exit codes 0 / 1 / 2 (usage); a closed stdout (| head) is not an error; no input on an interactive terminal is refused.
- Tests: 13 unit tests and 8 black-box tests of the binary with assert_cmd 2.2, predicates 3.1 and tempfile 3.27.
- Quality gate scripts/check.sh: rustfmt (width 120), clippy pedantic with -D warnings on all targets, all tests, --locked; unsafe code is forbidden.
- Development loop scripts/dev.sh without extra tools: rebuilds and reruns the tool with DEV_ARGS from .env after every change to src/, build.rs, Cargo.toml, Cargo.lock or .env.
- Release profile with LTO, one codegen unit and stripped symbols (about 1.2 MB on macOS arm64).
- Requirements: Rust 1.85+ and the Xcode Command Line Tools (xcode-clt); Everything in Docker needs only Docker.
- Docker choice with the standard labels: No Docker, Docker to run the app (default) and Everything in Docker.
- Docker to run the app: distroless non-root image of the tool, named after COMPOSE_PROJECT_NAME and run on the project folder with docker compose run; commands Docker build, Docker run and Docker clean.
- Everything in Docker: dev container with rustfmt and clippy, target/ and the crate cache in volumes; there is no macOS binary, so Build, Run release and Install give way to Docker build and Docker run.
- Install (cargo install --path . --locked) in No Docker and Docker to run the app.
- No data-storage choice: the tool keeps no state and works on files in the folder it runs in.
- The Docker options' cleanup removes their containers, volumes and images after a verification and when a preview is deleted.
- Preview runs the release build with --help and explains how to try the binary; with Everything in Docker it builds the production image and runs the example command.
- Russian translations of all manifest texts.
- Verification runs check, build and docker-build (where the option has it) with docker=run, full and none.
- Checked on macOS arm64 with Rust 1.98.1 and Docker 29.7: verify, preview in run and full mode with cleanup on delete, the dev loop in the container, and a project named Test "Shop" Магазин (check, --help, Docker image, locked install).
- MSRV 1.85 checked in a rust:1.85 container.
