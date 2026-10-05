//! `stats`: count lines, words, characters and bytes of files or standard input.

use std::fs::File;
use std::io::{self, BufReader, IsTerminal, Write};
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use clap::Args;
use tracing::{debug, info};

use crate::domain::stats::{self, Counts, Report};
use crate::output::{self, Format};

/// The name of standard input on the command line and in results.
const STDIN: &str = "-";

#[derive(Debug, Args)]
pub struct StatsArgs {
    /// Files to read; `-` reads standard input, which is also the default without files
    #[arg(value_name = "FILE")]
    pub files: Vec<PathBuf>,

    /// Output format
    #[arg(short, long, value_enum, default_value_t)]
    pub format: Format,
}

pub fn run(args: &StatsArgs, out: &mut impl Write) -> Result<()> {
    let inputs: Vec<&Path> = if args.files.is_empty() {
        if io::stdin().is_terminal() {
            bail!("no input: pass one or more files, or pipe text into standard input");
        }
        vec![Path::new(STDIN)]
    } else {
        args.files.iter().map(PathBuf::as_path).collect()
    };

    let mut report = Report::default();
    for input in inputs {
        let counts = count_input(input).with_context(|| format!("cannot read {}", input.display()))?;
        debug!(input = %input.display(), ?counts, "counted");
        report.add(input.display().to_string(), counts);
    }
    info!(inputs = report.inputs.len(), bytes = report.total.bytes, "stats ready");

    match args.format {
        Format::Text => write_text(out, &report),
        Format::Json => output::write_json(out, &report),
    }
    .context("cannot write the result")
}

fn count_input(input: &Path) -> io::Result<Counts> {
    if input == Path::new(STDIN) {
        stats::count(io::stdin().lock())
    } else {
        stats::count(BufReader::new(File::open(input)?))
    }
}

/// A right-aligned table like `wc`, with a header and a `total` row when there are several inputs.
fn write_text(out: &mut impl Write, report: &Report) -> io::Result<()> {
    let mut rows: Vec<(&str, &Counts)> =
        report.inputs.iter().map(|entry| (entry.name.as_str(), &entry.counts)).collect();
    if rows.len() > 1 {
        rows.push(("total", &report.total));
    }
    let width = rows
        .iter()
        .flat_map(|(_, counts)| [counts.lines, counts.words, counts.chars, counts.bytes])
        .map(|number| number.to_string().len())
        .fold("lines".len(), usize::max);

    writeln!(out, "{:>width$} {:>width$} {:>width$} {:>width$}  name", "lines", "words", "chars", "bytes")?;
    for (name, counts) in rows {
        writeln!(
            out,
            "{:>width$} {:>width$} {:>width$} {:>width$}  {name}",
            counts.lines, counts.words, counts.chars, counts.bytes
        )?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn render(report: &Report) -> String {
        let mut out = Vec::new();
        write_text(&mut out, report).unwrap();
        String::from_utf8(out).unwrap()
    }

    #[test]
    fn one_input_has_no_total_row() {
        let mut report = Report::default();
        report.add("notes.txt", Counts { lines: 2, words: 5, chars: 30, bytes: 31 });
        assert_eq!(render(&report), "lines words chars bytes  name\n    2     5    30    31  notes.txt\n");
    }

    #[test]
    fn several_inputs_get_a_total_row_and_wider_columns() {
        let mut report = Report::default();
        report.add("big.log", Counts { lines: 1, words: 1, chars: 1, bytes: 1_234_567 });
        report.add("-", Counts { lines: 1, words: 2, chars: 3, bytes: 4 });
        let expected = [
            "  lines   words   chars   bytes  name",
            "      1       1       1 1234567  big.log",
            "      1       2       3       4  -",
            "      2       3       4 1234571  total",
            "",
        ];
        assert_eq!(render(&report), expected.join("\n"));
    }
}
