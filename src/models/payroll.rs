#[derive(Debug)]
pub struct Payroll {
    _id: i32,
    pub date: i64,
    pub gross: f32,
    pub net: f32,
    pub ss: f32,
    pub irpf: f32,
    pub company_id: i32,
    pub category_id: i32,
}

impl Payroll {
    pub fn new(date: i64, gross: f32, net: f32, ss: f32, irpf: f32, company_id: i32, category_id: i32) -> Payroll {
        Payroll {
            _id: 0,
            date,
            gross,
            net,
            ss,
            irpf,
            company_id,
            category_id,
        }
    }
}