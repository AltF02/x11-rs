// x11-rs: Rust bindings for X11 libraries
// The X11 libraries are available under the MIT license.
// These bindings are public domain.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(deref_nullptr)]
#![allow(clippy::missing_safety_doc)]

#[macro_use]
extern crate lazy_static;

extern crate libc;

#[macro_use]
mod link;
mod internal;

pub mod error;

#[macro_use]
pub mod xlib;

pub mod dpms;
pub mod glx;
pub mod keysym;
pub mod xcursor;
pub mod xf86vmode;
pub mod xfixes;
pub mod xft;
pub mod xinerama;
pub mod xinput;
pub mod xinput2;
pub mod xlib_xcb;
pub mod xmd;
pub mod xmu;
pub mod xrecord;
pub mod xrender;
pub mod xshm;
pub mod xss;
pub mod xt;
pub mod xtest;

pub mod xrandr {
    include!("xrandr.rs");
    include!("old_xrandr.rs");
}
