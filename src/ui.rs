use crate::component::Object;
use crate::types::Callback;

pub enum FlexDir {
    Col,
    Row
}

impl Default for FlexDir {
    fn default() -> Self {
        FlexDir::Col
    }
}

pub enum DivChild {
    Div(Div),
    Text(Text)
}

pub struct Div {
    pub dir: FlexDir,
    pub flex_grow: Option<f32>,
    pub width: Option<f32>,
    pub height: Option<f32>,
    pub children: Vec<DivChild>,
    pub vscroll: bool,
    pub hscroll: bool,
}

impl Object for Div {
    fn get_name(&self) -> String {
        "Box".to_string()
    }

    fn get_properties(&self) -> Vec<String> {
        todo!()
    }
}


pub struct Text {
    pub text: String,
}

impl Object for Text {
    fn get_name(&self) -> String {
        "Text".to_string()
    }

    fn get_properties(&self) -> Vec<String> {
        todo!()
    }
}

pub struct Clickable {
    pub on_click: Callback<()>
}

pub struct Keyboard {
    pub on_key_down: Callback<()>,
    pub on_key_up: Callback<()>,
}