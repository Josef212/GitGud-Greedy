pub use sub_cmd::SubCmd;

use std::fmt::Formatter;
use clap::Parser;

use crate::{Db, Opts};

mod sub_cmd;
mod add_transaction;
mod add_payroll;
mod add_names;
mod get_names;

use add_transaction::*;
use add_payroll::*;
use add_names::*;
use get_names::*;

#[derive(Parser, Debug)]
pub enum SubCommand {
    #[clap(version="1.0", author="Josef212")]
    AddTransaction(AddTransaction),
    #[clap(version="1.0", author="Josef212")]
    AddTransactionP(AddTransactionP),
    #[clap(version="1.0", author="Josef212")]
    AddPayroll(AddPayroll),
    #[clap(version="1.0", author="Josef212")]
    AddPayrollP(AddPayrollP),
    #[clap(version="1.0", author="Josef212")]
    RepeatPayroll(RepeatPayroll),
    #[clap(version="1.0", author="Josef212")]
    AddTag(AddTag),
    #[clap(version="1.0", author="Josef212")]
    AddCompany(AddCompany),
    #[clap(version="1.0", author="Josef212")]
    AddCategory(AddCategory),
    #[clap(version="1.0", author="Josef212")]
    GetName(GetName),
    #[clap(version="1.0", author="Josef212")]
    GetId(GetId),
    #[clap(version="1.0", author="Josef212")]
    GetTags(GetTags),
    #[clap(version="1.0", author="Josef212")]
    GetCompanies(GetCompanies),
    #[clap(version="1.0", author="Josef212")]
    GetCategories(GetCategories),
}

impl std::fmt::Display for SubCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SubCommand::AddTag(_) => write!(f, "{}", "AddTag"),
            SubCommand::AddCompany(_) => write!(f, "{}", "AddCompany"),
            SubCommand::AddCategory(_) => write!(f, "{}", "AddCategory"),
            SubCommand::AddTransaction(_) => write!(f, "{}", "AddTransaction"),
            SubCommand::AddTransactionP(_) => write!(f, "{}", "AddTransactionP"),
            SubCommand::AddPayroll(_) => write!(f, "{}", "AddPayroll"),
            SubCommand::AddPayrollP(_) => write!(f, "{}", "AddPayrollP"),
            SubCommand::RepeatPayroll(_) => write!(f, "{}", "RepeatPayroll"),
            SubCommand::GetName(_) => write!(f, "{}", "Debug-GetName"),
            SubCommand::GetId(_) => write!(f, "{}", "Debug-GetId"),
            SubCommand::GetTags(_) => write!(f, "{}", "GetTags"),
            SubCommand::GetCompanies(_) => write!(f, "{}", "GetCompanies"),
            SubCommand::GetCategories(_) => write!(f, "{}", "GetCategories"),
            
            #[allow(unreachable_patterns)]
            _ => write!(f, "Not implemented enumerator display")
        }
    }
}

impl SubCommand {
    pub fn execute(&self, db: &Db, opts: &Opts) {
        log::debug!("Processing command: {:?}", self);
        match self {
            // TODO: Can this be done generic???
            SubCommand::AddTag(cmd) => cmd.execute(db, opts),
            SubCommand::AddCompany(cmd) => cmd.execute(db, opts),
            SubCommand::AddCategory(cmd) => cmd.execute(db, opts),
            SubCommand::AddTransaction(cmd) => cmd.execute(db, opts),
            SubCommand::AddTransactionP(cmd) => cmd.execute(db, opts),
            SubCommand::AddPayroll(cmd) => cmd.execute(db, opts),
            SubCommand::AddPayrollP(cmd) => cmd.execute(db, opts),
            // SubCommand::RepeatPayroll(cmd) => cmd.execute(db, opts),
            SubCommand::GetName(cmd) => cmd.execute(db, opts),
            SubCommand::GetId(cmd) => cmd.execute(db, opts),
            SubCommand::GetTags(cmd) => cmd.execute(db, opts),
            SubCommand::GetCompanies(cmd) => cmd.execute(db, opts),
            SubCommand::GetCategories(cmd) => cmd.execute(db, opts),

            #[allow(unreachable_patterns)]
            _ => log::error!("SubCommand {} not implemented.", self),
        }
    }
}