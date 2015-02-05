// Copyright (c) 2015, <daggerbot@gmail.com>
// All rights reserved.

#![crate_name="example"]
#![crate_type="bin"]

extern crate xlib;

use std::mem::zeroed;
use std::ptr::{
  null,
  null_mut,
};

use xlib::{
  Atom,
  ClientMessage,
  XBlackPixel,
  XCloseDisplay,
  XCreateSimpleWindow,
  XDefaultScreen,
  XDestroyWindow,
  XEvent,
  XInternAtom,
  XMapWindow,
  XNextEvent,
  XOpenDisplay,
  XRootWindow,
  XSetWMProtocols,
  XStoreName,
  XWhitePixel,
};


//
// main
//


fn main () {
  unsafe {
    // open connection to display server
    let display = XOpenDisplay(null());
    if display == null_mut() {
      panic!("can't connect to the display server");
    }

    // create window
    let screen = XDefaultScreen(display);
    let root = XRootWindow(display, screen);
    let border = XBlackPixel(display, screen);
    let background = XWhitePixel(display, screen);
    let window = XCreateSimpleWindow(display, root, 0, 0, 640, 480, 0, border, background);
    XStoreName(display, window, "Hello World!\0".as_ptr() as *const i8);
    XMapWindow(display, window);

    // listen for close requests
    let protocols = XInternAtom(display, "WM_PROTOCOLS\0".as_ptr() as *const i8, 0);
    let close_message = XInternAtom(display, "WM_DELETE_WINDOW\0".as_ptr() as *const i8, 0);
    XSetWMProtocols(display, window, &close_message, 1);

    // main loop
    let mut event: XEvent = zeroed();
    loop {
      XNextEvent(display, &mut event);
      if event.kind() == ClientMessage {
        let xclient = event.xclient();
        if xclient.message_type == protocols && xclient.format == 32 {
          if xclient.data.get_long(0) as Atom == close_message {
            break;
          }
        }
      }
    }

    // clean up
    XDestroyWindow(display, window);
    XCloseDisplay(display);
  }
}
