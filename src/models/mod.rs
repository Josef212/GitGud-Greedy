use rusqlite::{Connection, Error, params, ToSql};
use log;

pub mod transaction;
pub mod payroll;

use crate::models::transaction::Transaction;
use crate::models::payroll::Payroll;

const TAGS_KEY: &str = "tags";
const TAGS_TABLE: &str = "
id INTEGER PRIMARY KEY, 
name TEXT NOT NULL,
description TEXT
";

const COMPANIES_KEY: &str = "companies";
const COMPANIES_TABLE: &str = "
id INTEGER PRIMARY KEY,
name TEXT NOT NULL,
description TEXT
";

const CATEGORIES_KEY: &str = "categories";
const CATEGORIES_TABLE: &str = "
id INTEGER PRIMARY KEY,
name TEXT NOT NULL,
description TEXT
";

const TRANSACTIONS_KEY: &str = "transactions";
const TRANSACTIONS_TABLE: &str = "
id INTEGER PRIMARY KEY, 
name TEXT NOT NULL, 
date INTEGER NOT NULL, 
amount REAL NOT NULL, 
tag_id INTEGER REFERENCES tags(id)
";

const PAYROLLS_KEY: &str = "payrolls";
const PAYROLLS_TABLE: &str = "
id INTEGER PRIMARY KEY, 
date INTEGER NOT NULL, 
gross REAL NOT NULL, 
net REAL NOT NULL, 
ss REAL NOT NULL, 
irpf REAL NOT NULL,
company_id INTEGER REFERENCES companies(id), 
category_id INTEGER REFERENCES categories(id)
";

pub trait ToParams {
    fn to_params(&self) -> [&dyn ToSql];
}

pub struct Db {
    name: String,
    connection: Connection,
}

impl Db {
    pub fn load(db_name: &str) -> Result<Db, Error> {
        log::trace!("Loading db from {}", db_name);
        
        let conn = rusqlite::Connection::open(db_name)?;
        let db = Db {
            name: String::from(db_name),
            connection: conn,
        };
        
        db.init_tables()?;
        
        Ok(db)
    }
    
    pub fn init_tables(&self) -> Result<(), Error> {
        log::trace!("Init tables if not created...");

        self.create_table_if_not_exists(TAGS_KEY, TAGS_TABLE)?;
        self.create_table_if_not_exists(COMPANIES_KEY, COMPANIES_TABLE)?;
        self.create_table_if_not_exists(CATEGORIES_KEY, CATEGORIES_TABLE)?;
        self.create_table_if_not_exists(TRANSACTIONS_KEY, TRANSACTIONS_TABLE)?;
        self.create_table_if_not_exists(PAYROLLS_KEY, PAYROLLS_TABLE)?;
        
        Ok(())
    }
    
    fn create_table_if_not_exists(&self, table_name: &str, table_format: &str) -> Result<usize, Error> {
        log::trace!("Creating {} table if not exists on {}", table_name.to_uppercase(), self.name);
        let sql = format!("CREATE TABLE IF NOT EXISTS {} ({})", table_name, table_format);
        self.connection.execute(&sql, [])
    }
    
    pub fn insert_transaction(&self, transaction: &Transaction) -> Result<usize, Error> {
        log::trace!("Inserting new transaction: {:?} to {}", transaction, self.name);
        
        let sql = format!("INSERT INTO {} (name, date, amount, tag_id) VALUES (?1, ?2, ?3, ?4)", TRANSACTIONS_KEY);
        let params = params![&transaction.name, &transaction.date, &transaction.amount, &transaction.tag_id];
        
        self.connection.execute(&sql, params)
    }
    
    pub fn insert_payroll(&self, payroll: &Payroll) -> Result<usize, Error> {
        log::trace!("Inserting new payroll: {:?} to {}", payroll, self.name);

        let sql = format!("INSERT INTO {} (date, gross, net, ss, irpf, company_id, category_id) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)", PAYROLLS_KEY);
        let params = params![&payroll.date, &payroll.gross, &payroll.net, &payroll.ss, &payroll.irpf, &payroll.company_id, &payroll.category_id];
        
        self.connection.execute(&sql, params)
    }
    
    pub fn insert_tag(&self, tag: &str, description: &str) -> Result<usize, Error> {
        self.insert_name(TAGS_KEY, tag, description)
    }
    
    pub fn insert_company(&self, company: &str, description: &str) -> Result<usize, Error> {
        log::trace!("Inserting company. C: {} - D: {}", company, description);
        self.insert_name(COMPANIES_KEY, company, description)
    }

    pub fn insert_category(&self, category: &str, description: &str) -> Result<usize, Error> {
        self.insert_name(CATEGORIES_KEY, category, description)
    }

    // Using 'name' to describe tag, company, category generically. (something with only [id, name, description])
    fn insert_name(&self, table: &str, value: &str, description: &str) -> Result<usize, Error> {
        log::trace!("Inserting 'name' into {}::{}. Value: {} - Description: {}", self.name, table, value, description);
        
        let sql = format!("INSERT INTO {} (name, description) VALUES (?1, ?2)", table);
        let params = params![&value, &description];
        
        self.connection.execute(&sql, params)
    }
    
    pub fn get_tag_str(&self, tag_id: i32) -> Result<String, Error> {
        self.get_name_str(TAGS_KEY, tag_id)
    }
    
    pub fn get_tag_id(&self, name: &str) -> Result<i32, Error> {
        self.get_name_id(TAGS_KEY, name)
    }
    
    pub fn get_company_str(&self, company_id: i32) -> Result<String, Error> {
        self.get_name_str(COMPANIES_KEY, company_id)
    }
    
    pub fn get_company_id(&self, name: &str) -> Result<i32, Error> {
        self.get_name_id(COMPANIES_KEY, name)
    }

    pub fn get_category_str(&self, category_id: i32) -> Result<String, Error> {
        self.get_name_str(CATEGORIES_KEY, category_id)
    }

    pub fn get_category_id(&self, name: &str) -> Result<i32, Error> {
        self.get_name_id(CATEGORIES_KEY, name)
    }

    fn get_name_str(&self, table: &str, id: i32) -> Result<String, Error> {
        log::trace!("Get name for id: {}", id);
        
        // TODO: Add better and more friendly error handling
        
        let sql = format!("SELECT * FROM {} WHERE id = {}", table, id);
        let mut stmt = self.connection.prepare(&sql)?;
        let mut names = stmt.query_map([], |row| {
            let value: String = row.get(1).unwrap();
            Ok(value)
        })?;
        
        Ok(names.nth(0).unwrap().unwrap())
    }
    
    fn get_name_id(&self, table: &str, name: &str) -> Result<i32, Error> {
        log::trace!("Get id for name: {}", name);

        // TODO: Add better and more friendly error handling
        
        let sql = format!("SELECT * FROM {} WHERE name = '{}'", table, name);
        log::trace!("Executing sql: {}", sql);
        let mut stmt = self.connection.prepare(&sql)?;
        let mut ids = stmt.query_map([], |row| {
            let value: i32 = row.get(0).unwrap();
            Ok(value)
        })?;
        
        Ok(ids.nth(0).unwrap().unwrap())
    }
    
    pub fn get_all_tags(&self) -> Result<Vec<(i32, String, String)>, Error> {
        self.get_all_names(TAGS_KEY)
    }
    
    pub fn get_all_companies(&self) -> Result<Vec<(i32, String, String)>, Error> {
        self.get_all_names(COMPANIES_KEY)
    }
    
    pub fn get_all_categories(&self) -> Result<Vec<(i32, String, String)>, Error> {
        self.get_all_names(CATEGORIES_KEY)
    }
    
    fn get_all_names(&self, table: &str) -> Result<Vec<(i32, String, String)>, Error> {
        log::trace!("Getting all tags");
        
        let mut ret = Vec::new();
        
        let sql = format!("SELECT * FROM {}", table);
        log::trace!("Executing sql: {}", sql);
        let mut stmt = self.connection.prepare(&sql)?;
        let mut rows = stmt.query([])?;
        
        while let Some(r) = rows.next()? {
            let id: i32 = r.get_unwrap(0);
            let name: String = r.get_unwrap(1);
            let desc: String = r.get_unwrap(2);

            ret.push((id, name, desc));
        }
        
        Ok(ret)
    }
}