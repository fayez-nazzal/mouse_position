//! Contains the used structs and enums to get the mouse position.
//! 
// Position struct, which contains x and y coordinates.
pub struct Position {
    pub x: i32,
    pub y: i32
}

/// Mouse Position enum, which can either be a position or an error.
pub enum Mouse {
    Position { x: i32, y: i32 },
    Error
}