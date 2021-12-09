use crate::models::Db;
use crate::commons::Opts;

pub trait SubCmd {
    fn execute(&self, db: &Db, opts: &Opts);
}