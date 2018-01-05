// This software is available under the terms of the MIT license.

use std::fmt::{self, Display, Formatter};

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

/// Function registry.
#[derive(Clone, Copy, Debug)]
pub struct Registry {
    pub fns: &'static [Fn],
}
