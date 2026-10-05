//! Compiles the display name into the binary: `env!("APP_NAME")` in `src/config.rs`.
//!
//! The value comes from `APP_NAME` in `.env` (the single source of truth), then from the
//! `APP_NAME` environment variable (Docker and CI builds, where `.env` is not available), then from
//! `.env.example`. Cargo re-runs this script when any of them changes.

use std::path::Path;

const KEY: &str = "APP_NAME";

fn main() {
    println!("cargo::rerun-if-changed=.env");
    println!("cargo::rerun-if-changed=.env.example");
    println!("cargo::rerun-if-env-changed={KEY}");

    let name = read_env_file(Path::new(".env"))
        .or_else(|| std::env::var(KEY).ok().filter(|value| !value.trim().is_empty()))
        .or_else(|| read_env_file(Path::new(".env.example")));
    match name {
        Some(name) => println!("cargo::rustc-env={KEY}={name}"),
        None => {
            println!("cargo::error={KEY} is not set: copy .env.example to .env and set {KEY} there");
        }
    }
}

/// `APP_NAME` from a dotenv file; `None` when the file or the key is missing or the value is blank.
fn read_env_file(path: &Path) -> Option<String> {
    let entries = dotenvy::from_path_iter(path).ok()?;
    entries
        .filter_map(Result::ok)
        .find(|(key, _)| key == KEY)
        .map(|(_, value)| value.trim().to_string())
        .filter(|value| !value.is_empty())
}
