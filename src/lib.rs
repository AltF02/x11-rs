// x11-rs: Rust bindings for X11 libraries
// The X11 libraries are available under the MIT license.
// These bindings are public domain.

#![crate_name="x11"]
#![crate_type="lib"]

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(raw_pointer_derive)]

extern crate libc;

pub mod glx;
pub mod keysym;
pub mod xcursor;
pub mod xf86vmode;
pub mod xlib;
pub mod xrender;
