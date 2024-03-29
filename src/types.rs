#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i64),
    Float(f64),
    Str(String),
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

pub enum Action {
    Construct{ id: usize },
    Destruct{ id: usize },
    LoadField{ id: usize, field: usize },
    StoreField{ id: usize, field: usize, val: Value },
    Call{ id: usize, args: Vec<Value> },
    Import{ path: String },
    Quit
}

#[derive(Debug, Clone, PartialEq)]
pub struct Const {
    pub id: usize,
    pub value: Value,
}