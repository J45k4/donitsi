
pub struct Tringle {
    pub top: f32,
    pub left: f32,
    pub right: f32,
}

pub enum Child {
    Tringle(Tringle),
    Container(Container),
}

pub struct Container {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub width: f32,
    pub height: f32,
    pub depth: f32,
    pub children: Vec<Child>,
    pub texture: Option<String>
}

pub struct Text {
    pub text: String,
}

#[derive(Clone, Debug)]
pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub color: Option<String>,
}

#[derive(Clone, Debug)]
pub struct Shape {
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
}


pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

pub struct Tensor {

}

impl Tensor {
    pub fn concat(tensors: &[Tensor]) -> Tensor {
        Tensor {}
    }
}



pub struct Light {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub intensity: f32,
}

pub struct Translation {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub struct Rotation {

}

impl Rotation {
    pub fn xrot(x: f32) -> Rotation {
        Rotation {}
    }
}