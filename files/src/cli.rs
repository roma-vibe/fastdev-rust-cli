//! Command-line interface: global options and the list of commands.
//!
//! Each command's own arguments live next to its handler in `src/commands/<command>.rs`.

use clap::{ArgAction, Args, Parser, Subcommand};

use crate::commands::stats::StatsArgs;
use crate::config;

#[derive(Debug, Parser)]
#[command(name = config::BIN_NAME, version, about = config::ABOUT, propagate_version = true)]
pub struct Cli {
    #[command(flatten)]
    pub global: GlobalArgs,

    #[command(subcommand)]
    pub command: Command,
}

/// Options accepted before or after any command.
#[derive(Debug, Args)]
pub struct GlobalArgs {
    /// Show more log output on stderr (-v info, -vv debug, -vvv trace)
    #[arg(short, long, action = ArgAction::Count, global = true)]
    pub verbose: u8,

    /// Show only errors on stderr
    #[arg(short, long, global = true, conflicts_with = "verbose")]
    pub quiet: bool,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Count lines, words, characters and bytes of files or standard input
    Stats(StatsArgs),
}

#[cfg(test)]
mod tests {
    use clap::CommandFactory;

    use super::*;

    #[test]
    fn cli_definition_is_valid() {
        Cli::command().debug_assert();
    }

    #[test]
    fn global_options_work_after_the_command() {
        let cli = Cli::try_parse_from([config::BIN_NAME, "stats", "-vv", "Cargo.toml"]).unwrap();
        assert_eq!(cli.global.verbose, 2);
        assert!(matches!(cli.command, Command::Stats(_)));
    }

    #[test]
    fn verbose_and_quiet_conflict() {
        assert!(Cli::try_parse_from([config::BIN_NAME, "-v", "-q", "stats"]).is_err());
    }
}
