//! Application metadata, fixed at build time.
//!
//! The binary does not read `.env` at run time: `build.rs` compiles `APP_NAME` from `.env` into
//! the binary, and everything else comes from `Cargo.toml`. Run-time behaviour is controlled by
//! command-line options and a few standard environment variables (`RUST_LOG`, `NO_COLOR`).

/// Display name of the tool (`APP_NAME` in `.env`).
pub const APP_NAME: &str = env!("APP_NAME");

/// Name of the executable (`package.name` in `Cargo.toml`).
pub const BIN_NAME: &str = env!("CARGO_PKG_NAME");

/// First line of `--help`: the display name and the package description.
pub const ABOUT: &str = concat!(env!("APP_NAME"), " — ", env!("CARGO_PKG_DESCRIPTION"));
