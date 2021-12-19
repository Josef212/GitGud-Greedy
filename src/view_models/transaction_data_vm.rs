use std::collections::HashMap;

use crate::Db;
use crate::models::transaction::Transaction;
use comfy_table::{Table, Row, ContentArrangement, Cell, Attribute, Color};
use comfy_table::presets::UTF8_FULL;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;

struct TagInfo {
    id: i32,
    count: usize,
    amount: f32,
}

impl TagInfo {
    fn empty(id: i32) -> Self { 
        Self { id, count: 0, amount: 0.0 } 
    }
    
    fn add_count(&mut self, amount: f32) {
        self.count += 1;
        self.amount += amount;
    }
    
    fn avg(&self) -> f32 {
        self.amount / (self.count as f32)
    }
}

pub struct TransactionDataVm<'a> {
    transactions: &'a Vec<Transaction>,
    total_amount: f32,
    amount_avg: f32,
    tags_info: HashMap<i32, TagInfo>,
}

impl<'a> TransactionDataVm<'a> {
    pub fn generate(from: &'a Vec<Transaction>) -> Self {
        let mut total_amount = 0.0;
        let mut tags_info: HashMap<i32, TagInfo> = HashMap::new();
        
        for t in from {
            total_amount += t.amount;
            tags_info.entry(t.tag_id).or_insert(TagInfo::empty(t.tag_id));
            tags_info.get_mut(&t.tag_id).unwrap().add_count(t.amount);
        }

        TransactionDataVm {
            transactions: from,
            total_amount,
            amount_avg: (total_amount / (from.len() as f32)),
            tags_info,
        }
    }
    
    pub fn render(&self, db: &Db) {
        self.recap();
        self.tags(db);
        self.transactions(db);
    }
    
    fn recap(&self) {
        let mut table = TransactionDataVm::create_table(vec!["Total", "Tags count", "Avg. price"]);
        table.add_row(vec![
            Cell::new(format!("{:.2}", self.total_amount)),
            Cell::new(format!("{}", self.tags_info.len())),
            Cell::new(format!("{:.2}", self.amount_avg)),
        ]);
        
        log::info!("Summary:\n{}", table);
    }
    
    fn tags(&self, db: &Db) {
        let mut table = TransactionDataVm::create_table(vec!["Id", "Tag", "Total", "Count", "Avg."]);
        for (_, info) in &self.tags_info {
            let tag = db.get_tag_str(info.id).unwrap_or(String::from("Unknown"));

            table.add_row(vec![
                Cell::new(format!("{:02}", info.id)),
                Cell::new(format!("{}", tag)),
                Cell::new(format!("{:.2}", info.amount)),
                Cell::new(format!("{:}", info.count)),
                Cell::new(format!("{:.2}", info.avg())),
            ]);
        }
        
        log::info!("Per tags data:\n{}", table);
    }

    fn transactions(&self, db: &Db) {
        let mut table = TransactionDataVm::create_table(vec!["Id", "Name", "Date", "Amount", "Tag"]);

        for t in self.transactions {
            let tag = db.get_tag_str(t.tag_id).unwrap_or(String::from("Unknown"));
            table.add_row(vec![
                Cell::new(t._id),
                Cell::new(&t.name),
                Cell::new(&t.date),
                Cell::new(t.amount),
                Cell::new(&tag),
            ]);
        }

        log::info!("Transactions:\n{}", table);
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