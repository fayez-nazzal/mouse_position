use crate::cursor_position::{Cursor, Position};

mod impl_linux;
mod cursor_position;

fn main() {
    
    loop {
        let position = Cursor::get_cursor_position();
        
        let position  = match position {
            Cursor::Position { x, y } => Position { x, y },
            // if error, default to 0, 0
            Cursor::Error => Position { x: 0, y: 0 }
        };
    
        println!("x: {}, y: {}", position.x, position.y);

        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
