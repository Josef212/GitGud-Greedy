use std::collections::HashMap;
use crate::Db;
use crate::models::transaction::Transaction;
use super::ViewModel;

struct TagInfo {
    id: i32,
    count: usize,
    amount: f32,
}

impl TagInfo {
    fn empty(id: i32) -> Self { Self { id, count: 0, amount: 0.0 } }
    
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
        log::info!("Transactions =====================");
        for t in self.transactions {
            let tag = db.get_tag_str(t.tag_id).unwrap_or(String::from("Unknown"));

            log::info!(
                "  [{:04}] Name: {} - Date: {} - Amount: {:.2} - Tag: {}",
                t._id, t.name, t.date, t.amount, tag
            );
        }

        log::info!("======================================");
        log::info!("Total spent: {:.2}", self.total_amount);
        log::info!("Total tags: {}", self.tags_info.len());
        log::info!("Avg per tag: {:.2}", self.amount_avg);

        for (_, info) in &self.tags_info {
            let tag = db.get_tag_str(info.id).unwrap_or(String::from("Unknown"));
            log::info!(
                "  [{:02}] {} - A: {:.2} - C: {} - Avg: {:.2}",
                info.id, tag, info.amount, info.count, info.avg()
            );
        }
    }
}

// impl ViewModel<TransactionDataVm, &'_ Vec<Transaction>> for TransactionDataVm