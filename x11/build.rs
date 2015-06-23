// x11-rs: Rust bindings for X11 libraries
// The X11 libraries are available under the MIT license.
// These bindings are public domain.

extern crate pkg_config;

fn main () {
  if cfg!(feature="glx") { let _ = pkg_config::find_library("gl"); }
  if cfg!(feature="xcursor") { let _ = pkg_config::find_library("xcursor"); }
  if cfg!(feature="xf86vmode") { let _ = pkg_config::find_library("xxf86vm"); }
  if cfg!(feature="xinerama") { let _ = pkg_config::find_library("xinerama"); }
  if cfg!(feature="xinput") { let _ = pkg_config::find_library("xi"); }
  if cfg!(feature="xlib") { let _ = pkg_config::find_library("x11"); }
  if cfg!(feature="xmu") { let _ = pkg_config::find_library("xmu"); }
  if cfg!(feature="xrender") { let _ = pkg_config::find_library("xrender"); }
  if cfg!(feature="xt") { let _ = pkg_config::find_library("xt"); }
  if cfg!(feature="xtest") { let _ = pkg_config::find_library("xtst"); }
  if cfg!(feature="xproto") { let _ = pkg_config::find_library("xproto").unwrap(); }
}
