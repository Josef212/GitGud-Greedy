// https://github.com/clap-rs/clap
// https://docs.rs/clap/latest/clap/
// https://rust-lang-nursery.github.io/rust-cookbook/database/sqlite.html

mod commands;
mod models;

use std::error::Error;
use std::io::Write;
use std::str::FromStr;
use clap::Parser;
use chrono::Local;
use env_logger::fmt::Color;
use log::LevelFilter;

use crate::commands::{SubCommand, SubCmd};
use crate::models::Db;

pub struct Cli {
    pub opts: Opts,
}

#[derive(Parser, Debug)]
#[clap(version="1.0", author="Josef212")]
pub struct Opts {
    #[clap(short, long, default_value="default.conf")]
    config: String,
    #[clap(short, long, default_value="Debug")]
    log: String,
    #[clap(subcommand)]
    sub_cmd: Option<SubCommand>,
}

impl Cli {
    pub fn print_info(&self) {
        let opts = &self.opts;
        
        log::info!("Value for config: {}", opts.config);
        log::info!("LogLevel: {}", log::max_level());
    }

    pub fn match_subcommand(&self) {
        match &self.opts.sub_cmd {
            Some(SubCommand::Test(t)) => t.execute(),
            _ => log::error!("No matching subcommand found. Use -h or --help to see the list.")
        }
    }
}

pub fn init() -> Cli {
    let opts: Opts = Opts::parse();
    init_logger(&opts.log);
    
    Cli { 
        opts 
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

pub fn test() -> Result<(), Box<dyn Error>> {
    let _db = Db::load()?;
    Ok(())
}