//! A a simple crate to get the mouse position in a cross platform way.
//! It uses winapi crate to get the mouse position on windows, x11-dl for linux, and core-graphics for macos.
//! Example Usage:
//! ```rust
//! use mouse_position::mouse_position::{Mouse};
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
