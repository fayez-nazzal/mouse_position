use std::io;
use crate::cursor_position::Cursor;

#[cfg(target_os = "linux")]
impl Cursor {
    pub fn get_cursor_position() -> Cursor {
        use std::{process::{Command, Stdio}, io::{Read, BufReader}};

        let output = match Command::new("xdotool")
                .arg("getmouselocation")
                .arg("--shell")
                .stdout(Stdio::piped())
                .spawn() {
            Ok(it) => it,
            Err(_) => return Cursor::Error
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
    
        Cursor::Position { x, y }
    }
}