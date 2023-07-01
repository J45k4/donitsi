
pub struct ComponentProperty {
    pub name: String,
    pub value_id: u64,
}

pub trait Component {
    fn get_name(&self) -> String;

    fn get_properties(&self) -> Vec<ComponentProperty>;

    fn on_construct(&self) {}
}