use clap::Parser;
use log;

use crate::commands::sub_cmd::{SubCmd, ask_parameter};
use crate::models::Db;
use crate::commons::Opts;
use crate::models::transaction::Transaction;

#[derive(Parser, Debug)]
pub struct ParsePayroll {
    filename: String,
}

impl SubCmd for ParsePayroll {
    fn execute(&self, db: &Db, _opts: &Opts) {
        todo!()
    }
}