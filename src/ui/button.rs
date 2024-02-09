#[derive(Clone)]
pub struct Button {
    pub id: String,
    pub text: String,
    pub width: f32,
    pub pos_x: f32,
    pub pos_y: f32,
    pub height: f32,
    pub hover: bool,
    pub pressed: bool,
}

impl Button {
    pub fn new(id: String, text: String, pos_x: f32, pos_y: f32, width: f32, height: f32) -> Self {
        Self {
            id,
            text,
            pos_x,
            pos_y,
            width,
            height,
            hover: false,
            pressed: false,
        }
    }
}
