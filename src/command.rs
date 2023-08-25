use clap::{Parser};
use std::path::PathBuf;

/// Sudoku solver written in Rust
#[derive(Debug, Parser)]
pub struct SudokuCli {
    /// File containing the grid to solve
    pub filename: PathBuf,

    /// Output file for the solved grid
    #[arg(short, long)]
    pub output: Option<String>,
}
