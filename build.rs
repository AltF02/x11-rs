extern crate gl_generator;

use std::env::var;
use std::fs::File;
use std::path::PathBuf;


fn main () {
  let out_dir;
  match var("OUT_DIR") {
    Ok(s) => { out_dir = s; },
    Err(e) => { panic!("can't get $OUT_DIR: {}", e); },
  }

  let file;
  match File::create(out_dir + "/glx_generated.rs") {
    Ok(f) => { file = f; },
    Err(e) => { panic!("can't open glx_generated.rs: {}", e); },
  }

  match gl_generator::generate_bindings(gl_generator::StaticGenerator,
      gl_generator::registry::Ns::Glx, gl_generator::Fallback::All,
}
