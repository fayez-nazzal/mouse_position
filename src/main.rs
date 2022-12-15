/// A simple crate to get the mouse position in a cross platform way.
/// It uses the winapi crate to get the mouse position on windows, x11-dl to get the mouse position on linux, and core-graphics to get the mouse position on macos.
/// Example Usage:
/// ```rust
/// use mouse_position::mouse_position::{Mouse, Position};
/// 
/// fn main() {
///     let position = Mouse::get_mouse_position();
///     match position {
///         Mouse::Position { x, y } => println!("x: {}, y: {}", x, y),
///         Mouse::Error => println!("Error getting mouse position"),
///    }
/// }
/// ```

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
