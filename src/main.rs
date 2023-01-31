use anyhow::anyhow;

use crate::app::run;

pub mod app;
pub mod cli;

fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();

    match run() {
        Ok(it) => Ok(it),
        Err(err) => {
            eprintln!("{:#?}", err.context(anyhow!("Failed to run `app`")));
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io::{self, Read, Write};

    use assert_cmd::Command;
    use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

    use super::*;

    // assert_cmd aims to simplify the process for doing integration testing of CLIs, including:
    // * Finding your crate's binary to test
    // * Assert on the result of your program's run.
    #[test]
    fn it_assert_cmd() {
        // let mut cmd = Command::cargo_bin("bin_fixture").unwrap();
        // cmd.assert().success();
        //
        // let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
        // let output = cmd.unwrap();
        // println!("{:?}", output);
    }

    // # Usage
    // write_red(&format!("Couldn't find...{:?}", p.to_string_lossy())).unwrap();
    fn write_red(msg: &str) -> io::Result<()> {
        let mut stdout = StandardStream::stdout(ColorChoice::Always);
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)))?;
        writeln!(&mut stdout, "{msg}")
    }
}
