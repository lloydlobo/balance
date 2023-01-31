use std::fmt::Display;

use clap::{Args, Parser, Subcommand, ValueEnum};

/// Balance CLI.
#[derive(Debug, Parser)] // requires `clap/derive` feature
#[command(name = "balance")]
#[command(about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

/// `SUBCOMMAND` for `Args`.
#[derive(Debug, Subcommand)]
pub enum Commands {
    /// account module
    #[command(arg_required_else_help = true)]
    Acc(Acc),
    /// transaction module
    #[command(arg_required_else_help = true)]
    Tx(Tx),
}

#[derive(Debug, Args)]
pub struct Acc {
    #[arg(short = 'A', long)]
    pub account: String,
    #[arg(short = 'a', long)]
    pub amount: i64,
    #[arg(short, long)]
    pub budget_month: i64,
    #[arg(short, long)]
    pub budget_year: i64,
}

#[derive(Debug, Args)]
pub struct Tx {
    /// the datetime of entry is recorded if not passed.
    #[arg(short = 'd', long)]
    pub date: Option<u32>,
    #[arg(short = 'a', long)]
    pub amount: i64,
    #[arg(short = 'A', long)]
    pub account: String,
    #[arg(short = 'O', long)]
    pub offset_account: Option<String>,
    #[arg(short = 'D', long)]
    pub description: Option<String>,
}

// https://github.com/clap-rs/clap/blob/956dc6a6daa7c2d101db286fc366ee1c41b32cf2/examples/git-derive.rs
#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
pub enum ColorWhen {
    Always,
    Auto,
    Never,
}

impl Display for ColorWhen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // The value is `None` for skipped variants.
        self.to_possible_value().expect("should skip no values").get_name().fmt(f)
    }
}
