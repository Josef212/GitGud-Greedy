use rusqlite::{Connection, Error, params, Params, Row};
use log;

pub mod transaction;
pub mod payroll;
pub mod account;

use crate::models::transaction::Transaction;
use crate::models::payroll::Payroll;
use crate::models::account::Account;

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
date DATE NOT NULL, 
amount REAL NOT NULL, 
tag_id INTEGER REFERENCES tags(id)
";

const PAYROLLS_KEY: &str = "payrolls";
const PAYROLLS_TABLE: &str = "
id INTEGER PRIMARY KEY, 
date DATE NOT NULL, 
gross REAL NOT NULL, 
net REAL NOT NULL, 
ss REAL NOT NULL, 
irpf REAL NOT NULL,
company_id INTEGER REFERENCES companies(id), 
category_id INTEGER REFERENCES categories(id)
";

const ACCOUNTS_KEY: &str = "accounts";
const ACCOUNTS_TABLE: &str = "
id INTEGER PRIMARY KEY,
name TEXT NOT NULL,
amount REAL NOT NULL,
description TEXT
";

pub struct Name {
    pub id: i32,
    pub name: String,
    pub description: String,
}

impl Name {
    pub fn new(id: i32, name: String, description: String) -> Name {
        Name { id, name, description }
    }
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
        self.create_table_if_not_exists(ACCOUNTS_KEY, ACCOUNTS_TABLE)?;
        
        Ok(())
    }
    
    fn create_table_if_not_exists(&self, table_name: &str, table_format: &str) -> Result<usize, Error> {
        log::trace!("Creating {} table if not exists on {}", table_name.to_uppercase(), self.name);
        let sql = format!("CREATE TABLE IF NOT EXISTS {} ({})", table_name, table_format);
        self.connection.execute(&sql, [])
    }
    
    // pub fn decode_date(date_int: i64) -> String {
    //     NaiveDateTime::from_timestamp(date_int, 0)
    //         .format("%d-%m-%Y")
    //         .to_string()
    // }
    // 
    // pub fn code_date(date: &String) -> Result<i64, ParseError> {
    //     let date = NaiveDate::parse_from_str(&date, "%d-%m-%Y")?;
    //     let time = Utc::now().time();
    //     let date_time = NaiveDateTime::new(date, time);
    //     Ok(date_time.timestamp())
    // }
    
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
    
    pub fn insert_account(&self, account: &Account) -> Result<usize, Error> {
        log::trace!("Inserting new account: {:?} to {}", account, self.name);
        
        let sql = format!("INSERT INTO {} (name, amount, description) VALUES (?1, ?2, ?3)", ACCOUNTS_KEY);
        let params = params![&account.name, &account.amount, &account.description];
        
        self.connection.execute(&sql, params)
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
    
    pub fn get_account(&self, name: &str) -> Result<Account, Error> {
        log::trace!("Get account for: {}", name);
        
        let sql = format!("SELECT * FROM {} WHERE name = '{}'", ACCOUNTS_KEY, name);
        log::trace!("Executing sql: {}", sql);
        
        let data: Vec<Account> = self.query(&sql, [], |r| Some(Account::from_row(r)))?;
        
        if data.len() == 0 {
            return Err(Error::QueryReturnedNoRows);
        }

        if data.len() > 1 {
            log::warn!("Found several accounts with same name value [{}]", name);
        }

        Ok(data[0].clone())
    }

    fn get_name_str(&self, table: &str, id: i32) -> Result<String, Error> {
        log::trace!("Get name for id: {}", id);
        
        let sql = format!("SELECT * FROM {} WHERE id = {}", table, id);
        log::trace!("Executing sql: {}", sql);
        
        let names: Vec<String> = self.query(&sql, [], |r| {
            if let Ok(v) = r.get(1) {
                return Some(v);
            }

            None
        })?;

        if names.len() == 0 {
            return Err(Error::QueryReturnedNoRows);
        }
        
        Ok(names[0].clone())
    }
    
    fn get_name_id(&self, table: &str, name: &str) -> Result<i32, Error> {
        log::trace!("Get id for name: {}", name);

        let sql = format!("SELECT * FROM {} WHERE name = '{}'", table, name);
        log::trace!("Executing sql: {}", sql);

        let ids: Vec<i32> = self.query(&sql, [], |r| {
            if let Ok(v) = r.get(0) {
                return Some(v);
            }

            None
        })?;
        if ids.len() == 0 {
            return Err(Error::QueryReturnedNoRows);
        }
        
        if ids.len() > 1 {
            log::warn!("Found several ids with same name value [{}]", name);
        }
        
        Ok(ids[0])
    }

    fn query<T, P: Params, TFn>(&self, sql: &String, params: P, mut convertor: TFn) -> Result<Vec<T>, Error>
        where TFn: FnMut(&Row) -> Option<T> {
        log::trace!("Executing query: {}", sql);

        let mut stmt = self.connection.prepare(&sql)?;
        let mut rows = stmt.query(params)?;

        let mut ret: Vec<T> = Vec::new();

        while let Some(r) = rows.next()? {
            if let Some(value) = convertor(r) {
                ret.push(value);
            }
        }

        Ok(ret)
    }
    
    pub fn edit_account(&self, _name: &str, _amount: f32) -> Result<usize, Error> {
        todo!("Edit account amount is still not implemented")
    }
    
    pub fn get_all_accounts(&self) -> Result<Vec<Account>, Error> {
        log::trace!("Getting all accounts data");
        
        let sql = format!("SELECT * FROM {}", ACCOUNTS_KEY);
        log::trace!("Executing sql: {}", sql);
        
        let ret = self.query(&sql, [], |r| Some(Account::from_row(r)))?;
        
        Ok(ret)
    }
    
    pub fn get_all_tags(&self) -> Result<Vec<Name>, Error> {
        self.get_all_names(TAGS_KEY)
    }
    
    pub fn get_all_companies(&self) -> Result<Vec<Name>, Error> {
        self.get_all_names(COMPANIES_KEY)
    }
    
    pub fn get_all_categories(&self) -> Result<Vec<Name>, Error> {
        self.get_all_names(CATEGORIES_KEY)
    }
    
    fn get_all_names(&self, table: &str) -> Result<Vec<Name>, Error> {
        log::trace!("Getting all {}", table);
        
        let sql = format!("SELECT * FROM {}", table);
        log::trace!("Executing sql: {}", sql);

        let ret = self.query(&sql, [], |r| {
            let id: i32 = r.get_unwrap(0);
            let name: String = r.get_unwrap(1);
            let desc: String = r.get_unwrap(2);

            Some(Name::new(id, name, desc))
        })?;
        
        Ok(ret)
    }
    
    pub fn get_payroll_data(&self, year: Option<u32>, month: Option<u32>) -> Result<Vec<Payroll>, Error> {
        log::trace!("Getting payrolls data");
        
        if year == None && month != None {
            log::error!("Error getting payrolls data. Year is None but month is not. Available combinations are (all none), (year and none month) or (year and month)");
            std::process::exit(0);
        }

        let mut sql = format!("SELECT * FROM {}", PAYROLLS_KEY);
        match year {
            Some(y) => sql += &format!(" WHERE strftime('%Y', date) = '{:04}'", y),
            None => (),
        }
        
        match month {
            Some(m) => sql += &format!(" AND strftime('%m', date) = '{:02}'", m),
            None => (),
        }

        sql += " ORDER BY date ASC";

        log::trace!("Executing sql: {}", sql);
        
        let ret = self.query(&sql, [], |r| Some(Payroll::from_row(r)))?;
        
        Ok(ret)
    }
    
    pub fn get_transaction_data(&self, year: Option<u32>, month: Option<u32>) -> Result<Vec<Transaction>, Error> {
        log::trace!("Getting transactions data");

        if year == None && month != None {
            log::error!("Error getting transactions data. Year is None but month is not. Available combinations are (all none), (year and none month) or (year and month)");
            std::process::exit(0);
        }

        let mut sql = format!("SELECT * FROM {}", TRANSACTIONS_KEY);
        match year {
            Some(y) => sql += &format!(" WHERE strftime('%Y', date) = '{:04}'", y),
            None => (),
        }

        match month {
            Some(m) => sql += &format!(" AND strftime('%m', date) = '{:02}'", m),
            None => (),
        }
        
        sql += " ORDER BY date ASC";

        log::trace!("Executing sql: {}", sql);

        let ret = self.query(&sql, [], |r| Some(Transaction::from_row(r)))?;

        Ok(ret)
    }

    pub fn get_payroll_data_range(&self, _start_year: u32, _start_month: u32, _end_year: u32, _end_month: u32) -> Result<Vec<Payroll>, Error> {
        // TODO: Maybe can use a struct to wrap date ranges. Will probably be handy for other fetch
        todo!()
    }
    
    pub fn get_transaction_data_range(&self, _start_year: u32, _start_month: u32, _end_year: u32, _end_month: u32) -> Result<Vec<Transaction>, Error> {
        // TODO: Maybe can use a struct to wrap date ranges. Will probably be handy for other fetch
        todo!()
    }
}