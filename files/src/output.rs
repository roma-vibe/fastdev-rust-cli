//! Writing command results to stdout: output formats and helpers shared by all commands.
//!
//! Results go to stdout; logs and errors go to stderr (`tracing`, `main`). Commands write through
//! a `&mut impl Write` so they can be tested with a `Vec<u8>`.

use std::io::{self, BufWriter, Write};

use clap::ValueEnum;
use serde::Serialize;

/// How a command prints its result (`--format`).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, ValueEnum)]
pub enum Format {
    /// Aligned, human-readable text
    #[default]
    Text,
    /// Pretty-printed JSON for scripts and other tools
    Json,
}

/// Buffered, locked stdout. Call `flush` before returning so write errors are reported.
pub fn stdout() -> impl Write {
    BufWriter::new(io::stdout().lock())
}

/// Writes `value` as pretty-printed JSON followed by a newline.
pub fn write_json(out: &mut impl Write, value: &impl Serialize) -> io::Result<()> {
    serde_json::to_writer_pretty(&mut *out, value).map_err(io::Error::from)?;
    writeln!(out)
}

/// True when the error was caused by a closed stdout (e.g. `my-tool stats | head -n 1`), which
/// is not a failure of the tool.
pub fn is_broken_pipe(error: &anyhow::Error) -> bool {
    error
        .chain()
        .filter_map(|cause| cause.downcast_ref::<io::Error>())
        .any(|cause| cause.kind() == io::ErrorKind::BrokenPipe)
}

#[cfg(test)]
mod tests {
    use anyhow::Context;

    use super::*;

    #[test]
    fn writes_json_with_trailing_newline() {
        let mut out = Vec::new();
        write_json(&mut out, &serde_json::json!({ "lines": 2 })).unwrap();
        assert_eq!(String::from_utf8(out).unwrap(), "{\n  \"lines\": 2\n}\n");
    }

    #[test]
    fn detects_broken_pipe_in_error_chain() {
        let broken: anyhow::Result<()> = Err(io::Error::from(io::ErrorKind::BrokenPipe).into());
        assert!(is_broken_pipe(&broken.context("writing results").unwrap_err()));

        let other = anyhow::anyhow!("something else");
        assert!(!is_broken_pipe(&other));
    }
}
