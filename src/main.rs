use mouse_position::mouse_position::{Mouse, Position};

fn main() {

    loop {
        let position = Mouse::get_mouse_position();

        let position  = match position {
            Mouse::Position { x, y } => Position { x, y },
            // if error, default to 0, 0
            Mouse::Error => Position { x: 0, y: 0 }
        };
    
        println!("x: {}, y: {}", position.x, position.y);

        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
