// This software is available under the terms of the MIT license.

extern crate x11_registry as reg;

fn main () {
    reg::write_out_file(reg::StaticGenerator(reg::LIBX11), "bindings.rs").unwrap();
    println!("cargo:rustc-link-lib=X11");
}
