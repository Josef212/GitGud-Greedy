pub use sub_cmd::SubCmd;

use std::fmt::Formatter;
use clap::Parser;

use crate::{Db, Opts};

mod sub_cmd;
mod test;
mod add_transaction;
mod add_names;

use test::Test;
use add_transaction::AddTransaction;
use add_names::*;

#[derive(Parser, Debug)]
pub enum SubCommand {
    #[clap(version="1.0", author="Josef212")]
    Test(Test),
    #[clap(version="1.0", author="Josef212")]
    AddTransaction(AddTransaction),
    #[clap(version="1.0", author="Josef212")]
    AddTag(AddTag),
    #[clap(version="1.0", author="Josef212")]
    AddCompany(AddCompany),
    #[clap(version="1.0", author="Josef212")]
    AddCategory(AddCategory),
}

impl std::fmt::Display for SubCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SubCommand::Test(_) => write!(f, "{}", "Test"),
            SubCommand::AddTag(_) => write!(f, "{}", "AddTag"),
            SubCommand::AddCompany(_) => write!(f, "{}", "AddCompany"),
            SubCommand::AddCategory(_) => write!(f, "{}", "AddCategory"),
            SubCommand::AddTransaction(_) => write!(f, "{}", "AddTransaction"),
            
            _ => write!(f, "Not implemented enumerator display")
        }
    }
}

impl SubCommand {
    pub fn execute(&self, db: &Db, opts: &Opts) {
        log::debug!("Processing command: {:?}", self);
        match self {
            // TODO: Can this be done generic???
            SubCommand::Test(cmd) => cmd.execute(db, opts),
            SubCommand::AddTag(cmd) => cmd.execute(db, opts),
            SubCommand::AddCompany(cmd) => cmd.execute(db, opts),
            SubCommand::AddCategory(cmd) => cmd.execute(db, opts),
            SubCommand::AddTransaction(cmd) => cmd.execute(db, opts),

            _ => log::error!("SubCommand {} not implemented.", self),
        }
    }
}