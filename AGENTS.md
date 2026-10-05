# Rust CLI skeleton — instructions for AI agents

This repository is a fastDev skeleton (`rust-cli`). It is not a project: fastDev copies `files/` into
new projects and renders `*.tmpl` files. Read the fastDev skeleton authoring guide first
(MCP tool `get_authoring_guide`, or `docs/skeleton-authoring.md` in the fastDev repository).

- Change `template.toml` and `files/` only; `files/AGENTS.md.tmpl` and `files/SPEC.md.tmpl` are the
  instructions of the future projects, not of this repository.
- Do not commit, tag or edit `CHANGELOG.md` by hand: validate, verify and publish through fastDev
  (`validate_skeleton`, `verify_skeleton`, `publish_skeleton`). Publishing commits, creates the
  `vX.Y.Z` tag, pushes and updates the registry.
- Never move or delete a published tag.
- Everything is written in English.

## Working on the Rust sources

- Never build inside `files/` (a `files/target/` folder is rejected). Copy `files/` to a scratch
  folder, add `.env` (`cp .env.example .env`), work and test there (`sh scripts/check.sh`,
  `cargo build --release`, `sh scripts/dev.sh`), then copy the changed files back.
- `files/Cargo.toml` keeps the neutral package name `app` (so does the root entry of
  `files/Cargo.lock`). fastDev renames both to `APP_SLUG` with `[[set]]` (`package.name` and
  `package[name=app].name`), so `--locked` works at once; the setup step `cargo fetch` then moves the
  renamed entry to its sorted place. Before copying `Cargo.toml`/`Cargo.lock` back, make sure the
  package name is `app` again and `files/Cargo.lock` lists it in sorted order.
- Dependency updates are manual (fastDev has no cargo adapter yet): in the scratch copy run
  `cargo update` (minor/patch) or edit versions in `Cargo.toml` for majors, check the release notes,
  keep `rust-version` supported (the resolver picks compatible versions; test the MSRV with
  `docker run --rm -v "$PWD:/src:ro" rust:<msrv>-slim sh -c 'cp -r /src /w && cd /w && cargo test --locked'`),
  then update `stack` in `template.toml` and the docs.
- The Docker images use `rust:1.98-slim-trixie` (build and dev stages of `files/Dockerfile.tmpl`) and
  `gcr.io/distroless/cc-debian13:nonroot`; update the tags together, together with the Rust version
  mentioned in `files/AGENTS.md.tmpl` and `README.md`.
- `[verify] commands` names `docker-build`, which only the Docker options add: the `run` and `full`
  variants build the production image, `none` skips it. `full` removes `build`, `start` and
  `install` (there is no macOS binary) and previews with `docker-run`.
