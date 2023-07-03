use crate::types::Value;


pub struct ComponentProperty {
    pub name: String,
    pub value_id: u64,
}

pub trait Object {
    fn get_name(&self) -> String;

    fn get_properties(&self) -> Vec<String>;

    fn call_method(&self, method_name: &str, args: Value) {}

    fn call(&self, args: Value) {}

    fn on_construct(&self) {}

    fn on_destroy(&self) {}
}


