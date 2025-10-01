// x11-rs: Rust bindings for X11 libraries
// The X11 libraries are available under the MIT license.
// These bindings are public domain.

macro_rules! x11_link {
  { $struct_name:ident, $pkg_name:expr, [$($lib_name:expr),*], $nsyms:expr,
    $(pub fn $fn_name:ident ($($param_name:ident : $param_type:ty),*) -> $ret_type:ty,)*
    variadic:
    $(pub fn $vfn_name:ident ($($vparam_name: ident : $vparam_type:ty),+) -> $vret_type:ty,)*
    globals:
    $(pub static $var_name:ident : $var_type:ty,)*
  } => {
    extern "C" {
      $(pub fn $fn_name ($($param_name : $param_type),*) -> $ret_type;)*
      $(pub fn $vfn_name ($($vparam_name : $vparam_type),+, ...) -> $vret_type;)*
    }

    extern "C" {
      $(pub static $var_name : $var_type;)*
    }

    #[allow(unused)]
    #[allow(clippy::manual_non_exhaustive)]
    pub struct $struct_name {
      _private: (),
      $(pub $fn_name: unsafe extern "C" fn ($($param_type),*) -> $ret_type,)*
      $(pub $vfn_name: unsafe extern "C" fn ($($vparam_type),+, ...) -> $vret_type,)*
      $(pub $var_name: *mut $var_type,)*
    }

    unsafe impl Send for $struct_name {}
    unsafe impl Sync for $struct_name {}

    impl $struct_name {
      pub fn open () -> Result<$struct_name, $crate::error::OpenError> {
        Ok($struct_name {
          _private: (),
          $($fn_name: $fn_name,)*
          $($vfn_name: $vfn_name,)*
          $($var_name: unsafe { ::std::mem::transmute($var_name) },)*
        })
      }
    }
  }
}
