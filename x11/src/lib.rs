// x11-rs: Rust bindings for X11 libraries
// The X11 libraries are available under the MIT license.
// These bindings are public domain.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]
#![allow(unused_parens)]
#![no_std]

extern crate libc;
#[cfg(any(test, feature = "std"))]
extern crate std;

// usage of c primitives
pub(crate) mod os_primitives {
    // if we can use std, re-export all of os::raw
    #[cfg(feature = "std")]
    pub use std::os::raw::*;
    // otherwise, re-export the primitives from libc
    #[cfg(not(feature = "std"))]
    pub use libc::{
        c_char, c_double, c_float, c_int, c_long, c_longlong, c_schar, c_short, c_uchar, c_uint,
        c_ulong, c_ulonglong, c_ushort, c_void,
    };
}

#[macro_use]
mod link;
mod internal;

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
pub mod xrandr;
pub mod xrecord;
pub mod xrender;
pub mod xss;
pub mod xt;
pub mod xtest;
