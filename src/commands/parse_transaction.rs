use clap::Parser;
use log;

use std::path::Path;
use csv::StringRecord;
use serde::Deserialize;

use crate::commands::sub_cmd::SubCmd;
use crate::models::Db;
use crate::commons::Opts;
use crate::models::transaction::Transaction;

#[derive(Parser, Debug)]
pub struct ParseTransaction {
    filename: String,
}

#[derive(Debug, Deserialize)]
struct TransRow {
    amount: String,
    date: String,
    name: String,
    tag: String,
}

impl TransRow {
    fn new() -> TransRow {
        TransRow {
            amount: String::new(),
            date: String::new(),
            name: String::new(),
            tag: String::new(),
        }
    }
    
    fn to_transaction(&self, db: &Db) -> Result<Transaction, Vec<String>> {
        let mut errors = Vec::new();
        
        let amount = self.amount.replace(',', ".");
        let amount = amount.parse::<f32>().unwrap_or_else(|e| {
            let e = format!("Error parsing amount [{}]. E: {}", self.amount, e);
            errors.push(e);
            
            0.0
        });
        
        let tag_id = db.get_tag_id(&self.tag).unwrap_or_else(|e| {
            let e = format!("Error getting tag id from [{}]. E: {}", self.tag, e);
            errors.push(e);
            
            0
        });
        
        // TODO: Verify date has a good format. (YYYY-MM-DD)
        let date = self.date.replace('/', "-");
        
        if errors.len() > 0 {
            return Err(errors);
        }
        
        Ok(Transaction::new(&self.name, &date, amount, tag_id))
    }
}

impl SubCmd for ParseTransaction {
    fn execute(&self, db: &Db, _opts: &Opts) {
        if !Path::new(&self.filename).exists() {
            log::error!("File [{}] does not exists", self.filename);
            std::process::exit(0);
        }
        
        log::info!("Parsing transactions from file: {}", self.filename);
        
        let mut reader = csv::Reader::from_path(&self.filename).unwrap_or_else(|e| {
            log::error!("Error creating csv reader from file [{}]. Error: {}", self.filename, e);
            std::process::exit(0);
        });
        
        log::trace!("Csv reader created successfully");
        
        let mut i = 0;
        let mut transaction_rows = 0;
        let mut error_rows = 0;
        let mut errors = Vec::new();
        
        for result in reader.records() {
            let record = result.unwrap_or_else(|e| {
                errors.push((i, format!("Error getting string record. E: {}", e)));
                StringRecord::new()
            });
            
            let row: TransRow = record
                .deserialize(None)
                .unwrap_or_else(|e| {
                    errors.push((i, format!("Error deserializing row. E: {}", e)));
                    TransRow::new()
                });
            
            let transaction = row.to_transaction(db);
            match transaction {
                Err(er) => { 
                    for e in er {
                        error_rows += 1;
                        errors.push((i, e));
                    }
                },
                Ok(t) => {
                    transaction_rows += 1;
                    match db.insert_transaction(&t) {
                        Ok(_) => (),
                        Err(e) => {
                            error_rows += 1;
                            errors.push((i, format!("Error inserting transaction. E: {}", e)));
                        },
                    }
                },
            }
            
            i += 1;
        }
        
        log::info!("Parse complete. Success: {} - Error: {}", transaction_rows, error_rows);
        if errors.len() > 0 {
            log::info!("Errors:");
            for (i, e) in errors {
                log::info!("[L:{}] {}", i, e);
            }
        }
    }
}
