// This software is available under the terms of the MIT license.

extern crate x11_registry;

fn main () {
    x11_registry::StaticFn::write_out_file(&x11_registry::LIBX11, "bindings.rs").unwrap();
    println!("cargo:rustc-link-lib=X11");
}
