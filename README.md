### x11-rs - Rust bindings for X11 libraries version 3

This software is available under the terms of the MIT license.

This repository provides several crates:
* `x11` - X11 types and constants.
* `x11-registry` - Registries and generators for X11 functions.

In addition to these primary crates, there are crates for individual libraries:
* `libx11-sys` - Bindings for `libX11.so`.

In `x11-rs` version 3, there is no longer an `x11-dl` crate.
I attempted to write `x11-dl` as a one-size-fits-all solution, but it had too many problems, so I've abandoned it.
If a dynamic binding is needed, a custom one must be built for each use case.
This can be easily achieved using `x11-registry`.
