// This software is available under the terms of the MIT license.

use std::env;
use std::fmt::{self, Display, Formatter};
use std::fs::File;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

/// Type descriptor.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Type {
    Ident(&'static str),
    ConstPtr(&'static Type),
    MutPtr(&'static Type),
}

impl Display for Type {
    fn fmt (&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            Type::Ident(name) => f.write_str(name),
            Type::ConstPtr(inner) => {
                f.write_str("*const ")?;
                inner.fmt(f)
            },
            Type::MutPtr(inner) => {
                f.write_str("*mut ")?;
                inner.fmt(f)
            },
        }
    }
}

/// Function descriptor.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Fn {
    pub name: &'static str,
    pub return_type: Option<Type>,
    pub params: &'static [Type],
}

/// Static function formatter.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StaticFn (pub Fn);

impl StaticFn {
    pub fn write<W: Write> (fns: &[Fn], w: &mut W) -> io::Result<()> {
        writeln!(w, "extern \"C\" {{")?;
        for &f in fns.iter() {
            writeln!(w, "{}", StaticFn(f))?;
        }
        writeln!(w, "}}")?;
        Ok(())
    }

    pub fn write_file<P: AsRef<Path>> (fns: &[Fn], path: P) -> io::Result<()> {
        let mut file = File::create(path)?;
        StaticFn::write(fns, &mut file)
    }

    pub fn write_out_file<P: AsRef<Path>> (fns: &[Fn], path: P) -> io::Result<()> {
        match env::var("OUT_DIR") {
            Ok(outdir) => StaticFn::write_file(fns, PathBuf::from(outdir).join(path)),
            Err(err) => Err(io::Error::new(io::ErrorKind::Other, err)),
        }
    }
}

impl Display for StaticFn {
    fn fmt (&self, f: &mut Formatter) -> fmt::Result {
        f.write_str("pub fn ")?;
        f.write_str(self.0.name)?;
        f.write_str(" (")?;

        let mut first_param = true;

        for &ty in self.0.params.iter() {
            if !first_param {
                f.write_str(", ")?;
                first_param = true;
            }
            f.write_str("_: ")?;
            ty.fmt(f)?;
        }

        f.write_str(")")?;

        if let Some(ty) = self.0.return_type {
            f.write_str(" -> ")?;
            ty.fmt(f)?;
        }

        f.write_str(";")
    }
}

//============================================================================//
// Registries

#[cfg(feature = "libx11")]
include!(concat!(env!("OUT_DIR"), "/libx11.rs"));
