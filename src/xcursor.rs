// x11-rs: Rust bindings for X11 libraries
// The X11 libraries are available under the MIT license.
// These bindings are public domain.

use libc::c_char;

use ::xlib::{
  Cursor,
  Display,
};


#[link(name="Xcursor")]
extern "C" {
  pub fn XcursorLibraryLoadCursor (display: *mut Display, name: *const c_char) -> Cursor;
}
