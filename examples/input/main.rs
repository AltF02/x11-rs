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

use std::ffi::{CString};
use std::mem::{transmute};
use std::slice::from_raw_parts;
use x11::{xlib, xinput2};
use libc::{c_uchar, c_int, c_uint};

mod demo_window;
use demo_window::DemoWindow;

const TITLE: &'static str = "XInput Demo";
const DEFAULT_WIDTH: c_uint = 640;
const DEFAULT_HEIGHT: c_uint = 480;

#[derive(Debug)]
enum AxisType {
    HorizontalScroll,
    VerticalScroll,
    Other
}

#[derive(Debug)]
struct Axis {
    id: i32,
    device_id: i32,
    axis_number: i32,
    axis_type: AxisType
}

#[derive(Debug)]
struct AxisValue {
    device_id: i32,
    axis_number: i32,
    value: f64
}

struct InputState {
    cursor_pos: (f64, f64),
    axis_values: Vec<AxisValue>
}

fn read_input_axis_info(display: *mut xlib::Display) -> Vec<Axis> {
    let mut axis_list = Vec::new();
    let mut device_count = 0;

    // only get events from the master devices which are 'attached'
    // to the keyboard or cursor
    let devices = unsafe{xinput2::XIQueryDevice(display, xinput2::XIAllMasterDevices, &mut device_count)};
    for i in 0..device_count {
        let device = unsafe { *(devices.offset(i as isize)) };
        for k in 0..device.num_classes {
            let class = unsafe { *(device.classes.offset(k as isize)) };
            match unsafe { (*class)._type } {
                xinput2::XIScrollClass => {
                    let scroll_class: &xinput2::XIScrollClassInfo = unsafe{transmute(class)};
                    axis_list.push(Axis{
                        id: scroll_class.sourceid,
                        device_id: device.deviceid,
                        axis_number: scroll_class.number,
                        axis_type: match scroll_class.scroll_type {
                            xinput2::XIScrollTypeHorizontal => AxisType::HorizontalScroll,
                            xinput2::XIScrollTypeVertical => AxisType::VerticalScroll,
                            _ => { unreachable!() }
                        }
                    })
                },
                xinput2::XIValuatorClass => {
                    let valuator_class: &xinput2::XIValuatorClassInfo = unsafe{transmute(class)};
                    axis_list.push(Axis{
                        id: valuator_class.sourceid,
                        device_id: device.deviceid,
                        axis_number: valuator_class.number,
                        axis_type: AxisType::Other
                    })
                },
                _ => { /* TODO */ }
            }
        }
    }

    axis_list.sort_by(|a, b| {
        if a.device_id != b.device_id {
            a.device_id.cmp(&b.device_id)
        } else if a.id != b.id {
            a.id.cmp(&b.id)
        } else {
            a.axis_number.cmp(&b.axis_number)
        }
    });
    axis_list
}

/// Given an input motion event for an axis and the previous
/// state of the axises, return the horizontal/vertical
/// scroll deltas
fn calc_scroll_deltas(event: &xinput2::XIDeviceEvent,
                     axis_id: i32,
                     axis_value: f64,
                     axis_list: &[Axis],
                     prev_axis_values: &mut Vec<AxisValue>) -> (f64, f64) {
    let prev_value_pos = prev_axis_values.iter().position(|prev_axis| {
        prev_axis.device_id == event.sourceid &&
            prev_axis.axis_number == axis_id
    });
    let delta = match prev_value_pos {
        Some(idx) => axis_value - prev_axis_values[idx].value,
        None => 0.0
    };

    let new_axis_value = AxisValue{
        device_id: event.sourceid,
        axis_number: axis_id,
        value: axis_value
    };

    match prev_value_pos {
        Some(idx) => prev_axis_values[idx] = new_axis_value,
        None => prev_axis_values.push(new_axis_value)
    }

    let mut scroll_delta = (0.0, 0.0);

    for axis in axis_list.iter() {
        if axis.id == event.sourceid &&
            axis.axis_number == axis_id {
                match axis.axis_type {
                    AxisType::HorizontalScroll => scroll_delta.0 = delta,
                    AxisType::VerticalScroll => scroll_delta.1 = delta,
                    _ => {}
                }
            }
    }

    scroll_delta
}

fn main () {
    let mut demo_window = DemoWindow::new(TITLE, DEFAULT_WIDTH, DEFAULT_HEIGHT);

    // query XInput support
    let mut opcode: c_int = 0;
    let mut event: c_int = 0;
    let mut error: c_int = 0;
    let xinput_str = CString::new("XInputExtension").unwrap();
    let xinput_available = unsafe {
        xlib::XQueryExtension(demo_window.display, xinput_str.as_ptr(), &mut opcode, &mut event, &mut error)
    };
    if xinput_available == xlib::False {
        panic!("XInput not available")
    }

    let mut xinput_major_ver = xinput2::XI_2_Major;
    let mut xinput_minor_ver = xinput2::XI_2_Minor;
    if unsafe{xinput2::XIQueryVersion(demo_window.display,
      &mut xinput_major_ver, &mut xinput_minor_ver)} != xlib::Success as c_int {
        panic!("XInput2 not available");
    }
    println!("XI version available {}.{}", xinput_major_ver, xinput_minor_ver);

    // init XInput events
    let mut mask: [c_uchar; 1] = [0];
    let mut input_event_mask = xinput2::XIEventMask {
        deviceid: xinput2::XIAllMasterDevices,
        mask_len: mask.len() as i32,
        mask: mask.as_mut_ptr()
    };
    let events = &[
        xinput2::XI_ButtonPress,
        xinput2::XI_ButtonRelease,
        xinput2::XI_KeyPress,
        xinput2::XI_KeyRelease,
        xinput2::XI_Motion
    ];
    for &event in events {
        xinput2::XISetMask(&mut mask, event);
    }

    match unsafe{xinput2::XISelectEvents(demo_window.display,
      demo_window.window, &mut input_event_mask, 1)} {
        status if status as u8 == xlib::Success => (),
        err => panic!("Failed to select events {:?}", err)
    }

    // Show window
    demo_window.show();

    // Main loop
    let display = demo_window.display;
    let axis_list = read_input_axis_info(display);

    let mut prev_state = InputState{
        cursor_pos: (0.0, 0.0),
        axis_values: Vec::new()
    };

    demo_window.run_event_loop(|event| {
        match event.get_type() {
            xlib::GenericEvent => {
                let mut cookie: xlib::XGenericEventCookie = From::from(*event);
                if unsafe{xlib::XGetEventData(display, &mut cookie)} != xlib::True {
                    println!("Failed to retrieve event data");
                    return;
                }
                match cookie.evtype {
                    xinput2::XI_KeyPress | xinput2::XI_KeyRelease => {
                        let event_data: &xinput2::XIDeviceEvent = unsafe{transmute(cookie.data)};
                        if cookie.evtype == xinput2::XI_KeyPress {
                            if event_data.flags & xinput2::XIKeyRepeat == 0 {
                                println!("Key {} pressed", event_data.detail);
                            }
                        } else {
                            println!("Key {} released", event_data.detail);
                        }
                    },
                    xinput2::XI_ButtonPress | xinput2::XI_ButtonRelease => {
                        let event_data: &xinput2::XIDeviceEvent = unsafe{transmute(cookie.data)};
                        if cookie.evtype == xinput2::XI_ButtonPress {
                            println!("Button {} pressed", event_data.detail);
                        } else {
                            println!("Button {} released", event_data.detail);
                        }
                    },
                    xinput2::XI_Motion => {
                        let event_data: &xinput2::XIDeviceEvent = unsafe{transmute(cookie.data)};
                        let axis_state = event_data.valuators;
                        let mask = unsafe{ from_raw_parts(axis_state.mask, axis_state.mask_len as usize) };
                        let mut axis_count = 0;

                        let mut scroll_delta = (0.0, 0.0);
                        for axis_id in 0..axis_state.mask_len {
                            if xinput2::XIMaskIsSet(&mask, axis_id) {
                                let axis_value = unsafe{*axis_state.values.offset(axis_count)};
                                let delta = calc_scroll_deltas(event_data, axis_id, axis_value, &axis_list, &mut prev_state.axis_values);
                                scroll_delta.0 += delta.0;
                                scroll_delta.1 += delta.1;
                                axis_count += 1;
                            }
                        }

                        if scroll_delta.0.abs() > 0.0 || scroll_delta.1.abs() > 0.0 {
                            println!("Mouse wheel/trackpad scrolled by ({}, {})", scroll_delta.0, scroll_delta.1);
                        }

                        let new_cursor_pos = (event_data.event_x, event_data.event_y);
                        if new_cursor_pos != prev_state.cursor_pos {
                            println!("Mouse moved to ({}, {})", new_cursor_pos.0, new_cursor_pos.1);
                            prev_state.cursor_pos = new_cursor_pos;
                        }
                    },
                    _ => ()
                }
                unsafe{xlib::XFreeEventData(display, &mut cookie)};
            },
            _ => ()
        }
    });
}
