# x11-rs - X11 library bindings for Rust

[![Build Status](https://travis-ci.org/Daggerbot/x11-rs.svg?branch=master)](https://travis-ci.org/Daggerbot/x11-rs)

`x11`: [![](http://meritbadge.herokuapp.com/x11)](https://crates.io/crates/x11)

`x11-dl`: [![](http://meritbadge.herokuapp.com/x11-dl)](https://crates.io/crates/x11-dl)

The X11 libraries written in C are available under the terms of the MIT license.
These bindings are available as public domain.

Note that it in most cases it makes sense to instead use the XCB library, via
the [xcb](https://crates.io/crates/xcb) crate. XCB is a newer library for X11,
which supports multithreaded access and saner error handling, among other
improvements. Using xlib may still make sense for the following reasons:

* More examples / documentation of usage.
* When porting existing code which uses xlib.
* When requiring features only supported by xlib.

As of 2017 I don't have as much time to work on this myself as I once did (due to school and work,
both full time), but feel free to submit a pull request if anything you need is missing!
