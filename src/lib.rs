// https://github.com/clap-rs/clap
// https://docs.rs/clap/latest/clap/
// https://rust-lang-nursery.github.io/rust-cookbook/database/sqlite.html

mod commons;
mod commands;
mod models;

use std::error::Error;
use std::io::Write;
use std::str::FromStr;
use chrono::Local;
use env_logger::fmt::Color;
use log::LevelFilter;

use crate::commons::Opts;
use crate::commands::{SubCommand, SubCmd};
use crate::models::Db;

pub struct Cli {
    pub opts: Opts,
    pub db: Db,
}

impl Cli {
    pub fn print_info(&self) {
        let opts = &self.opts;
        
        log::info!("Database name: {}", opts.get_db_name());
        log::info!("LogLevel: {}", log::max_level());
    }

    pub fn match_subcommand(&self) {
        match &self.opts.get_sub_cmd() {
            Some(sub_cmd) => self.execute_subcommand(sub_cmd),
            None => log::error!("No matching subcommand found. Use -h or --help to see the list."),
        }
    }
    
    fn execute_subcommand(&self, cmd: &SubCommand) {
        match cmd {
            // TODO: Can this be done generic???
            SubCommand::Test(t) => self.execute_sub_cmd(t),
            
            _ => log::error!("SubCommand {} not implemented.", cmd),
        }
    }
    
    fn execute_sub_cmd(&self, sub_cmd: &dyn SubCmd) {
        sub_cmd.execute(&self.db, &self.opts);
    }
}

pub fn init() -> Cli {
    let opts: Opts = Opts::new();
    init_logger(&opts.get_log());
    let db = load_db(&opts.get_db_name());
    
    Cli { 
        opts,
        db,
    }
}

fn init_logger(log_level: &String) {
    env_logger::Builder::new()
        .format(|buf, record| {
            let level = record.level();
            let mut style = buf.style();
            style
                .set_bold(true)
                .set_color(match level {
                    log::Level::Error => Color::Red,
                    log::Level::Warn => Color::Yellow,
                    log::Level::Trace => Color::Cyan,
                    _ => Color::White
                });

            writeln!(buf,
                     "({}) [{}]: {}",
                     Local::now().format("%Y-%m-%d %H:%M:%S"),
                     style.value(level),
                     record.args()
            )
        })
        .filter_level(LevelFilter::from_str(log_level.as_str()).unwrap_or(LevelFilter::Error))
        .init();

    std::panic::set_hook(Box::new(|err| {log::error!("{}", err)}));
}

fn load_db(db_name: &str) -> Db {
    match Db::load(db_name) {
        Err(e) => {
            log::error!("Error loading db [{}]: {}", db_name, e);
            std::process::exit(1);
        },
        Ok(db) => return db
    }
}

pub fn test() -> Result<(), Box<dyn Error>> {
    Ok(())
}