
extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=bz2");

    let bindings = bindgen::Builder::default()
        .header("/usr/include/X11/extensions/XInput.h")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("xinput.rs"))
        .expect("Couldn't write bindings!");
}