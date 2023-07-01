use crate::component::Component;
use crate::component::ComponentProperty;


pub struct Main {

}

impl Component for Main {
    fn get_name(&self) -> String {
        "Main".to_string()
    }

    fn get_properties(&self) -> Vec<ComponentProperty> {
        todo!()
    }
}