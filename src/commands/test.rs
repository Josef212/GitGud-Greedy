use clap::Parser;
use crate::commands::sub_cmd::SubCmd;

#[derive(Parser)]
pub struct Test {
    #[clap(short)]
    pub debug: bool,
}

impl SubCmd for Test {
    fn execute(&self) {
        println!("Test debug info: {}", self.debug);
    }
}