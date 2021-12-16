use rusqlite::Row;

#[derive(Debug)]
pub struct Payroll {
    pub _id: i32,
    pub date: String,
    pub gross: f32,
    pub net: f32,
    pub ss: f32,
    pub irpf: f32,
    pub company_id: i32,
    pub category_id: i32,
}

impl Payroll {
    pub fn new(date: &String, gross: f32, net: f32, ss: f32, irpf: f32, company_id: i32, category_id: i32) -> Payroll {
        Payroll {
            _id: 0,
            date: date.clone(),
            gross,
            net,
            ss,
            irpf,
            company_id,
            category_id,
        }
    }
    
    pub fn from_row(r: &Row) -> Payroll {
        Payroll {
            _id: r.get_unwrap(0),
            date: r.get_unwrap(1),
            gross: r.get_unwrap(2),
            net: r.get_unwrap(3),
            ss: r.get_unwrap(4),
            irpf: r.get_unwrap(5),
            company_id: r.get_unwrap(6),
            category_id: r.get_unwrap(7)
        }
    }
}