use clap::Parser;
use log;

use crate::commands::sub_cmd::SubCmd;
use crate::models::Db;
use crate::commons::Opts;
use crate::view_models::payroll_data_vm::PayrollDataVm;

#[derive(Parser, Debug)]
pub struct PayrollData {
    #[clap(short, long)]
    year: Option<u32>,
    #[clap(short, long)]
    month: Option<u32>,
    #[clap(short, long)]
    list: bool,
}

impl SubCmd for PayrollData {
    fn execute(&self, db: &Db, _opts: &Opts) {
        let payrolls = db.get_payroll_data(self.year, self.month).unwrap_or_else(|e| {
            log::error!("Error getting payrolls: {}", e);
            std::process::exit(0);
        });
        
        PayrollDataVm::generate(&payrolls).render(self.list, db);
    }
}