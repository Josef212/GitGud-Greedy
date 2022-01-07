use rusqlite::Row;

#[derive(Debug, Clone)]
pub struct Account {
    pub _id: i32,
    pub name: String,
    pub amount: f32,
    pub description: String,
}

impl Account {
    pub fn new(name: &str, amount: f32, description: &str) -> Self{
        Self {
            _id: 0,
            name: String::from(name),
            amount,
            description: String::from(description),
        }
    }
    
    pub fn from_row(r: &Row) -> Self {
        Self {
            _id: r.get_unwrap(0),
            name: r.get_unwrap(1),
            amount: r.get_unwrap(2),
            description: r.get_unwrap(3),
        }
    }
}