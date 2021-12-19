use std::cmp::max;
use std::collections::HashMap;

use crate::Db;
use crate::models::payroll::Payroll;
use comfy_table::{Table, Row, ContentArrangement, Cell, Attribute, Color};
use comfy_table::presets::UTF8_FULL;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;

struct PairInfo {
    id: i32,
    count: usize,
    amount: f32,
}

impl PairInfo {
    fn empty(id: i32) -> Self {
        Self { id, count: 0, amount: 0.0 }
    }
    
    fn add_count(&mut self, amout: f32) {
        self.count += 1;
        self.amount += amout;
    }
    
    fn avg(&self) -> f32 {
        self.amount/ (self.count as f32)
    }
}

pub struct PayrollDataVm<'a> {
    payrolls: &'a Vec<Payroll>,
    gross_total: f32,
    net_total: f32,
    ss_total: f32,
    irpf_total: f32,
    companies_info: HashMap<i32, PairInfo>,
    categories_info: HashMap<i32, PairInfo>,
}

impl<'a> PayrollDataVm<'a> {
    pub fn generate(from: &'a Vec<Payroll>) -> Self {
        let mut gross_total = 0.0;
        let mut net_total = 0.0;
        let mut ss_total = 0.0;
        let mut irpf_total = 0.0;
        let mut company_total = HashMap::new();
        let mut category_total = HashMap::new();
        
        for p in from {
            gross_total += p.gross;
            net_total += p.net;
            ss_total += p.ss;
            irpf_total += p.irpf;
            
            company_total.entry(p.company_id).or_insert(PairInfo::empty(p.company_id));
            company_total.get_mut(&p.company_id).unwrap().add_count(p.gross);
            
            category_total.entry(p.category_id).or_insert(PairInfo::empty(p.category_id));
            category_total.get_mut(&p.category_id).unwrap().add_count(p.gross);
        }
        
        Self {
            payrolls: from,
            gross_total,
            net_total,
            ss_total,
            irpf_total,
            companies_info: company_total,
            categories_info: category_total,
        }
    }
    
    pub fn render(&self, db: &Db) {
        self.recap();
        self.payrolls(db);
        self.foo(db);
    }
    
    fn recap(&self) {
        let count_str = self.payrolls.len().to_string();
        let mut table = PayrollDataVm::create_table(vec![&count_str, "Gross", "Net", "SS", "Irpf"]);
        
        table.add_row(vec![
            Cell::new("Total").add_attribute(Attribute::Bold),
            Cell::new(format!("{}", self.gross_total)),
            Cell::new(format!("{}", self.net_total)),
            Cell::new(format!("{}", self.ss_total)),
            Cell::new(format!("{}", self.irpf_total)),
        ]);
        
        table.add_row(vec![
            Cell::new("Avg.").add_attribute(Attribute::Bold),
            Cell::new(format!("{}", self.gross_avg())),
            Cell::new(format!("{}", self.net_avg())),
            Cell::new(format!("{}", self.ss_avg())),
            Cell::new(format!("{}", self.irpf_avg())),
        ]);
        
        log::info!("Summary:\n{}", table);
    }

    fn payrolls(&self, db: &Db) {
        let mut table = PayrollDataVm::create_table(vec![
            "Id", "Date", "Gross", "Net", "SS", "Irpf", "Company", "Category"
        ]);
        
        for p in self.payrolls {
            let company = db.get_company_str(p.company_id).unwrap_or(String::from("Unknown"));
            let category = db.get_category_str(p.category_id).unwrap_or(String::from("Unknown"));

            table.add_row(vec![
                Cell::new(p._id),
                Cell::new(&p.date),
                Cell::new(p.gross),
                Cell::new(p.net),
                Cell::new(p.ss),
                Cell::new(p.irpf),
                Cell::new(&company),
                Cell::new(&category),
            ]);
        }
        
        log::info!("Payrolls:\n{}", table);
    }
    
    fn foo(&self, db: &Db) {
        let mut table = PayrollDataVm::create_table(vec![
            "Id", "Name", "Gross", "Avg.", "Count",
            "",
            "Id", "Name", "Gross", "Avg.", "Count",
        ]);
        
        let company_keys: Vec<i32> = self.companies_info.keys().cloned().collect();
        let category_keys: Vec<i32> = self.categories_info.keys().cloned().collect();
        let count = max(self.companies_info.len(), self.categories_info.len());
        
        for i in 0..count {
            let company_info = self.companies_info.get(&company_keys[i]).unwrap();
            let category_info = self.companies_info.get(&category_keys[i]).unwrap();
            
            let mut row = Row::new();
            self.insert_company(&mut row, company_info, db);
            row.add_cell(Cell::new(""));
            self.insert_category(&mut row, category_info, db);
            
            table.add_row(row);
        }

        log::info!("Summarized data:\n{}", table);
    }
    
    fn insert_company(&self, row: &mut Row, info: &PairInfo, db: &Db) {
        let name = db.get_company_str(info.id).unwrap_or(String::from("Unknown"));
        row.add_cell(Cell::new(format!("{:02}", info.id)));
        row.add_cell(Cell::new(format!("{}", name)));
        row.add_cell(Cell::new(format!("{:.2}", info.amount)));
        row.add_cell(Cell::new(format!("{:.2}", info.avg())));
        row.add_cell(Cell::new(format!("{}", info.count)));
    }

    fn insert_category(&self, row: &mut Row, info: &PairInfo, db: &Db) {
        let name = db.get_category_str(info.id).unwrap_or(String::from("Unknown"));
        row.add_cell(Cell::new(format!("{:02}", info.id)));
        row.add_cell(Cell::new(format!("{}", name)));
        row.add_cell(Cell::new(format!("{:.2}", info.amount)));
        row.add_cell(Cell::new(format!("{:.2}", info.avg())));
        row.add_cell(Cell::new(format!("{}", info.count)));
    }

    fn gross_avg(&self) -> f32 {
        self.gross_total / (self.payrolls.len() as f32)
    }
    
    fn net_avg(&self) -> f32 {
        self.net_total / (self.payrolls.len() as f32)
    }

    fn ss_avg(&self) -> f32 {
        self.ss_total / (self.payrolls.len() as f32)
    }

    fn irpf_avg(&self) -> f32 {
        self.irpf_total / (self.payrolls.len() as f32)
    }
    
    // TODO: Abstract this
    fn create_table(header: Vec<&str>) -> Table {
        let mut table = Table::new();
        let cells: Vec<Cell> = header.iter().map(|h| { Cell::new(h).add_attribute(Attribute::Bold).fg(Color::Green) }).collect();
        let header = Row::from(cells);
        table
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_ROUND_CORNERS)
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_header(header);

        table
    }
}