#[derive(Debug)]
pub struct Payroll {
    id: i32,
    pub date: String,
    pub gross: f32,
    pub net: f32,
    pub ss: f32,
    pub irpf: f32,
    pub company: i32,
    pub category: String,
}