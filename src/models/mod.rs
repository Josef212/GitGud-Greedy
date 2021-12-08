use rusqlite::{Connection, Error};
use log;

pub mod transaction;
pub mod payroll;

const DB_NAME: &str = "gg_financials.db";

const TAGS_KEY: &str = "tags";
const TAGS_TABLE: &str = "
id INTEGER PRIMARY KEY, 
name TEXT NOT NULL
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
name TEXT NOT NULL, 
date INTEGER NOT NULL, 
gross REAL NOT NULL, 
net REAL NOT NULL, 
ss REAL NOT NULL, 
irpf REAL NOT NULL
";

pub struct Db {
    name: String,
    connection: Connection,
}

impl Db {
    pub fn load() -> Result<Db, Error> {
        log::trace!("Loading db from {}", DB_NAME);
        
        let conn = rusqlite::Connection::open(DB_NAME)?;
        let db = Db {
            name: String::from(DB_NAME),
            connection: conn,
        };
        
        db.init_tables()?;
        
        Ok(db)
    }
    
    pub fn init_tables(&self) -> Result<(), Error> {
        log::trace!("Init tables if not created...");

        self.create_table_if_not_exists(TAGS_KEY, TAGS_TABLE)?;
        self.create_table_if_not_exists(TRANSACTIONS_KEY, TRANSACTIONS_TABLE)?;
        self.create_table_if_not_exists(PAYROLLS_KEY, PAYROLLS_TABLE)?;
        
        Ok(())
    }
    
    fn create_table_if_not_exists(&self, table_name: &str, table_format: &str) -> rusqlite::Result<usize> {
        log::trace!("Creating {} table if not exists.", table_name.to_uppercase());
        let sql = format!("CREATE TABLE IF NOT EXISTS {} ({})", table_name, table_format);
        self.connection.execute(&sql, [])
    }
}