// https://github.com/clap-rs/clap
// https://docs.rs/clap/latest/clap/
// https://rust-lang-nursery.github.io/rust-cookbook/database/sqlite.html

mod commands;

use clap::Parser;
use crate::commands::{SubCommand, SubCmd};

pub struct Cli {
    pub opts: Opts,
}

#[derive(Parser)]
#[clap(version="1.0", author="Josef212")]
pub struct Opts {
    #[clap(short, long, default_value="default.conf")]
    config: String,
    #[clap(default_value="input.txt")]
    input: String,
    #[clap(short, long)]
    verbose: bool,
    #[clap(subcommand)]
    subcmd: Option<SubCommand>,
}

impl Cli {
    pub fn new() -> Cli {
        Cli { opts: Opts::parse() }
    }
    
    pub fn print(&self) {
        let opts = &self.opts;
        if opts.verbose {
            println!("Value for config: {}", opts.config);
            println!("Using input file: {}", opts.input);
            println!("Verbose: {}", opts.verbose);
        }
    }
    
    pub fn match_subcommand(&self) {
        let opts = &self.opts;
        match &opts.subcmd {
            Some(SubCommand::Test(t)) => {
                t.execute();
            }
            _ => {}
        }
    }
}
