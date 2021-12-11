use clap::Parser;
use log;

use crate::commands::sub_cmd::SubCmd;
use crate::models::Db;
use crate::commons::Opts;
use crate::models::transaction::Transaction;

#[derive(Parser, Debug)]
pub struct AddTag {
    name: String,
    #[clap(default_value="")]
    description: String,
}

impl SubCmd for AddTag {
    fn execute(&self, db: &Db, _opts: &Opts) {
        db.insert_tag(&self.name, &self.description).unwrap_or_else(|e| {
            log::error!("Error inserting tag: {}", e);
            std::process::exit(1);
        });
        
        log::info!("Tag [{:?}] inserted successfully", self);
    }
}

#[derive(Parser, Debug)]
pub struct AddCompany {
    name: String,
    #[clap(default_value="")]
    description: String,
}

impl SubCmd for AddCompany {
    fn execute(&self, db: &Db, _opts: &Opts) {
        db.insert_company(&self.name, &self.description).unwrap_or_else(|e| {
            log::error!("Error inserting company: {}", e);
            std::process::exit(1);
        });

        log::info!("Company [{:?}] inserted successfully", self);
    }
}

#[derive(Parser, Debug)]
pub struct AddCategory {
    name: String,
    #[clap(default_value="")]
    description: String,
}

impl SubCmd for AddCategory {
    fn execute(&self, db: &Db, _opts: &Opts) {
        db.insert_category(&self.name, &self.description).unwrap_or_else(|e| {
            log::error!("Error inserting category: {}", e);
            std::process::exit(1);
        });

        log::info!("Category [{:?}] inserted successfully", self);
    }
}
