// x11-rs: Rust bindings for X11 libraries
// The X11 libraries are available under the MIT license.
// These bindings are public domain.

use std::cmp::min;
use std::mem::{
  size_of,
  zeroed,
};


//
// x11_link!
//


#[cfg(feature="dynamic")]
macro_rules! x11_link {
  { $struct_name:ident, $lib_name:expr,
    $(pub fn $fn_name:ident ($($param_name:ident : $param_type:ty),*) -> $ret_type:ty,)*
  } => {
    pub struct $struct_name {
      #[allow(dead_code)]
      lib: ::dylib::DynamicLibrary,
      $(pub $fn_name: unsafe extern "C" fn ($($param_type),*) -> $ret_type,)*
    }

    impl $struct_name {
      pub fn open () -> Result<$struct_name, String> {
        unsafe {
          let lib = try!(::dylib::DynamicLibrary::open(Some(::std::path::Path::new($lib_name))));
          $(let $fn_name: *mut () = try!(lib.symbol(stringify!($fn_name)));)*
          return Ok($struct_name {
            lib: lib,
            $($fn_name: ::std::mem::transmute($fn_name),)*
          });
        }
      }
    }
  };

  { $struct_name:ident, $lib_name:expr,
    $(pub fn $fn_name:ident ($($param_name:ident : $param_type:ty),*) -> $ret_type:ty,)*
    variadic:
    $(pub fn $vfn_name:ident ($($vparam_name: ident : $vparam_type:ty),+) -> $vret_type:ty,)*
  } => {
    pub struct $struct_name {
      #[allow(dead_code)]
      lib: ::dylib::DynamicLibrary,
      $(pub $fn_name: unsafe extern "C" fn ($($param_type),*) -> $ret_type,)*
      $(pub $vfn_name: unsafe extern "C" fn ($($vparam_type),+, ...) -> $vret_type,)*
    }

    impl $struct_name {
      pub fn open () -> Result<$struct_name, String> {
        unsafe {
          let lib = try!(::dylib::DynamicLibrary::open(Some(::std::path::Path::new($lib_name))));
          $(let $fn_name: *mut () = try!(lib.symbol(stringify!($fn_name)));)*
          $(let $vfn_name: *mut () = try!(lib.symbol(stringify!($vfn_name)));)*
          return Ok($struct_name {
            lib: lib,
            $($fn_name: ::std::mem::transmute($fn_name),)*
            $($vfn_name: ::std::mem::transmute($vfn_name),)*
          });
        }
      }
    }
  };
}

#[cfg(not(feature="dynamic"))]
macro_rules! x11_link {
  { $struct_name:ident, $lib_name:expr,
    $(pub fn $fn_name:ident ($($param_name:ident : $param_type:ty),*) -> $ret_type:ty,)*
  } => {
    extern "C" {
      $(pub fn $fn_name ($($param_name : $param_type),*) -> $ret_type;)*
    }
  };

  { $struct_name:ident, $lib_name:expr,
    $(pub fn $fn_name:ident ($($param_name:ident : $param_type:ty),*) -> $ret_type:ty,)*
    variadic:
    $(pub fn $vfn_name:ident ($($vparam_name: ident : $vparam_type:ty),+) -> $vret_type:ty,)*
  } => {
    extern "C" {
      $(pub fn $fn_name ($($param_name : $param_type),*) -> $ret_type;)*
      $(pub fn $vfn_name ($($vparam_name : $vparam_type),+, ...) -> $vret_type;)*
    }
  };
}


//
// public functions
//


pub unsafe fn mem_eq<T: Sized> (a: &T, b: &T) -> bool {
  let a_addr = a as *const T as usize;
  let b_addr = b as *const T as usize;

  for i in (0..size_of::<T>()) {
    if *((a_addr + i) as *const u8) != *((b_addr + i) as *const u8) {
      return false;
    }
  }

  return true;
}

pub unsafe fn transmute_union<I, O> (input: &I) -> O
  where I : Sized, O : Sized
{
  let mut output: O = zeroed();
  let copy_len = min(size_of::<I>(), size_of::<O>());

  for i in 0..copy_len {
    *((&mut output as *mut O as usize + i) as *mut u8) = *((input as *const I as usize + i) as *const u8);
  }

  return output;
}