use clap::Parser;
use log;

use crate::commands::sub_cmd::SubCmd;
use crate::models::Db;
use crate::commons::Opts;

#[derive(Parser, Debug)]
pub struct ParseTransaction {
    filename: String,
}

impl SubCmd for ParseTransaction {
    fn execute(&self, _db: &Db, _opts: &Opts) {
        todo!()
    }
}
