// This software is available under the terms of the MIT license.

#![cfg(unix)]

extern crate libc;
extern crate x11;

use libc::*;
use x11::*;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
