
#[derive(Debug)]
pub struct Transaction {
    id: i32,
    name: String,
    date: String,
    amount: f32,
    tag: String,
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