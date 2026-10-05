//! Diagnostics with `tracing`, written to stderr so stdout stays clean for the command's output.

use std::io::IsTerminal;

use tracing_subscriber::EnvFilter;
use tracing_subscriber::filter::LevelFilter;

/// Installs the global subscriber. `RUST_LOG` (e.g. `debug` or `my_tool=trace`) overrides the
/// level chosen with `-v`/`-q`.
pub fn init(verbose: u8, quiet: bool) {
    let filter = match std::env::var("RUST_LOG") {
        Ok(directives) if !directives.trim().is_empty() => EnvFilter::new(directives),
        _ => EnvFilter::default().add_directive(level(verbose, quiet).into()),
    };
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_writer(std::io::stderr)
        .with_ansi(std::io::stderr().is_terminal())
        .with_target(false)
        .without_time()
        .init();
}

/// Warnings by default; each `-v` shows one level more, `-q` shows errors only.
fn level(verbose: u8, quiet: bool) -> LevelFilter {
    if quiet {
        return LevelFilter::ERROR;
    }
    match verbose {
        0 => LevelFilter::WARN,
        1 => LevelFilter::INFO,
        2 => LevelFilter::DEBUG,
        _ => LevelFilter::TRACE,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_flags_to_levels() {
        assert_eq!(level(0, false), LevelFilter::WARN);
        assert_eq!(level(1, false), LevelFilter::INFO);
        assert_eq!(level(2, false), LevelFilter::DEBUG);
        assert_eq!(level(5, false), LevelFilter::TRACE);
        assert_eq!(level(0, true), LevelFilter::ERROR);
    }
}
