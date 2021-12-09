use clap::Parser;
use log;

use crate::commands::sub_cmd::SubCmd;
use crate::models::Db;
use crate::commons::Opts;

#[derive(Parser, Debug)]
pub struct Test {
    #[clap(short)]
    pub debug: bool,
}

impl SubCmd for Test {
    fn execute(&self, db: &Db, opts: &Opts) {
        log::debug!("Test debug info: {}", self.debug);
    }
}