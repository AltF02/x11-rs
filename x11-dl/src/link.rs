// x11-rs: Rust bindings for X11 libraries
// The X11 libraries are available under the MIT license.
// These bindings are public domain.

use std::path::Path;

use dylib::DynamicLibrary;

use ::error::{
  OpenError,
  OpenErrorKind,
};


//
// x11_link!
//


macro_rules! x11_link {
  { $struct_name:ident, [$($lib_name:expr),*],
    $(pub fn $fn_name:ident ($($param_name:ident : $param_type:ty),*) -> $ret_type:ty,)*
    variadic:
    $(pub fn $vfn_name:ident ($($vparam_name: ident : $vparam_type:ty),+) -> $vret_type:ty,)*
    globals:
    $(pub static $var_name:ident : $var_type:ty,)*
  } => {
    pub struct $struct_name {
      #[allow(dead_code)]
      lib: ::dylib::DynamicLibrary,
      $(pub $fn_name: unsafe extern "C" fn ($($param_type),*) -> $ret_type,)*
      $(pub $vfn_name: unsafe extern "C" fn ($($vparam_type),+, ...) -> $vret_type,)*
      $(pub $var_name: *mut $var_type,)*
    }

    impl $struct_name {
      pub fn open () -> Result<$struct_name, ::error::OpenError> {
        unsafe {
          let mut s: $struct_name = ::std::mem::uninitialized();
          s.lib = try!(::link::open_lib(&[$($lib_name),*]));
          $(s.$fn_name = ::std::mem::transmute(try!(::link::get_sym(&s.lib, stringify!($fn_name))));)*
          $(s.$vfn_name = ::std::mem::transmute(try!(::link::get_sym(&s.lib, stringify!($vfn_name))));)*
          $(s.$var_name = ::std::mem::transmute(try!(::link::get_sym(&s.lib, stringify!($var_name))));)*
          Ok(s)
        }
      }
    }
  }
}


//
// public functions
//


pub fn get_sym (lib: &DynamicLibrary, name: &str) -> Result<*mut (), OpenError> {
  unsafe {
    match lib.symbol(name) {
      Ok(sym) => Ok(sym),
      Err(msg) => Err(OpenError::new(OpenErrorKind::Symbol, msg)),
    }
  }
}

pub fn open_lib (names: &[&'static str]) -> Result<DynamicLibrary, OpenError> {
  assert!(!names.is_empty());
  let mut msgs = Vec::new();

  for name in names.iter() {
    match DynamicLibrary::open(Some(Path::new(*name))) {
      Ok(lib) => { return Ok(lib); },
      Err(err) => { msgs.push(err); },
    }
  }

  let mut detail = String::new();
  for i in 0..msgs.len() {
    if i != 0 {
      detail.push_str(", ");
    }
    detail.push_str("\"");
    detail.push_str(msgs[i].as_ref());
    detail.push_str("\"");
  }

  return Err(OpenError::new(OpenErrorKind::Library, detail));
}
