// XInput2 example for x11-rs
//
// This is a basic example showing how to use XInput2 to read
// keyboard, mouse and other input events in X11.
//
// See Pete Hutterer's "XI2 Recipes" blog series,
// starting at http://who-t.blogspot.co.uk/2009/05/xi2-recipes-part-1.html
// for a guide.

extern crate x11;
extern crate libc;

use std::ffi::CString;
use std::mem::{transmute};
use std::slice::from_raw_parts;
use x11::{xlib, xinput, xinput2};
use libc::{c_uchar, c_int, c_uint};

mod demo_window;
use demo_window::DemoWindow;

const TITLE: &'static str = "XInput Demo";
const DEFAULT_WIDTH: c_uint = 640;
const DEFAULT_HEIGHT: c_uint = 480;

fn main () {
  unsafe {
    let mut demo_window = DemoWindow::new(TITLE, DEFAULT_WIDTH, DEFAULT_HEIGHT);

    // query XInput support
    let mut opcode: c_int = 0;
    let mut event: c_int = 0;
    let mut error: c_int = 0;
    let xinput_str = CString::new("XInputExtension").unwrap();
    if xlib::XQueryExtension(demo_window.display, xinput_str.as_ptr(), &mut opcode, &mut event, &mut error) == xlib::False {
        panic!("XInput not available")
    }
    
    let mut xinput_major_ver = xinput2::XI_2_Major;
    let mut xinput_minor_ver = xinput2::XI_2_Minor;
    if xinput::XIQueryVersion(demo_window.display, &mut xinput_major_ver, &mut xinput_minor_ver) != xlib::Success as c_int {
        panic!("XInput2 not available");
    }
    println!("XI version available {}.{}", xinput_major_ver, xinput_minor_ver);

    // init XInput events
    let mut mask: [c_uchar; 1] = [0];
    let mut input_event_mask = xinput::XIEventMask {
        deviceid: xinput2::XIAllDevices,
        mask_len: mask.len() as i32,
        mask: mask.as_mut_ptr()
    };
    xinput2::XISetMask(&mut mask, xinput2::XI_ButtonPress);
    xinput2::XISetMask(&mut mask, xinput2::XI_Motion);
    xinput2::XISetMask(&mut mask, xinput2::XI_KeyPress);

    match xinput::XISelectEvents(demo_window.display, demo_window.window, &mut input_event_mask, 1) {
        status if status as u8 == xlib::Success => (),
        err => panic!("Failed to select events {:?}", err)
    }

    // Show window
    demo_window.show();

    // Main loop
    let display = demo_window.display;
    demo_window.run_event_loop(|event| {
        match event.get_type() {
            xlib::GenericEvent => {
                let mut cookie: xlib::XGenericEventCookie = From::from(*event);
                if xlib::XGetEventData(display, &mut cookie) != xlib::True {
                    println!("Failed to retrieve event data");
                    return;
                }
                match cookie.evtype {
                    xinput2::XI_Motion => {
                        let event_data: &xinput2::XIDeviceEvent = transmute(cookie.data);
                        let axis_state = event_data.valuators;
                        let mask = from_raw_parts(axis_state.mask, axis_state.mask_len as usize);
                        let mut axis_count = 0;
                        for axis_id in 0..axis_state.mask_len {
                            if xinput2::XIMaskIsSet(&mask, axis_id) {
                                let axis_value = *axis_state.values.offset(axis_count);
                                println!("Touchpad motion. Axis {} value {}", axis_id, axis_value);
                                axis_count += 1;
                            }
                        }
                    },
                    _ => ()
                }
                xlib::XFreeEventData(display, &mut cookie);
            },
            _ => ()
        }
    });
  }
}
