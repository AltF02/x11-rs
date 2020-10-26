// x11-rs: Rust bindings for X11 libraries
// The X11 libraries are available under the MIT license.
// These bindings are public domain.

use std::os::raw::{
  c_int,
};

use ::xlib::{
  Bool,
  XID,
  Status,
  Window,
  Display,
};

//
// functions
//

x11_link! { Xcomposite, xcomposite, ["libXcomposite.so.1", "libXcomposite.so"], 11,
  pub fn XCompositeQueryExtension(_3: *mut Display, _2: *mut c_int, _1: *mut c_int) -> Bool,
  pub fn XCompositeQueryVersion(_3: *mut Display, _2: *mut c_int, _1: *mut c_int) -> Status,
  pub fn XCompositeVersion() -> c_int,
  pub fn XCompositeRedirectWindow(_3: *mut Display, _2: Window, _1: c_int) -> (),
  pub fn XCompositeRedirectSubwindows(_3: *mut Display, _2: Window, _1: c_int) -> (),
  pub fn XCompisiteUnredirectWindow(_3: *mut Display, _2: Window, _1: c_int) -> (),
  pub fn XCompisiteUnredirectSubwindows(_3: *mut Display, _2: Window, _1: c_int) -> (),
  pub fn XCompisiteCreateRegionFromBorderClip(_2: *mut Display, _1: Window) -> XserverRegion,
  pub fn XCompositeNameWindowPixmap(_2: *mut Display, _1: Window) -> Pixmap,
  pub fn XCompositeGetOverlayWindow(_2: *mut Display, _1: Window) -> Window,
  pub fn XCompisiteReleaseOverlayWindow(_2: *mut Display, _1: Window) -> (),
variadic:
globals:
}


//
// types
//

pub type XserverRegion = XID;
pub type Pixmap = XID;
