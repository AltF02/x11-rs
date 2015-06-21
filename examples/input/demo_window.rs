extern crate x11;
extern crate libc;

use std::ffi::CString;
use std::ptr::{
  null,
  null_mut,
};
use std::mem::{zeroed};
use libc::{c_uint};
use x11::{xlib};

/// Provides a basic framework for connecting to an X Display,
/// creating a window, displaying it and running the event loop
pub struct DemoWindow {
    pub display: *mut xlib::Display,
    pub window: xlib::Window,

    wm_protocols: xlib::Atom,
    wm_delete_window: xlib::Atom
}

impl DemoWindow {
    /// Create a new window with a given title and size
    pub fn new(title: &str, width: u32, height: u32) -> DemoWindow {
        unsafe {
            // Open display
            let display = xlib::XOpenDisplay(null());
            if display == null_mut() {
                panic!("can't open display");
            }

            // Load atoms
            let wm_delete_window_str = CString::new("WM_DELETE_WINDOW").unwrap();
            let wm_protocols_str = CString::new("WM_PROTOCOLS").unwrap();

            let wm_delete_window = xlib::XInternAtom(display, wm_delete_window_str.as_ptr(), xlib::False);
            let wm_protocols = xlib::XInternAtom(display, wm_protocols_str.as_ptr(), xlib::False);

            if wm_delete_window == 0 || wm_protocols == 0 {
                panic!("can't load atoms");
            }

            // Create window
            let screen_num = xlib::XDefaultScreen(display);
            let root = xlib::XRootWindow(display, screen_num);
            let white_pixel = xlib::XWhitePixel(display, screen_num);

            let mut attributes: xlib::XSetWindowAttributes = zeroed();
            attributes.background_pixel = white_pixel;

            let window = xlib::XCreateWindow(display, root, 0, 0, width as c_uint, height as c_uint, 0, 0,
                                             xlib::InputOutput as c_uint, null_mut(),
                                             xlib::CWBackPixel, &mut attributes);
            // Set window title
            let title_str = CString::new(title).unwrap();
            xlib::XStoreName(display, window, title_str.as_ptr() as *mut _);

            // Subscribe to delete (close) events
            let mut protocols = [wm_delete_window];

            if xlib::XSetWMProtocols(display, window, &mut protocols[0] as *mut xlib::Atom, 1) == xlib::False {
                panic!("can't set WM protocols");
            }

            DemoWindow{
                display: display,
                window: window,
                wm_protocols: wm_protocols,
                wm_delete_window: wm_delete_window
            }
        }
    }

    /// Display the window
    pub fn show(&mut self) {
        unsafe {
            xlib::XMapWindow(self.display, self.window);
        }
    }

    /// Process events for the window. Window close events are handled automatically,
    /// other events are passed on to |event_handler|
    pub fn run_event_loop<EventHandler>(&mut self, event_handler: EventHandler) 
    where EventHandler: Fn(&xlib::XEvent) {
        unsafe {
            let mut event: xlib::XEvent = zeroed();
            loop {
                xlib::XNextEvent(self.display, &mut event);
                match event.get_type() {
                    xlib::ClientMessage => {
                        let xclient: xlib::XClientMessageEvent = From::from(event);

                        // WM_PROTOCOLS client message
                        if xclient.message_type == self.wm_protocols && xclient.format == 32 {
                            let protocol = xclient.data.get_long(0) as xlib::Atom;

                            // WM_DELETE_WINDOW (close event)
                            if protocol == self.wm_delete_window {
                                break;
                            }
                        }
                    },
                    _ => event_handler(&event)
                }
            }
        }
    }
}

impl Drop for DemoWindow {
    /// Destroys the window and disconnects from the display
    fn drop(&mut self) {
        unsafe {
            xlib::XDestroyWindow(self.display, self.window);
            xlib::XCloseDisplay(self.display);
        }
    }
}

