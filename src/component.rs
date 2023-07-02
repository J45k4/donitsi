
pub struct ComponentProperty {
    pub name: String,
    pub value_id: u64,
}

pub trait Component {
    fn get_name(&self) -> String;

    fn get_properties(&self) -> Vec<ComponentProperty>;

    fn on_construct(&self) {}

    fn test() {}
}

pub enum FlexDir {
    Col,
    Row
}

impl Default for FlexDir {
    fn default() -> Self {
        FlexDir::Col
    }
}

pub enum BoxChild {
    Box(Box),
    Text(Text)
}

pub struct Box {
    pub dir: FlexDir,
    pub flex_grow: Option<f32>,
    pub children: Vec<BoxChild>
}

impl Component for Box {
    fn get_name(&self) -> String {
        "Box".to_string()
    }

    fn get_properties(&self) -> Vec<ComponentProperty> {
        todo!()
    }
}

pub struct Text {
    pub text: String,
}

impl Component for Text {
    fn get_name(&self) -> String {
        "Text".to_string()
    }

    fn get_properties(&self) -> Vec<ComponentProperty> {
        todo!()
    }

    fn test() {
        println!("Testing text");
    }
}
