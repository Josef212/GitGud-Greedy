use clap::Parser;
use log;

use crate::commands::sub_cmd::SubCmd;
use crate::models::Db;
use crate::commons::Opts;
use crate::models::account::Account;
use crate::view_models::balance_summary_vm::BalanceSummaryVm;

#[derive(Parser, Debug)]
pub struct BalanceSummary {
    #[clap(short, long)]
    year: Option<u32>,
    #[clap(short, long)]
    month: Option<u32>,
}

impl SubCmd for BalanceSummary {
    fn execute(&self, db: &Db, _opts: &Opts) {
        // TODO: If no date use current date
        
        let year = self.year;
        let month = self.month;
        
        let payroll = db.get_payroll_data(year, month).unwrap_or_else(|e| {
            log::error!("Error getting payrolls: {}", e);
            std::process::exit(0);
        });
        
        let transactions = db.get_transaction_data(year, month).unwrap_or_else(|e| {
            log::error!("Error getting transactions: {}", e);
            std::process::exit(0);
        });
        
        let vm = BalanceSummaryVm::generate(&payroll, &transactions);
        vm.render(db);
    }
}