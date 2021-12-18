use std::collections::HashMap;
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

        let mut total_amount = 0.0;
        let mut tags_count: HashMap<i32, u32> = HashMap::new();
        let mut amount_per_tags: HashMap<i32, f32> = HashMap::new();

        log::info!("Transactions =====================");
        for t in &transactions {
            
            total_amount += t.amount;
            *tags_count.entry(t.tag_id).or_insert(0) += 1;
            *amount_per_tags.entry(t.tag_id).or_insert(0.0) += t.amount;
            
            let tag = db.get_tag_str(t.tag_id).unwrap_or(String::from("Unknown"));

            log::info!(
                "  [{:04}] Name: {} - Date: {} - Amount: {:.2} - Tag: {}",
                t._id, t.name, t.date, t.amount, tag
            );
        }
        
        let amount_avg: f32 = total_amount / (transactions.len() as f32);
        
        log::info!("======================================");
        log::info!("Total spent: {:.2}", total_amount);
        log::info!("Total tags: {}", tags_count.len());
        log::info!("Avg per tag: {:.2}", amount_avg);
        
        for t in tags_count {
            let tag_id = t.0;
            let count: u32 = t.1;
            let tag = db.get_tag_str(tag_id).unwrap_or(String::from("Unknown"));
            let amount = amount_per_tags[&tag_id];
            let avg: f32 = amount / (count as f32);
            
            log::info!(
                "  [{:02}] {} - A: {:.2} - C: {} - Avg: {:.2}",
                tag_id, tag, amount, count, avg
            );
        }
    }
}
