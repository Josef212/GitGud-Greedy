use std::collections::HashMap;
use clap::Parser;
use log;

use crate::commands::sub_cmd::SubCmd;
use crate::models::Db;
use crate::commons::Opts;

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
        
        let mut gross_total = 0.0;
        let mut net_total = 0.0;
        let mut ss_total = 0.0;
        let mut irpf_total = 0.0;
        let mut company_total = HashMap::new();
        let mut category_total = HashMap::new();
        
        log::info!("Payrolls =====================");
        for p in &payrolls {
            let company = db.get_company_str(p.company_id).unwrap_or(String::from("Unknown"));
            let category = db.get_category_str(p.category_id).unwrap_or(String::from("Unknown"));
            
            gross_total += p.gross;
            net_total += p.net;
            ss_total += p.ss;
            irpf_total += p.irpf;
            *company_total.entry(p.company_id).or_insert(0.0) += p.net;
            *category_total.entry(p.category_id).or_insert(0.0) += p.net;
            
            log::info!(
                "  [{:04}] Date: {} - Gross: {:.2} - Net: {:.2} - SS: {:.2} - Irpf: {:.2} - Company: {} - Category: {}",
                p._id, p.date, p.gross, p.net, p.ss, p.irpf, company, category
            );
        }
        
        let count = payrolls.len() as f32;
        let gross_avg = gross_total / count;
        let net_avg = net_total / count;
        let ss_avg = ss_total / count;
        let irpf_avg = irpf_total / count;

        log::info!("======================================");
        log::info!("Payrolls count: {}", payrolls.len() as usize);
        log::info!("Gross: {:.2} - Avg: {:.2}", gross_total, gross_avg);
        log::info!("Net:   {:.2} - Avg: {:.2}", net_total, net_avg);
        log::info!("SS:    {:.2} - Avg: {:.2}", ss_total, ss_avg);
        log::info!("Irpf:  {:.2} - Avg: {:.2}", irpf_total, irpf_avg);
        
        log::info!("Per company:");
        for c in company_total.keys() {
            let total = company_total[c];
            let name = db.get_company_str(*c).unwrap_or(String::from("Unknown"));
            let portion = total / net_total * 100.0;
            
            log::info!(
                "  [{:02}] {} - {:.2} - {:.2}%",
                *c, name, total, portion
            )
        }

        log::info!("Per category:");
        for c in category_total.keys() {
            let total = category_total[c];
            let name = db.get_category_str(*c).unwrap_or(String::from("Unknown"));
            let portion = total / net_total * 100.0;

            log::info!(
                "  [{:02}] {} - {:.2} - {:.2}%",
                *c, name, total, portion
            )
        }
    }
}