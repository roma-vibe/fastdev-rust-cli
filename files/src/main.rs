//! Entry point: parses the command line, sets up logging, runs the command and turns the outcome
//! into an exit code (0 success, 1 error, 2 invalid usage).

mod cli;
mod commands;
mod config;
mod domain;
mod logging;
mod output;

use std::process::ExitCode;

use clap::Parser;

fn main() -> ExitCode {
    // Prints help or version and exits, or reports invalid arguments and exits with code 2.
    let cli = cli::Cli::parse();
    logging::init(cli.global.verbose, cli.global.quiet);
    tracing::debug!(app = config::APP_NAME, version = env!("CARGO_PKG_VERSION"), "starting");

    match commands::run(&cli.command) {
        Ok(()) => ExitCode::SUCCESS,
        // Whoever read the output has stopped (e.g. `| head`): not an error.
        Err(error) if output::is_broken_pipe(&error) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("error: {error:#}");
            ExitCode::FAILURE
        }
    }
}
