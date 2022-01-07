use crate::Db;

use crate::models::transaction::Transaction;
use crate::models::payroll::Payroll;

use crate::view_models::payroll_data_vm::PayrollDataVm;
use crate::view_models::transaction_data_vm::TransactionDataVm;

use comfy_table::{Table, Row, ContentArrangement, Cell, Attribute, Color};
use comfy_table::presets::UTF8_FULL;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;

pub struct BalanceSummaryVm<'a> {
    payroll: PayrollDataVm<'a>,
    transactions: TransactionDataVm<'a>,
}

impl<'a> BalanceSummaryVm<'a> {
    pub fn generate(payrolls: &'a Vec<Payroll>, transactions: &'a Vec<Transaction>) -> Self {
        Self {
            payroll: PayrollDataVm::generate(payrolls),
            transactions: TransactionDataVm::generate(transactions),
        }
    }
    
    pub fn render(&self, db: &Db) {
        self.payroll.render(db);
        self.transactions.render(db);
        
        let header = vec![
            Cell::new("Income").fg(Color::Green),
            Cell::new("Expenses").fg(Color::Red),
            Cell::new("Total").add_attribute(Attribute::Bold),
        ];
        let mut table = Table::new();
        table
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_ROUND_CORNERS)
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_header(Row::from(header));
        
        let income = self.payroll.get_net();
        let expenses = self.transactions.total();
        let total = income - expenses;
        
        table.add_row(vec![
            Cell::new(income),
            Cell::new(expenses),
            Cell::new(total),
        ]);
        
        log::info!("Summary:\n{}", table);
    }
}
