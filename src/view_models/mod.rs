pub mod transaction_data_vm;

pub trait ViewModel<T, B> {
    fn generate(from: B) -> T;
    fn render(&self);
}