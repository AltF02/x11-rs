// This software is available under the terms of the MIT license.

use std::os::raw::*;

use xproto::*;

pub type Bool = c_int;
pub type Status = c_int;
pub const True: Bool = 1;
pub const False: Bool = 0;

pub enum Display {}
pub enum Visual {}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct XSetWindowAttributes {
    pub background_pixmap: Pixmap,
    pub background_pixel: c_ulong,
    pub border_pixmap: Pixmap,
    pub border_pixel: c_ulong,
    pub bit_gravity: c_int,
    pub win_gravity: c_int,
    pub backing_store: c_int,
    pub backing_planes: c_ulong,
    pub backing_pixel: c_ulong,
    pub save_under: Bool,
    pub event_mask: c_long,
    pub do_not_propagate_mask: c_long,
    pub override_redirect: Bool,
    pub colormap: Colormap,
    pub cursor: Cursor,
}

//============================================================================//
// XEvent structures

#[derive(Clone, Copy)]
#[repr(C)]
pub union XEvent {
    pub type_: c_int,
    pub xany: XAnyEvent,
    pub xkey: XKeyEvent,
    pub xbutton: XButtonEvent,
    pub xmotion: XMotionEvent,
    pub xcrossing: XCrossingEvent,
    pub xfocus: XFocusChangeEvent,
    pub xexpose: XExposeEvent,
    pub xgraphicsexpose: XGraphicsExposeEvent,
    pub xnoexpose: XNoExposeEvent,
    pub xvisibility: XVisibilityEvent,
    pub xcreatewindow: XCreateWindowEvent,
    pub xdestroywindow: XDestroyWindowEvent,
    pub xunmap: XUnmapEvent,
    pub xmap: XMapEvent,
    pub xmaprequest: XMapRequestEvent,
    pub xreparent: XReparentEvent,
    pub xconfigure: XConfigureEvent,
    pub xgravity: XGravityEvent,
    pub xresizerequest: XResizeRequestEvent,
    pub xconfigurerequest: XConfigureRequestEvent,
    pub xcirculate: XCirculateEvent,
    pub xcirculaterequest: XCirculateRequestEvent,
    pub xproperty: XPropertyEvent,
    pub xselectionclear: XSelectionClearEvent,
    pub xselectionrequest: XSelectionRequestEvent,
    pub xselection: XSelectionEvent,
    pub xcolormap: XColormapEvent,
    pub xclient: XClientMessageEvent,
    pub xmapping: XMappingEvent,
    pub xerror: XErrorEvent,
    pub xkeymap: XKeymapEvent,
    pub xgeneric: XGenericEvent,
    pub xcookie: XGenericEventCookie,
    pub pad: [c_long; 24],
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct XAnyEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: Bool,
    pub display: *mut Display,
    pub window: Window,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct XKeyEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: Bool,
    pub display: *mut Display,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: c_int,
    pub y: c_int,
    pub x_root: c_int,
    pub y_root: c_int,
    pub state: c_uint,
    pub keycode: c_uint,
    pub same_screen: Bool,
}
pub type XKeyPressedEvent = XKeyEvent;
pub type XKeyReleasedEvent = XKeyEvent;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct XButtonEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: Bool,
    pub display: *mut Display,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: c_int,
    pub y: c_int,
    pub x_root: c_int,
    pub y_root: c_int,
    pub state: c_uint,
    pub button: c_uint,
    pub same_screen: Bool,
}
pub type XButtonPressedEvent = XButtonEvent;
pub type XButtonReleasedEvent = XButtonEvent;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct XMotionEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: Bool,
    pub display: *mut Display,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: c_int,
    pub y: c_int,
    pub x_root: c_int,
    pub y_root: c_int,
    pub state: c_uint,
    pub is_hint: c_char,
    pub same_screen: Bool,
}
pub type XPointerMovedEvent = XMotionEvent;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct XCrossingEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: Bool,
    pub display: *mut Display,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: c_int,
    pub y: c_int,
    pub x_root: c_int,
    pub y_root: c_int,
    pub mode: c_int,
    pub detail: c_int,
    pub same_screen: Bool,
    pub focus: Bool,
    pub state: c_uint,
}
pub type XEnterWindowEvent = XCrossingEvent;
pub type XLeaveWindowEvent = XCrossingEvent;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct XFocusChangeEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: Bool,
    pub display: *mut Display,
    pub window: Window,
    pub mode: c_int,
    pub detail: c_int,
}
pub type XFocusInEvent = XFocusChangeEvent;
pub type XFocusOutEvent = XFocusChangeEvent;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct XExposeEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: Bool,
    pub display: *mut Display,
    pub window: Window,
    pub x: c_int,
    pub y: c_int,
    pub width: c_int,
    pub height: c_int,
    pub count: c_int,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct XGraphicsExposeEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: Bool,
    pub display: *mut Display,
    pub window: Window,
    pub x: c_int,
    pub y: c_int,
    pub width: c_int,
    pub height: c_int,
    pub count: c_int,
    pub major_code: c_int,
    pub minor_code: c_int,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct XNoExposeEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: Bool,
    pub display: *mut Display,
    pub drawable: Drawable,
    pub major_code: c_int,
    pub minor_code: c_int,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct XVisibilityEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: Bool,
    pub display: *mut Display,
    pub window: Window,
    pub state: c_int,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct XCreateWindowEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: Bool,
    pub display: *mut Display,
    pub parent: Window,
    pub window: Window,
    pub x: c_int,
    pub y: c_int,
    pub width: c_int,
    pub height: c_int,
    pub border_width: c_int,
    pub override_redirect: Bool,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct XDestroyWindowEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: Bool,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct XUnmapEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: Bool,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub from_configure: Bool,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct XMapEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: Bool,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub override_redirect: Bool,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct XMapRequestEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: Bool,
    pub display: *mut Display,
    pub parent: Window,
    pub window: Window
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct XReparentEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: Bool,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub parent: Window,
    pub x: c_int,
    pub y: c_int,
    pub override_redirect: Bool,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct XConfigureEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: Bool,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub x: c_int,
    pub y: c_int,
    pub width: c_int,
    pub height: c_int,
    pub border_width: c_int,
    pub above: Window,
    pub override_redirect: Bool,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct XGravityEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: Bool,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub x: c_int,
    pub y: c_int,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct XResizeRequestEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: Bool,
    pub display: *mut Display,
    pub window: Window,
    pub width: c_int,
    pub height: c_int,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct XConfigureRequestEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: Bool,
    pub display: *mut Display,
    pub parent: Window,
    pub window: Window,
    pub x: c_int,
    pub y: c_int,
    pub width: c_int,
    pub height: c_int,
    pub border_width: c_int,
    pub above: Window,
    pub detail: c_int,
    pub value_mask: c_ulong,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct XCirculateEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: Bool,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub place: c_int,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct XCirculateRequestEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: Bool,
    pub display: *mut Display,
    pub parent: Window,
    pub window: Window,
    pub place: c_int,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct XPropertyEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: Bool,
    pub display: *mut Display,
    pub window: Window,
    pub atom: Atom,
    pub time: Time,
    pub state: c_int,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct XSelectionClearEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: Bool,
    pub display: *mut Display,
    pub window: Window,
    pub selection: Atom,
    pub time: Time,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct XSelectionRequestEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: Bool,
    pub display: *mut Display,
    pub owner: Window,
    pub requestor: Window,
    pub selection: Atom,
    pub target: Atom,
    pub property: Atom,
    pub time: Time,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct XSelectionEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: Bool,
    pub display: *mut Display,
    pub requestor: Window,
    pub selection: Atom,
    pub target: Atom,
    pub property: Atom,
    pub time: Time,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct XColormapEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: Bool,
    pub display: *mut Display,
    pub window: Window,
    pub colormap: Colormap,
    pub new: Bool,
    pub state: c_int,
}

// Anonymous in Xlib.h
#[derive(Clone, Copy)]
#[repr(C)]
pub union XClientMessageEventData {
    pub b: [c_char; 20],
    pub s: [c_short; 10],
    pub l: [c_long; 5],
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct XClientMessageEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: Bool,
    pub display: *mut Display,
    pub window: Window,
    pub message_type: Atom,
    pub format: c_int,
    pub data: XClientMessageEventData,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct XMappingEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: Bool,
    pub display: *mut Display,
    pub window: Window,
    pub request: c_int,
    pub first_keycode: c_int,
    pub count: c_int,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct XErrorEvent {
    pub type_: c_int,
    pub display: *mut Display,
    pub resourceid: XID,
    pub serial: c_ulong,
    pub error_code: c_uchar,
    pub request_code: c_uchar,
    pub minor_code: c_uchar,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct XKeymapEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: Bool,
    pub display: *mut Display,
    pub window: Window,
    pub key_vector: [c_char; 32],
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct XGenericEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: Bool,
    pub display: *mut Display,
    pub extension: c_int,
    pub evtype: c_int,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct XGenericEventCookie {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: Bool,
    pub display: *mut Display,
    pub extension: c_int,
    pub evtype: c_int,
    pub cookie: c_uint,
    pub data: *mut c_void,
}
