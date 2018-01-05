// This software is available under the terms of the MIT license.

mod generators;
mod registry;

use std::env;
use std::fmt::Display;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::io::{self, Write};

pub use generators::{StaticGenerator};
pub use registry::{Fn, Registry, Type};

/// Writes a registry to a file in 'OUT_DIR'.
pub fn write_out_file<G, P> (generator: G, relative_path: P) -> io::Result<()>
    where G: Display, P: AsRef<Path>
{
    let path = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR not set")).join(relative_path);
    let mut file = match File::create(&path) {
        Ok(f) => f,
        Err(err) => panic!("can't create '{}': {}", path.display(), err),
    };
    write!(file, "{}", generator).expect("write failed");
    Ok(())
}

// Include parsed registries.

#[cfg(feature = "libx11")] include!(concat!(env!("OUT_DIR"), "/libx11.rs"));
