use clap::Parser;
use log;

use crate::commands::sub_cmd::SubCmd;
use crate::models::Db;
use crate::commons::Opts;
use crate::models::payroll::Payroll;

#[derive(Parser, Debug)]
pub struct AddPayroll {
    date: String,
    gross: f32,
    net: f32,
    ss: f32,
    irpf: f32,
    company: String,
    category: String,
}

impl SubCmd for AddPayroll {
    fn execute(&self, db: &Db, _opts: &Opts) {
        let date_int = 0; // TODO: Format date from str to int
        let company_id = 0;
        let category_id = 0;
        
        // TODO: Validate all arguments. Maybe things like values are positive and higher than 0
        
        let model = Payroll::new(date_int, self.gross, self.net, self.ss, self.irpf, company_id, category_id);
        db.insert_payroll(&model).unwrap_or_else(|e| {
            log::error!("Error inserting payroll: {}", e);
            std::process::exit(0);
        });
        
        log::info!("Payroll [{:?}] inserted successfully", model);
    }
}

#[derive(Parser, Debug)]
pub struct RepeatPayroll {
    date: String,
}

impl SubCmd for RepeatPayroll {
    fn execute(&self, _db: &Db, _opts: &Opts) {
        todo!()
    }
}