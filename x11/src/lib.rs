// x11-rs: Rust bindings for X11 libraries
// The X11 libraries are available under the MIT license.
// These bindings are public domain.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

// A recent update to rustc has cause a bunch of `improper_ctypes` warnings to pop up. I believe
// that all of the types indicated are actually pointers to zero sized structs, and not zero sized
// structs themselves. I think this may be a bug in rustc. For now, let's clean up the compile log.
#![allow(improper_ctypes)]

extern crate libc;

#[macro_use]
mod link;
mod internal;

pub mod glx;
pub mod keysym;
pub mod xcursor;
pub mod xf86vmode;
pub mod xfixes;
pub mod xinerama;
pub mod xinput;
pub mod xinput2;
pub mod xlib;
pub mod xmu;
pub mod xrandr;
pub mod xrecord;
pub mod xrender;
pub mod xt;
pub mod xtest;
