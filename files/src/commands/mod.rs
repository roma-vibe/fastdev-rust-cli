//! Command handlers: one module per command, dispatched by [`run`].
//!
//! A handler turns its arguments into calls to `crate::domain`, handles files and standard input,
//! and renders the result to `out`. It returns errors instead of printing them or exiting.

pub mod stats;

use std::io::Write;

use anyhow::{Context, Result};

use crate::cli::Command;
use crate::output;

/// Runs `command` and writes its result to stdout.
pub fn run(command: &Command) -> Result<()> {
    let mut out = output::stdout();
    match command {
        Command::Stats(args) => stats::run(args, &mut out)?,
    }
    out.flush().context("cannot write the result")
}
