// x11-rs: Rust bindings for X11 libraries
// The X11 libraries are available under the MIT license.
// These bindings are public domain.


//
// x11_link!
//


#[cfg(feature="dynamic-lib")]
macro_rules! x11_link {
  { $struct_name:ident, $lib_name:expr,
    $(pub fn $fn_name:ident ($($param_name:ident : $param_type:ty),*) -> $ret_type:ty,)*
  } => {
    pub struct $struct_name {
      lib: ::dylib::DynamicLibrary,
      $(pub $fn_name : unsafe extern "C" fn ($(param_name : $param_type),*) -> $ret_type,)*
    }

    impl $struct_name {
      pub fn open () -> Result<$struct_name, String> {
        unsafe {
          let lib = try!(::dylib::DynamicLibrary::open(Some(&::std::path::Path::new($lib_name))));
          $(let $fn_name: *mut () = try!(lib.symbol(stringify!($fn_name)));)*
          return Ok($struct_name {
            lib: lib,
            $($fn_name : $fn_name,)*
          });
        }
      }
    }
  };

  { $struct_name:ident, $lib_name:expr,
    $(pub fn $fn_name:ident ($($param_name:ident : $param_type:ty),*) -> $ret_type:ty,)*
    variadic:
    $(pub fn $vfn_name:ident ($($vparam_name:ident : $vparam_type:ty),+) -> $vret_type:ty,)*
  } => {
    pub struct $struct_name {
      lib: ::dylib::DynamicLibrary,
      $(pub $fn_name : unsafe extern "C" fn ($($param_type),*) -> $ret_type,)*
      $(pub $vfn_name : unsafe extern "C" fn ($($vparam_type,)+, ...) -> $vret_type,)*
    }

    impl $struct_name {
      pub fn open () -> Result<$struct_name, String> {
        unsafe {
          let lib = try!(::dylib::DynamicLibrary::open(Some(&::std::path::Path::new($lib_name))));
          $(let $fn_name: *mut () = try!(lib.symbol(stringify!($fn_name)));)*
          $(let $vfn_name: *mut () = try!(lib.symbol(stringify!($vfn_name)));)*
          return Ok($struct_name {
            lib: lib,
            $($fn_name : ::std::mem::transmute($fn_name),)*
            $($vfn_name : ::std::mem::transmute($vfn_name),)*
          });
        }
      }
    }
  };
}

#[cfg(not(feature="dyanmic-lib"))]
macro_rules! x11_link {
  { $struct_name:ident, $lib_name:expr,
    $(pub fn $fn_name:ident ($($param_name:ident : $param_type:ty),*) -> $ret_type:ty,)*
  } => {
    extern "C" {
      $(pub fn $fn_name ($($param_name : $param_type),*) -> $ret_type);*
    }
  };

  { $struct_name:ident, $lib_name:expr,
    $(pub fn $fn_name:ident ($($param_name:ident : $param_type:ty),*) -> $ret_type:ty,)*
    variadic:
    $(pub fn $vfn_name:ident ($($vparam_name:ident : $vparam_type:ty),+) -> $vret_type:ty,)*
  } => {
    extern "C" {
      $(pub fn $fn_name ($($param_name : $param_type),*) -> $ret_type;)*
      $(pub fn $vfn_name ($($vparam_name : $vparam_type),+, ...) -> $vret_type;)*
    }
  };
}
