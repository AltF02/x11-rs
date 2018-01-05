// This software is available under the terms of the MIT license.

extern crate x11;
extern crate libx11;

use std::ptr;

fn main () {
    unsafe {
        // Open connection to X server.
        let display = libx11::XOpenDisplay(ptr::null());

        if display.is_null() {
            panic!("XOpenDisplay failed");
        }

        // Shut down connection to X server.
        libx11::XCloseDisplay(display);
    }
}
