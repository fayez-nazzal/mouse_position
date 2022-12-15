
//! Implementation for Windows.
use crate::mouse_position::Mouse;

#[cfg(target_os = "windows")]
impl Mouse {
    pub fn get_mouse_position() -> Mouse {
        use winapi::{um::winuser::{GetCursorPos}, shared::windef::POINT};

        let mut point = POINT { x: 0, y: 0 };
        let result = unsafe { GetCursorPos(&mut point) };

        if result == 1 {
            return Mouse::Position { x: point.x, y: point.y }
        }

        Mouse::Error
    }
}