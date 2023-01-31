use anyhow::anyhow;

use crate::app::run;

pub mod app;
pub mod cli;

fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();

    match run() {
        Ok(it) => it,
        Err(err) => {
            return Err(err.context(anyhow!("Failed to run `app`")));
        }
    };

    Ok(())
}

#[cfg(test)]
mod tests {
    // use super::*;

    use assert_cmd::Command;

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
}
