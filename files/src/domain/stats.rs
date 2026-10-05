//! Text statistics behind the `stats` command: lines, words, characters and bytes.

use std::io::{self, BufRead};
use std::ops::AddAssign;

use serde::Serialize;

/// Statistics of one input, or the sum of several.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize)]
pub struct Counts {
    /// Newline characters, like `wc -l` (a last line without `\n` is not counted).
    pub lines: u64,
    /// Runs of non-whitespace characters (Unicode whitespace separates words).
    pub words: u64,
    /// Unicode scalar values; each invalid UTF-8 sequence counts as one character.
    pub chars: u64,
    pub bytes: u64,
}

impl AddAssign for Counts {
    fn add_assign(&mut self, other: Self) {
        self.lines += other.lines;
        self.words += other.words;
        self.chars += other.chars;
        self.bytes += other.bytes;
    }
}

/// Reads `reader` to the end, one line at a time, and counts it. Memory use is bounded by the
/// longest line, so large files are fine.
pub fn count(mut reader: impl BufRead) -> io::Result<Counts> {
    let mut counts = Counts::default();
    let mut line = Vec::new();
    loop {
        line.clear();
        if reader.read_until(b'\n', &mut line)? == 0 {
            return Ok(counts);
        }
        let text = String::from_utf8_lossy(&line);
        counts += Counts {
            lines: u64::from(line.ends_with(b"\n")),
            words: text.split_whitespace().count() as u64,
            chars: text.chars().count() as u64,
            bytes: line.len() as u64,
        };
    }
}

/// Statistics of several named inputs and their total.
#[derive(Debug, Default, Serialize)]
pub struct Report {
    pub inputs: Vec<Entry>,
    pub total: Counts,
}

/// One input of a [`Report`]: a file path, or `-` for standard input.
#[derive(Debug, Serialize)]
pub struct Entry {
    pub name: String,
    #[serde(flatten)]
    pub counts: Counts,
}

impl Report {
    pub fn add(&mut self, name: impl Into<String>, counts: Counts) {
        self.total += counts;
        self.inputs.push(Entry { name: name.into(), counts });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn count_str(text: &str) -> Counts {
        count(text.as_bytes()).unwrap()
    }

    #[test]
    fn empty_input_has_zero_counts() {
        assert_eq!(count_str(""), Counts::default());
    }

    #[test]
    fn counts_lines_words_chars_and_bytes() {
        let counts = count_str("one two\n  three\n\nfour");
        assert_eq!(counts, Counts { lines: 3, words: 4, chars: 21, bytes: 21 });
    }

    #[test]
    fn counts_unicode_characters_and_whitespace() {
        // "é", "ё" and "ж" take two bytes each; U+3000 (ideographic space, three bytes) separates words.
        let counts = count_str("café\u{3000}ёж\n");
        assert_eq!(counts, Counts { lines: 1, words: 2, chars: 8, bytes: 13 });
    }

    #[test]
    fn invalid_utf8_counts_as_replacement_characters() {
        let counts = count(&b"ab\xFFc\n"[..]).unwrap();
        assert_eq!(counts, Counts { lines: 1, words: 1, chars: 5, bytes: 5 });
    }

    #[test]
    fn report_sums_inputs() {
        let mut report = Report::default();
        report.add("a.txt", count_str("a b\n"));
        report.add("-", count_str("c\n"));
        assert_eq!(report.inputs.len(), 2);
        assert_eq!(report.inputs[1].name, "-");
        assert_eq!(report.total, Counts { lines: 2, words: 3, chars: 6, bytes: 6 });
    }
}
