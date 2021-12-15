use clap::Parser;
use log;

use crate::commands::sub_cmd::{SubCmd, ask_parameter};
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
    category_id: i32,
}

impl SubCmd for AddPayroll {
    fn execute(&self, db: &Db, _opts: &Opts) {
        let date_int = Db::code_date(&self.date).unwrap_or_else(|e| {
            log::error!("Could not parse date {}. Error: {}", self.date, e);
            std::process::exit(0);
        });
        
        let company_id = db.get_company_id(&self.company).unwrap_or_else(|e| {
            log::error!("Could not find id for company {}. Error: {}", self.company, e);
            std::process::exit(0);
        });
        
        // TODO: Validate all arguments. Maybe things like values are positive and higher than 0
        
        let model = Payroll::new(date_int, self.gross, self.net, self.ss, self.irpf, company_id, self.category_id);
        db.insert_payroll(&model).unwrap_or_else(|e| {
            log::error!("Error inserting payroll: {}", e);
            std::process::exit(0);
        });
        
        log::info!("Payroll [{:?}] inserted successfully", model);
    }
}

#[derive(Parser, Debug)]
pub struct AddPayrollP;

impl SubCmd for AddPayrollP {
    fn execute(&self, db: &Db, _opts: &Opts) {
        let date = ask_parameter::<String>("date");
        let date = Db::code_date(&date).unwrap_or_else(|e| {
            log::error!("Could not parse date {}. Error: {}", date, e);
            std::process::exit(0);
        });
        
        let gross = ask_parameter::<f32>("gross");
        let net = ask_parameter::<f32>("net");
        let ss = ask_parameter::<f32>("ss");
        let irpf = ask_parameter::<f32>("irpf");
        let company = ask_parameter::<String>("company");
        let company = db.get_company_id(&company).unwrap_or_else(|e| {
            log::error!("Could not find company id for company {}. Error: {}", company, e);
            std::process::exit(0);
        });
        
        let category_id = ask_parameter::<i32>("category_id");
        
        // TODO: Validate parameters
        
        let payroll = Payroll::new(date, gross, net, ss, irpf, company, category_id);
        db.insert_payroll(&payroll).unwrap_or_else(|e| {
            log::error!("Error inserting payroll: {}", e);
            std::process::exit(0);
        });
        
        log::info!("Payroll [{:?}] inserted successfully", payroll);
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