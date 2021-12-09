#[derive(Debug)]
pub struct Transaction {
    id: i32,
    pub name: String,
    pub date: String,
    pub amount: f32,
    pub tag_id: i32,
}

impl Transaction {
    // pub fn new() -> Transaction {
    //     let db = Db::load();
    //     
    //     Transaction {
    //         id: 0,
    //         name: String::from(""),
    //         date: String::from(""),
    //         amount: 0.0,
    //         tag: String::from("")
    //     }
    // }
}