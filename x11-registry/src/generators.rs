// This software is available under the terms of the MIT license.

use std::fmt::{self, Display, Formatter};

use registry::{Fn, Registry, Type};

/// Generates a comma-separated sequence of types.
#[derive(Clone, Copy)]
struct ParamList (&'static [Type]);

impl Display for ParamList {
    fn fmt (&self, f: &mut Formatter) -> fmt::Result {
        let mut first = true;

        for &ty in self.0.iter() {
            if first {
                first = false;
            } else {
                f.write_str(", ")?;
            }

            f.write_str("_: ")?;
            ty.fmt(f)?;
        }

        Ok(())
    }
}

/// Generates a function as a public function with no body.
/// (For use inside an `extern "C"` block).
#[derive(Clone, Copy)]
struct PubFn (Fn);

impl Display for PubFn {
    fn fmt (&self, f: &mut Formatter) -> fmt::Result {
        write!(f,
               "pub fn {} ({}){};",
               self.0.name,
               ParamList(self.0.params),
               ReturnType(self.0.return_type))
    }
}

/// Generates a return type specifier if the inner value is `Some`.
#[derive(Clone, Copy)]
struct ReturnType (Option<Type>);

impl Display for ReturnType {
    fn fmt (&self, f: &mut Formatter) -> fmt::Result {
        if let Some(ty) = self.0 {
            f.write_str(" -> ")?;
            ty.fmt(f)?;
        }

        Ok(())
    }
}

/// Generates static function bindings.
#[derive(Clone, Copy)]
pub struct StaticGenerator (pub Registry);

impl Display for StaticGenerator {
    fn fmt (&self, f: &mut Formatter) -> fmt::Result {
        f.write_str("extern \"C\" {\n")?;

        for &func in self.0.fns.iter() {
            writeln!(f, "{}", PubFn(func))?;
        }

        f.write_str("}\n")
    }
}
