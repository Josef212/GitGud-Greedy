use clap::Parser;

use crate::commands::SubCommand;

#[derive(Parser, Debug)]
#[clap(version="1.0", author="Josef212")]
pub struct Opts {
    #[clap(short, long, default_value="default.conf")]
    config: String,
    #[clap(short, long, default_value="gg_financials.db")]
    db_name: String,
    #[clap(short, long, default_value="Debug")]
    log: String,
    #[clap(subcommand)]
    sub_cmd: Option<SubCommand>,
}

impl Opts {
    pub fn new() -> Opts {
        Opts::parse()
    }
    
    pub fn get_db_name(&self) -> &String {
        &self.db_name
    }
    
    pub fn get_log(&self) -> &String {
        &self.log
    }

    pub fn get_sub_cmd(&self) -> &Option<SubCommand> {
        &self.sub_cmd
    }
}