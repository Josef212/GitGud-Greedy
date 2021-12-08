// https://github.com/clap-rs/clap
// https://docs.rs/clap/latest/clap/
// https://rust-lang-nursery.github.io/rust-cookbook/database/sqlite.html

mod commands;

use std::io::Write;
use std::str::FromStr;
use clap::Parser;
use chrono::Local;
use log::LevelFilter;

use crate::commands::{SubCommand, SubCmd};

pub struct Cli {
    pub opts: Opts,
}

#[derive(Parser)]
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
    
    env_logger::Builder::new()
        .format(|buf, record| {
            writeln!(buf,
                     "{} [{}] - {}",
                     Local::now().format("%Y-%m-%dT%H:%M:%S"),
                     record.level(),
                     record.args()
            )
        })
        .filter_level(LevelFilter::from_str(opts.log.as_str()).unwrap_or(LevelFilter::Error))
        .init();

    Cli { 
        opts 
    }
}