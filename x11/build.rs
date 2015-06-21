// x11-rs: Rust bindings for X11 libraries
// The X11 libraries are available under the MIT license.
// These bindings are public domain.

use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::path::Path;

extern crate cmacros;
extern crate pkg_config;

#[derive(Debug)]
enum GenerateErr {
    Io(io::Error),
    Parse(String)
}

impl From<io::Error> for GenerateErr {
    fn from(err: io::Error) -> Self {
        GenerateErr::Io(err)
    }
}
impl From<String> for GenerateErr {
    fn from(err: String) -> Self {
        GenerateErr::Parse(err)
    }
}

fn generate_x11_constants(output_dir: &str) -> Result<(), GenerateErr> {
    let root_dir = Path::new("/usr/include/X11");

    let inputs = [("extensions/XI2.h", "xinput_constants.rs")];

    for input in &inputs {
        let input_path = input.0;
        let output_filename = input.1;

        let x11_header_path = root_dir.join(input_path);
        let mut x11_header = try!(File::open(x11_header_path));
        let mut header_src = String::new();
        try!(x11_header.read_to_string(&mut header_src));

        let macros = try!(cmacros::extract_macros(&header_src));
        let skipped_macros = [
            // macros with expansions that cmacros
            // cannot yet translate
            "XIAnyPropertyType",
            "XIAnyModifier",
            "XIOwnerEvents",
            "XINoOwnerEvents"
        ];
        let mut output = cmacros::generate_rust_src(&macros, |def| {
            // older versions of XI2.h have an
            // invalid definition of XI_TouchOwnershipChangedMask.
            // See http://cgit.freedesktop.org/xorg/proto/inputproto/commit/XI2.h?id=c2cf8cab4aa781306ff26b171107d26f12bac015
            if def.name == "XI_TouchOwnershipChangedMask" {
                cmacros::translate_macro(&cmacros::CMacro{
                    name: def.name.clone(),
                    args: None,
                    body: Some("(1 << XI_TouchOwnership)".to_string())
                })
            } else if !skipped_macros.contains(&(&def.name as &str)) {
                cmacros::translate_macro(def)
            } else {
                cmacros::TranslateAction::Skip
            }
        });
        output.push_str("\n");

        let output_path = Path::new(output_dir).join(output_filename);
        let mut output_file = try!(File::create(output_path));
        try!(output_file.write_all(output.as_bytes()));
    }
    Ok(())
}

fn main () {
    match generate_x11_constants(env!("OUT_DIR")) {
        Ok(_) => (),
        Err(err) => panic!("Failed to generate X11 constants: {:?}", err)
    }

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
}
