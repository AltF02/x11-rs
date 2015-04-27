// hello_world example for x11-rs

extern crate libc;
extern crate x11;

use std::ffi::CString;
use std::mem::zeroed;
use std::ptr::{
  null,
  null_mut,
};

use libc::c_uint;
use x11::xlib;

const TITLE: &'static str = "Hello World!";
const DEFAULT_WIDTH: c_uint = 640;
const DEFAULT_HEIGHT: c_uint = 480;


fn main () {
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

    let window = xlib::XCreateWindow(display, root, 0, 0, DEFAULT_WIDTH, DEFAULT_HEIGHT, 0, 0,
                                     xlib::InputOutput as c_uint, null_mut(),
                                     xlib::CWBackPixel, &mut attributes);

    // Set window title
    let title_str = CString::new(TITLE).unwrap();
    xlib::XStoreName(display, window, title_str.as_ptr() as *mut _);

    // Subscribe to delete (close) events
    let mut protocols = [wm_delete_window];

    if xlib::XSetWMProtocols(display, window, &mut protocols[0] as *mut xlib::Atom, 1) == xlib::False {
      panic!("can't set WM protocols");
    }

    // Show window
    xlib::XMapWindow(display, window);

    // Main loop
    let mut event: xlib::XEvent = zeroed();

    loop {
      xlib::XNextEvent(display, &mut event);
      match event.get_type() {
        xlib::ClientMessage => {
          let xclient: xlib::XClientMessageEvent = From::from(event);

          // WM_PROTOCOLS client message
          if xclient.message_type == wm_protocols && xclient.format == 32 {
            let protocol = xclient.data.get_long(0) as xlib::Atom;

            // WM_DELETE_WINDOW (close event)
            if protocol == wm_delete_window {
              break;
            }
          }
        },

        _ => {},
      }
    }

    // Clean up
    xlib::XDestroyWindow(display, window);
    xlib::XCloseDisplay(display);
  }
}
