use std::io;
use crate::mouse_position::Mouse;

#[cfg(target_os = "linux")]
impl Mouse {
    pub fn get_mouse_position() -> Mouse {
        use std::{process::{Command, Stdio}, io::{Read, BufReader}};

        let output = match Command::new("xdotool")
                .arg("getmouselocation")
                .arg("--shell")
                .stdout(Stdio::piped())
                .spawn() {
            Ok(it) => it,
            Err(_) => return Mouse::Error
        }
            .stdout
            .ok_or(io::Error::new(io::ErrorKind::Other, "Failed to capture xdotool output")).unwrap();
    
        let mut reader = BufReader::new(output);
        let mut buffer = String::new();
        reader.read_to_string(&mut buffer).unwrap();
    
        let mut x = 0;
        let mut y = 0;
    
        for line in buffer.lines() {
            let mut parts = line.split('=');
            let key = parts.next().unwrap();
            let value = parts.next().unwrap();
    
            if key == "X" {
                x = value.parse().unwrap();
            } else if key == "Y" {
                y = value.parse().unwrap();
            }
        }
    
        Mouse::Position { x, y }
    }
}