// This software is available under the terms of the MIT license.

#![allow(non_upper_case_globals)]

extern crate libc;

mod libx11;
mod xproto;

pub use libx11::*;
pub use xproto::*;
