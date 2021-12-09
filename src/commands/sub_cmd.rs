use crate::models::Db;

pub trait SubCmd {
    fn execute(&self, db: &Db);
}