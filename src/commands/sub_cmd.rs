use std::io;
use std::str::FromStr;

use crate::models::Db;
use crate::commons::Opts;

pub trait SubCmd {
    fn execute(&self, db: &Db, _opts: &Opts);
}

pub fn ask_parameter<T: FromStr>(msg: &str) -> T {
    let mut buffer = String::new();
    println!("{}: ", msg);
    io::stdin().read_line(&mut buffer).unwrap_or_else(|e| {
        log::error!("Error reading input: {}", e);
        std::process::exit(0);
    });
    
    let value = buffer.trim().parse::<T>().unwrap_or_else(|_| {
        log::error!("Error parsing input.");
        std::process::exit(0);
    });
    
    value
}
