use clap::Parser;
use log;

use crate::commands::sub_cmd::SubCmd;
use crate::models::Db;
use crate::commons::Opts;

#[derive(Parser, Debug)]
pub struct TransactionData {
    #[clap(short, long)]
    year: Option<u32>,
    #[clap(short, long)]
    month: Option<u32>,
}

impl SubCmd for TransactionData {
    fn execute(&self, db: &Db, _opts: &Opts) {
        let transactions = db.get_transaction_data(self.year, self.month).unwrap_or_else(|e| {
            log::error!("Error getting transactions: {}", e);
            std::process::exit(0);
        });

        // TODO: Format results
        // TODO: Generate view model and view renderer

        log::info!("Transactions =====================");
        for t in &transactions {
            let tag = db.get_tag_str(t.tag_id).unwrap_or(String::from("Unknown"));

            log::info!(
                "  [{:04}] Name: {} - Date: {} - Amount: {} - Tag: {}",
                t._id, t.name, t.date, t.amount, tag
            );
        }
    }
}
