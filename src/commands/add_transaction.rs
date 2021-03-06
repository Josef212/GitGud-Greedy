use clap::Parser;
use log;

use crate::commands::sub_cmd::{SubCmd, ask_parameter};
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
    fn execute(&self, db: &Db, _opts: &Opts) {
        let tag_id = db.get_tag_id(&self.tag).unwrap_or_else(|e| {
            log::error!("Could not find id for tag {}. Error: {}", self.tag, e);
            std::process::exit(0);
        });
        
        // TODO: Validate date is properly set. YYYY-MM-DD
        // TODO: Validate all params
        
        let transaction = Transaction::new(&self.name, &self.date, self.amount, tag_id);
        db.insert_transaction(&transaction).unwrap_or_else(|e| {
                log::error!("Error inserting transaction: {}", e);
                std::process::exit(1);
            }
        );
        
        log::info!("Transaction [{:?}] inserted successfully", transaction);
    }
}

#[derive(Parser, Debug)]
pub struct AddTransactionP;

impl SubCmd for AddTransactionP {
    fn execute(&self, db: &Db, _opts: &Opts) {
        let name = ask_parameter::<String>("name");
        let date = ask_parameter::<String>("date");
        let amount = ask_parameter::<f32>("amount");
        let tag = ask_parameter::<String>("tag");
        let tag = db.get_tag_id(&tag).unwrap_or_else(|e| {
            log::error!("Could not find id for tag {}. Error: {}", tag, e);
            std::process::exit(0);
        });

        // TODO: Validate date is properly set. YYYY-MM-DD
        // TODO: Validate parameters
        
        let transaction = Transaction::new(&name, &date, amount, tag);
        db.insert_transaction(&transaction).unwrap_or_else(|e| {
            log::error!("Error inserting transaction: {}", e);
            std::process::exit(1);
        });
        
        log::info!("Transaction [{:?}] inserted successfully", transaction);
    }
}