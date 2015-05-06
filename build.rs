// x11-rs: Rust bindings for X11 libraries
// The X11 libraries are available under the MIT license.
// These bindings are public domain.

extern crate pkg_config;

#[cfg(feature="dynamic")]
fn main () {
}

#[cfg(not(feature="dynamic"))]
fn main () {
  let _ = pkg_config::find_library("gl");
  let _ = pkg_config::find_library("x11");
  let _ = pkg_config::find_library("xcursor");
  let _ = pkg_config::find_library("xrender");
  let _ = pkg_config::find_library("xxf86vm");
}
