
//! Implementation for Windows.
use crate::mouse_position::Mouse;

#[cfg(target_os = "windows")]
impl Mouse {
    pub fn get_mouse_position() -> Mouse {
        use winapi::{
            um::winuser::GetCursorPos,
            shared::windef::POINT,
        };

        let mut point = POINT { x: 0, y: 0 };
        let result = unsafe { GetCursorPos(&mut point) };

        if result == 1 {
            return Mouse::Position { x: point.x, y: point.y }
        }

        Mouse::Error
    }

    pub fn set_dpi(monitor: usize) {
        use winapi::um::shellscalingapi::SetProcessDpiAwareness;
        use winit::event_loop::EventLoop;

        // forgive me
        let mut monitors = (*EventLoop::new()).available_monitors();
        let dpi = match monitors.nth(monitor) {
            Some(monitor) => monitor.scale_factor() * 100.0,
            None => panic!("Invalid monitor ID (counts from 0)")
        };

        unsafe { SetProcessDpiAwareness(dpi as u32); }
    }
}