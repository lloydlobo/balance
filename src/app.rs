pub mod models;
pub mod money;

use anyhow::Result;
use clap::Parser;

use crate::cli::{Cli, Commands};

pub fn run() -> Result<()> {
    let matches = Cli::parse();
    let cli: Cli = matches;
    let command: Commands = cli.command;

    match command {
        Commands::Acc(account) => {
            dbg!(Commands::Acc(account));
            let m_c = money::money_from_str()?;
            dbg!(&m_c);
            let m_c = money::money_minor(m_c, 10_000)?;
            dbg!(&m_c);
            println!("{}", &m_c.is_positive()); // true
        }
        Commands::Tx(transaction) => {
            dbg!(Commands::Tx(transaction));
        }
        Commands::Add { path } => {
            dbg!(Commands::Add { path });
        } // Commands::None => unreachable!(),
    }

    // Continued program logic goes here...

    Ok(())
}
