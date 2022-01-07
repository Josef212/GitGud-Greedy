use clap::Parser;
use log;

use crate::commands::sub_cmd::SubCmd;
use crate::models::Db;
use crate::commons::Opts;
use crate::models::account::Account;

#[derive(Parser, Debug)]
pub struct AddAccount {
    name: String,
    amount: f32,
    #[clap(default_value="")]
    description: String,
}

impl SubCmd for AddAccount {
    fn execute(&self, db: &Db, _opts: &Opts) {
        let account = Account::new(&self.name, self.amount, &self.description);
        db.insert_account(&account).unwrap_or_else(|e| {
            log::error!("Error inserting account [{:?}]: {}", account, e);
            std::process::exit(1)
        });
        
        log::info!("Account [{:?}] inserted successfully", account);
    }
}

#[derive(Parser, Debug)]
pub struct GetAccount {
    #[clap(default_value="all")]
    name: String,
}

impl SubCmd for GetAccount {
    fn execute(&self, _db: &Db, _opts: &Opts) {
        if self.name == "all" {
            self.execute_all(_db);
            return;
        }

        // TODO: Use a vm
        let account = _db.get_account(&self.name).unwrap_or_else(|e| {
            log::error!("Error getting account data ({}): {}", self.name, e);
            std::process::exit(0)
        });

        log::info!("[{}]: {} - ({}) | {}", account._id, account.name, account.amount, account.description);
    }
}

impl GetAccount {
    fn execute_all(&self, db: &Db) {
        let accounts = db.get_all_accounts().unwrap_or_else(|e| {
            log::error!("Error getting all account data: {}", e);
            std::process::exit(0)
        });
        
        // TODO: Use a vm
        
        let mut total: f32 = 0.0;
        for a in &accounts {
            total += a.amount;
            
            log::info!("[{}]: {} - ({}) | {}", a._id, a.name, a.amount, a.description);
        }
        
        log::info!("Total: {}", total);
    }
}

#[derive(Parser, Debug)]
pub struct SetAccountBalance {
    name: String,
    amount: f32,
}

impl SubCmd for SetAccountBalance {
    fn execute(&self, _db: &Db, _opts: &Opts) {
        todo!()
    }
}