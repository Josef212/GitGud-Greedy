use clap::Parser;
use log;

use crate::commands::sub_cmd::SubCmd;
use crate::models::Db;
use crate::commons::Opts;
use crate::models::transaction::Transaction;

#[derive(Parser, Debug)]
pub struct AddTransaction {
    name: String,
    date: String,
    amount: f32,
    tag: String,
}

impl SubCmd for AddTransaction {
    fn execute(&self, db: &Db, opts: &Opts) {
        let date_int = 0; // TODO: Format date from str to int
        //let tag_id = db.get_tag_id(&self.tag).unwrap_or(-1);
        let tag_id = 0;
        
        let transaction = Transaction::new(&self.name, date_int, self.amount, tag_id);
        db.insert_transaction(&transaction).unwrap_or_else(|e| {
                log::error!("Error inserting transaction: {}", e);
                std::process::exit(1);
            }
        );
        
        log::info!("Transaction [{:?}] inserted successfully", transaction);
    }
}