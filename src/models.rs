#[derive(Default, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Position {
    pub y: i32,
    pub x: i32,
}

pub struct Size {
    pub width: f32,
    pub height: f32,
}

impl Size {
    pub fn square(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}
