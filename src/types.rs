#[derive(Debug)]
pub enum Value {
    Int(i32),
    Float(f32),
    String(String),
    Bool(bool),
    Array(Vec<Value>),
    None,
}

pub struct Callback<T> {

}

impl<T> Callback<T> {
    fn call(&self) {

    }
}