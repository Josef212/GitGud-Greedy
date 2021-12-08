use clap::Parser;
use log;
use crate::commands::sub_cmd::SubCmd;

#[derive(Parser, Debug)]
pub struct Test {
    #[clap(short)]
    pub debug: bool,
}

impl SubCmd for Test {
    fn execute(&self) {
        log::debug!("Test debug info: {}", self.debug);
    }
}