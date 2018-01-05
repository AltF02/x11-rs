// This software is available under the terms of the MIT license.

#![allow(non_snake_case)]

extern crate x11;
extern crate libx11;

use std::mem;
use std::ptr;

fn main () {
    unsafe {
        // Open connection to X server.
        let display = libx11::XOpenDisplay(ptr::null());

        if display.is_null() {
            panic!("XOpenDisplay failed");
        }

        // Use the default (and usually only) screen.
        let screen = libx11::XDefaultScreen(display);

        // Create a window.
        let attribs = x11::XSetWindowAttributes {
            background_pixel: libx11::XWhitePixel(display, screen),
            ..mem::uninitialized()
        };

        let window = libx11::XCreateWindow(display,
                                           libx11::XRootWindow(display, screen),
                                           0, 0,
                                           400, 300,
                                           0,
                                           0,               // CopyFromParent
                                           x11::InputOutput,
                                           ptr::null_mut(), // CopyFromParent,
                                           x11::CWBackPixel,
                                           &attribs);

        // Set the window title.
        libx11::XStoreName(display, window, "Hello X11\0".as_bytes().as_ptr() as *const _);

        // Listen for close window requests.
        let WM_PROTOCOLS = libx11::XInternAtom(display,
                                               "WM_PROTOCOLS\0".as_bytes().as_ptr() as *const _,
                                               x11::False);

        let WM_DELETE_WINDOW = libx11::XInternAtom(display,
                                               "WM_DELETE_WINDOW\0".as_bytes().as_ptr() as *const _,
                                               x11::False);

        libx11::XSetWMProtocols(display, window, &WM_DELETE_WINDOW, 1);

        // Show the window.
        libx11::XMapWindow(display, window);

        // Main loop.
        let mut event: x11::XEvent = mem::uninitialized();

        loop {
            libx11::XNextEvent(display, &mut event);

            match event.type_ {
                x11::ClientMessage => {
                    if event.xclient.window == window
                        && event.xclient.message_type == WM_PROTOCOLS
                        && event.xclient.format == 32
                    {
                        break;
                    }
                },

                _ => ()
            }
        }

        // Destroy the window.
        libx11::XDestroyWindow(display, window);

        // Shut down connection to X server.
        libx11::XCloseDisplay(display);
    }
}
