
pub struct Position {
    pub x: i32,
    pub y: i32
}

pub enum Cursor {
    Position { x: i32, y: i32 },
    Error
}