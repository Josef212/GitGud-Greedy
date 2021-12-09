mod test;
mod sub_cmd;

use std::fmt::Formatter;
use clap::Parser;
use test::Test;
pub use sub_cmd::SubCmd;

#[derive(Parser, Debug)]
pub enum SubCommand {
    #[clap(version="1.0", author="Josef212")]
    Test(Test),
}

impl std::fmt::Display for SubCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}