//! Domain logic: what the tool computes, independent of the command line.
//!
//! Modules here take values or readers and return values or errors. They never parse arguments,
//! open files by name, print, read environment variables or exit the process.

pub mod stats;
