#[derive(Debug)]
pub enum Value {
    Int(i64),
    Float(f32),
    String(String),
    Bool(bool),
    Array(Vec<Value>),
    None,
}

pub struct Callback<T> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T> Callback<T> {
    fn call(&self) {

    }
}