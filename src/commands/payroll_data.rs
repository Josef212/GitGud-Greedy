use clap::Parser;
use log;

use crate::commands::sub_cmd::{SubCmd, ask_parameter};
use crate::models::Db;
use crate::commons::Opts;
use crate::models::payroll::Payroll;

#[derive(Parser, Debug)]
pub struct PayrollData {
    #[clap(short, long)]
    year: Option<u32>,
    #[clap(short, long)]
    month: Option<u32>,
}

impl SubCmd for PayrollData {
    fn execute(&self, db: &Db, _opts: &Opts) {
        let payrolls = db.get_payroll_data(self.year, self.month).unwrap_or_else(|e| {
            log::error!("Error getting payrolls: {}", e);
            std::process::exit(0);
        });
        
        // TODO: Format results
        // TODO: Generate view model and view renderer
        
        log::info!("Payrolls =====================");
        for p in &payrolls {
            let company = db.get_company_str(p.company_id).unwrap_or(String::from("Unknown"));
            let category = db.get_category_str(p.category_id).unwrap_or(String::from("Unknown"));
            
            log::info!(
                "  [{:04}] Date: {} - Gross: {} - Net: {} - SS: {} - Irpf: {} - Company: {} - Category: {}",
                p._id, p.date, p.gross, p.net, p.ss, p.irpf, company, category
            );
        }
    }
}