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
        log::trace!("Creating {} table if not exists.", table_name.to_uppercase());
        let sql = format!("CREATE TABLE IF NOT EXISTS {} ({})", table_name, table_format);
        self.connection.execute(&sql, [])
    }
    
    pub fn insert_transaction(&self, transaction: &Transaction) -> Result<usize, Error> {
        log::trace!("Inserting new transaction: {:?}", transaction);
        
        let sql = format!("INSERT INTO {} (name) values (?1, ?2, ?3, ?4)", TRANSACTIONS_KEY);
        let params = params![&transaction.name, &transaction.date, &transaction.amount, &transaction.tag_id];
        
        self.connection.execute(&sql, params)
    }
    
    pub fn insert_payroll(&self, payroll: &Payroll) -> Result<usize, Error> {
        log::trace!("Inserting new payroll: {:?}", payroll);

        let sql = format!("INSERT INTO {} (name) values (?1, ?2, ?3, ?4, ?5, ?6, ?7)", PAYROLLS_KEY);
        let params = params![&payroll.date, &payroll.gross, &payroll.net, &payroll.ss, &payroll.irpf, &payroll.company, &payroll.category];
        
        self.connection.execute(&sql, params)
    }
}