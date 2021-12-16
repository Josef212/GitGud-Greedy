#[derive(Debug)]
pub struct Transaction {
    _id: i32,
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
}