use clap::Parser;
use log;

use crate::commands::sub_cmd::SubCmd;
use crate::models::{Db, Name};
use crate::commons::Opts;

#[derive(Parser, Debug)]
pub struct GetName {
    table: String,
    id: i32,
}

impl SubCmd for GetName {
    fn execute(&self, db: &Db, _opts: &Opts) {
        let name = match self.table.to_lowercase().as_str() { 
            "tag" => db.get_tag_str(self.id),
            "company" => db.get_company_str(self.id),
            "category" => db.get_category_str(self.id),
            _ => {
                log::error!("Invalid table");
                std::process::exit(0);
            },
        }.unwrap_or_else(|e| {
            log::error!("Error processing command {:?}. Error: {}", self, e);
            std::process::exit(0);
        });
        
        log::info!("{} name for {} is {}", self.table, self.id, name);
    }
}

#[derive(Parser, Debug)]
pub struct GetId {
    table: String,
    name: String,
}

impl SubCmd for GetId {
    fn execute(&self, db: &Db, _opts: &Opts) {
        let id = match self.table.to_lowercase().as_str() {
            "tag" => db.get_tag_id(&self.name),
            "company" => db.get_company_id(&self.name),
            "category" => db.get_category_id(&self.name),
            _ => {
                log::error!("Invalid table");
                std::process::exit(0);
            },
        }.unwrap_or_else(|e| {
            log::error!("Error processing command {:?}. Error: {}", self, e);
            std::process::exit(0);
        });

        log::info!("{} id for {} is {}", self.table, self.name, id);
    }
}

#[derive(Parser, Debug)]
pub struct GetTags;
#[derive(Parser, Debug)]
pub struct GetCompanies;
#[derive(Parser, Debug)]
pub struct GetCategories;

impl SubCmd for GetTags {
    fn execute(&self, db: &Db, _opts: &Opts) {
        let tags = db.get_all_tags().unwrap_or_else(|e| {
            log::error!("Error getting tags list. Error: {}", e);
            std::process::exit(0);
        });
        
        // TODO: Generate view model and view renderer
        
        list_all("tags", &tags);
    }
}

impl SubCmd for GetCompanies {
    fn execute(&self, db: &Db, _opts: &Opts) {
        let companies = db.get_all_companies().unwrap_or_else(|e| {
            log::error!("Error getting companies list. Error: {}", e);
            std::process::exit(0);
        });

        // TODO: Generate view model and view renderer

        list_all("companies", &companies);
    }
}

impl SubCmd for GetCategories {
    fn execute(&self, db: &Db, _opts: &Opts) {
        let categories = db.get_all_categories().unwrap_or_else(|e| {
            log::error!("Error getting categories list. Error: {}", e);
            std::process::exit(0);
        });

        // TODO: Generate view model and view renderer

        list_all("categories", &categories);
    }
}

fn list_all(table: &str, list: &Vec<Name>) {
    print!("List of {}: \n", table);
    for value in list {
        let id = value.id;
        let name = &value.name;
        let desc = &value.description;

        print!("[{}]: {} - {}\n", id, name, desc);
    }

    println!();
}