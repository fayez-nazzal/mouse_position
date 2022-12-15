//! Implementation for Linux.
#[cfg(target_os = "linux")]
use crate::mouse_position::Mouse;

#[cfg(target_os = "linux")]
impl Mouse {
    pub fn get_mouse_position() -> Mouse {
        use x11_dl::xlib;

        // Initialize Xlib and open a connection to the X server
        let xlib = xlib::Xlib::open().unwrap();
        let display = unsafe { (xlib.XOpenDisplay)(std::ptr::null()) };

        // Get the root window for the current screen
        let screen = unsafe { (xlib.XDefaultScreen)(display) };
        let root = unsafe { (xlib.XRootWindow)(display, screen) };

        // Get the pointer position
        let mut root_return: xlib::Window = 0;
        let mut child_return: xlib::Window = 0;
        let mut root_x_return: i32 = -1;
        let mut root_y_return: i32 = -1;
        let mut win_x_return: i32 = 0;
        let mut win_y_return: i32 = 0;
        let mut mask_return: u32 = 0;

        unsafe {
            (xlib.XQueryPointer)(
                display,
                root,
                &mut root_return,
                &mut child_return,
                &mut root_x_return,
                &mut root_y_return,
                &mut win_x_return,
                &mut win_y_return,
                &mut mask_return,
            );
        }

        // Close the connection to the X server
        unsafe {
            (xlib.XCloseDisplay)(display);
        }

        if root_x_return == -1 || root_y_return == -1 {
            return Mouse::Error;
        }

        Mouse::Position {
            x: root_x_return,
            y: root_y_return,
        }
    }
}
