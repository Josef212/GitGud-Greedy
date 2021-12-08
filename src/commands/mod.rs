mod test;
mod sub_cmd;

use clap::Parser;
use test::Test;
pub use sub_cmd::SubCmd;

#[derive(Parser, Debug)]
pub enum SubCommand {
    #[clap(version="1.0", author="Josef212")]
    Test(Test),
}
