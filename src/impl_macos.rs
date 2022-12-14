
use crate::mouse_position::Mouse;

#[cfg(target_os = "macos")]
impl Mouse {
    pub fn get_mouse_position() -> Mouse {
        use core_graphics::event::{CGEvent};
        use core_graphics::event_source::{CGEventSource, CGEventSourceStateID};

        let event = CGEvent::new(CGEventSource::new(CGEventSourceStateID::CombinedSessionState).unwrap());
        let point = match event {
            Ok(event) => {
                let point = event.location();
                Mouse::Position { x: point.x as i32, y: point.y as i32 }
            },
            Err(_) => return Mouse::Error,
        };
        
        point
    }
}