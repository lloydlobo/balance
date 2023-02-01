pub mod models;
pub mod money;

use std::{
    fs,
    fs::File,
    io::ErrorKind,
    path::{Path, PathBuf},
};

use anyhow::{anyhow, Error, Result};
use clap::Parser;
use colored::*;

use crate::{
    app::models::{Account, Transaction},
    cli::{Cli, Commands},
};

pub fn run() -> anyhow::Result<(), anyhow::Error> {
    let matches = Cli::parse();
    let cli: Cli = matches;
    let command: Commands = cli.command;
    match command {
        Commands::Acc(account) => {
            println!("{:?}", Commands::Acc(account));
            let m_s = money::money_from_str()?;
            let m_c = money::money_minor(m_s.clone(), 10_000)?;
            println!("{}", &m_c.is_positive()); // true
            Ok::<(), Error>(())
        }
        Commands::Tx(transaction) => {
            let transaction = Transaction {
                date: match transaction.date {
                    Some(val) => val.to_string(),
                    None => chrono::offset::Local::now().to_string(),
                },
                amount: transaction.amount,
                offset_account: transaction.offset_account.unwrap_or_default(),
                description: transaction.description.unwrap_or_default(),
                account: Account {
                    account: transaction.account,
                    amount: 0,
                    budget_month: 0,
                    budget_year: 0,
                },
            };
            read_write_new_tx(transaction)?;
            Ok(())
        }
        Commands::Add { path } => {
            let mut files_path = Vec::new();
            let mut directories_path = Vec::new();
            path.iter()
                .map(|p| if !p.exists() { invalid_path_err_eprintln(p) } else { Ok(p) })
                .map(|x| x.ok())
                .for_each(|p| {
                    if let Some(pathbuf) = p {
                        if pathbuf.is_file() {
                            files_path.push(pathbuf);
                        } else {
                            directories_path.push(pathbuf);
                        }
                    }
                });
            for path in files_path.iter() {
                let contents = match fs::read_to_string(path) {
                    Ok(it) => it,
                    Err(err) => {
                        let ctx: Error = print_err(
                            "Failed to read `{f}` file content to a string",
                            &anyhow!("`{err:#?}`",).to_string(),
                        );
                        return Err(anyhow!("`{err:#?}`",).context(ctx));
                    }
                };
                println!("{}", &contents);
            }
            Ok(())
        }
    }?;

    // Continued program logic goes here...

    Ok(())
}

/// Append to a file instead of overwriting previous contents.
///
/// # Errors
/// This function will return an error if:
/// * [`NotFound`]: The specified file does not exist and neither `create` or `create_new` is set.
/// * [`NotFound`]: One of the directory components of the file path does not exist.
/// * [`PermissionDenied`]: User lacks permission to get the specified access rights for the file.
/// * [`PermissionDenied`]: User lacks permission to open one of the directory components at path.
/// * [`AlreadyExists`]: `create_new` was specified and the file already exists.
/// * [`InvalidInput`]: Invalid combinations of open options.
///
/// # Panics
/// Panics if any of the error unwraps on `Err`.
///
/// [See also](https://tms-dev-blog.com/how-to-read-and-write-yaml-in-rust-with-serde/)
fn read_write_new_tx(transaction: Transaction) -> Result<()> {
    let path = "transactions.yml";
    let mut f = match File::options().append(true).open(path) {
        Ok(it) => it,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => {
                eprintln!(
                    "{:#?}",
                    anyhow!("`{path}`: `{}`: {}", ErrorKind::NotFound, anyhow!("{err}"))
                        .context(anyhow!("Failed to open file at path: `{path}`",))
                );
                let file = File::create(path)?;
                print_success(&format!("Created file at `{}`", path), &format!("{file:?}"));
                file
            }
            // In comprehensive and thorough tests that want to verify that a test doesn't return
            // any known incorrect error kind, you may want to cut-and-paste the current full list
            // of errors from here into your test code, and then match `_` as the correct case. This
            // seems counterintuitive, but it will make your tests more robust. In particular, if
            // you want to verify that your code does produce an unrecognized error kind, the robust
            // solution is to check for all the recognized error kinds and fail in those cases.
            ErrorKind::PermissionDenied
            | ErrorKind::ConnectionRefused
            | ErrorKind::ConnectionReset
            | ErrorKind::ConnectionAborted
            | ErrorKind::NotConnected
            | ErrorKind::AddrInUse
            | ErrorKind::AddrNotAvailable
            | ErrorKind::BrokenPipe
            | ErrorKind::AlreadyExists
            | ErrorKind::WouldBlock
            | ErrorKind::InvalidInput
            | ErrorKind::InvalidData
            | ErrorKind::TimedOut
            | ErrorKind::WriteZero
            | ErrorKind::Interrupted
            | ErrorKind::Unsupported
            | ErrorKind::UnexpectedEof
            | ErrorKind::OutOfMemory
            | ErrorKind::Other => {
                let ctx = print_err(&err.kind().to_string(), &anyhow!("{err:#?}").to_string());
                panic!("{ctx}")
            }
            _ => panic!("{err:#?}",),
        },
    };

    serde_yaml::to_writer(&mut f, &transaction)?;
    Ok(())
}

fn print_err(msg: &str, err_str: &str) -> Error {
    let err = anyhow!("{} {} `{}`", "!!!".red().bold(), msg.red(), err_str.red().bold());
    eprintln!("{err:#?}");
    err
}
// FIXME: Prints ASCII escape codes or something if `tty` doesn't support `colored` output.
fn print_success(msg: &str, err_str: &str) -> String {
    let ctx =
        anyhow!(format!("{} {} `{}`", ">>>".green().bold(), msg.green(), err_str.green().bold()));
    println!("{ctx:#?}");
    ctx.to_string()
}
fn invalid_path_err_eprintln(p: &Path) -> Result<&PathBuf, (PathBuf, Error)> {
    let err_ctx = print_err("Couldn't find", &p.to_string_lossy());
    Err((
        p.to_path_buf(),
        anyhow!("The file or directory at `{}` does not exist", p.to_string_lossy())
            .context(err_ctx),
    ))
}
