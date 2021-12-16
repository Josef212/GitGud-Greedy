use rusqlite::Row;

#[derive(Debug)]
pub struct Transaction {
    pub _id: i32,
    pub name: String,
    pub date: String,
    pub amount: f32,
    pub tag_id: i32,
}

impl Transaction {
    pub fn new(name: &String, date: &String, amount: f32, tag_id: i32) -> Transaction {
        Transaction {
            _id: 0, 
            name: name.clone(), 
            date: date.clone(), 
            amount, 
            tag_id
        }
    }

    pub fn from_row(r: &Row) -> Transaction {
        Transaction {
            _id: r.get_unwrap(0),
            name: r.get_unwrap(1),
            date: r.get_unwrap(2),
            amount: r.get_unwrap(3),
            tag_id: r.get_unwrap(4),
        }
    }
}