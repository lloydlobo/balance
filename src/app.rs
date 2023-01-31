pub mod models;
pub mod money;
use std::{
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

use anyhow::{anyhow, Error, Result};
use clap::Parser;
use colored::*;

use crate::cli::{Cli, Commands};

pub fn run() -> Result<()> {
    let matches = Cli::parse();
    let cli: Cli = matches;
    let command: Commands = cli.command;

    match command {
        Commands::Acc(account) => {
            println!("{:?}", Commands::Acc(account));
            let m_s = money::money_from_str()?;
            let m_c = money::money_minor(m_s.clone(), 10_000)?;
            println!("{}", &m_c.is_positive()); // true
        }
        Commands::Tx(transaction) => {
            println!("{:#?}", Commands::Tx(transaction));
        }
        Commands::Add { path } => {
            let mut files = Vec::new();
            let mut directories = Vec::new();
            path.iter()
                .map(|p| if !p.exists() { invalid_path_err_eprintln(p) } else { Ok(p) })
                .map(|x| x.ok())
                .for_each(|p| {
                    if let Some(pathbuf) = p {
                        if pathbuf.is_file() {
                            files.push(pathbuf);
                        } else {
                            directories.push(pathbuf);
                        }
                    }
                });
            for f in files.iter() {
                let mut file = File::open(f).unwrap();
                let mut contents = String::new();
                file.read_to_string(&mut contents).unwrap();
                println!("{}", &contents);
            }
        }
    }

    // Continued program logic goes here...

    Ok(())
}

fn invalid_path_err_eprintln(p: &Path) -> Result<&PathBuf, (PathBuf, Error)> {
    eprintln!(
        "{} {} `{}`",
        "!!!".red().bold(),
        "Couldn't find".red(),
        p.to_string_lossy().to_string().red().bold()
    );

    Err((
        p.to_path_buf(),
        anyhow!("The file or directory at `{}` does not exist", p.to_string_lossy()),
    ))
}
