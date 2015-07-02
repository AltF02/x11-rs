// x11-rs: Rust bindings for X11 libraries
// The X11 libraries are available under the MIT license.
// These bindings are public domain.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(raw_pointer_derive)]

extern crate dylib;
extern crate libc;

#[macro_use]
mod link;
mod internal;

pub mod error;

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
pub mod xrecord;
pub mod xrender;
pub mod xt;
pub mod xtest;
