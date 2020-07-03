use ::xlib::Display;
use crate::os_primitives::c_void;

x11_link! { Xlib_xcb, xlib_xcb, ["libX11-xcb.so.1", "libX11-xcb.so"], 1,
    pub fn XGetXCBConnection(_1: *mut Display) -> *mut xcb_connection_t,
    variadic:
    globals:
}

pub type xcb_connection_t = c_void;
