// x11-rs: Rust bindings for X11 libraries
// The X11 libraries are available under the MIT license.
// These bindings are public domain.

use libc::{
  c_int,
  c_uint,
  c_ushort,
};

use ::xlib::{
  Bool,
  Display,
};


#[link(name="Xxf86vm")]
extern "C" {
  pub fn XF86VidModeGetAllModeLines (display: *mut Display, screen: c_int, mode_count_return: *mut c_int, modes_return: *mut *mut *mut XF86VidModeModeInfo) -> Bool;
  pub fn XF86VidModeSetViewPort (display: *mut Display, screen: c_int, x: c_int, y: c_int) -> Bool;
  pub fn XF86VidModeSwitchToMode (display: *mut Display, screen: c_int, modeline: *const XF86VidModeModeInfo) -> Bool;
}


#[allow(raw_pointer_derive)]
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XF86VidModeModeInfo {
  pub dotclock: c_uint,
  pub hdisplay: c_ushort,
  pub hsyncstart: c_ushort,
  pub hsyncend: c_ushort,
  pub htotal: c_ushort,
  pub vdisplay: c_ushort,
  pub vsyncstart: c_ushort,
  pub vsyncend: c_ushort,
  pub vtotal: c_ushort,
  pub flags: c_uint,
  pub privsize: c_int,
  pub private: *mut i32,
}
