use clap::Parser;
use log;

use crate::commands::sub_cmd::SubCmd;
use crate::models::Db;
use crate::commons::Opts;
use crate::view_models::transaction_data_vm::TransactionDataVm;

#[derive(Parser, Debug)]
pub struct TransactionData {
    #[clap(short, long)]
    year: Option<u32>,
    #[clap(short, long)]
    month: Option<u32>,
    #[clap(short, long)]
    list: bool,
    #[clap(short, long)]
    plot: bool,
}

impl SubCmd for TransactionData {
    fn execute(&self, db: &Db, _opts: &Opts) {
        let transactions = db.get_transaction_data(self.year, self.month).unwrap_or_else(|e| {
            log::error!("Error getting transactions: {}", e);
            std::process::exit(0);
        });

        let vm = TransactionDataVm::generate(&transactions);
        vm.render(db);
        
        if self.list {
            vm.full_list(db);
        }
        
        if self.plot {
            vm.plot(db);
        }
    }
}
