//! This is a simple crate to get the mouse position in a cross platform way.
//! It uses the winapi crate to get the mouse position on windows, x11-dl to get the mouse position on linux, and core-graphics to get the mouse position on macos.
//! Example Usage:
//! ```rust
//! use mouse_position::mouse_position::{Mouse, Position};
//! 
//! fn main() {
//!     let position = Mouse::get_mouse_position();
//!     match position {
//!         Mouse::Position { x, y } => println!("x: {}, y: {}", x, y),
//!         Mouse::Error => println!("Error getting mouse position"),
//!    }
//! }
//! ```

pub mod impl_linux;
pub mod impl_windows;
pub mod impl_macos;
pub mod mouse_position;
