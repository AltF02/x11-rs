// The X11 libraries are available under the MIT license.
// These bindings are public domain.

use std::os::raw::{
  c_int,
  c_ulong,
};

use ::xlib::XserverRegion;
use ::xlib::{
  Bool,
  XID,
  XRectangle,
  Status,
  Window,
  Display,
  Drawable,
  Damage,
  Time,
};


//
// functions
//

x11_link! { Xdamage, xdamage, ["libXdamage.so.1.1.0", "libXdamage.so"], 6,
  pub fn XDamageQueryExtension(_3: *mut Display, _2: *mut c_int, _1: *mut c_int) -> Bool,
  pub fn XDamageQueryVersion(_3: *mut Display, _2: *mut c_int, _1: *mut c_int) -> Status,
  pub fn XDamageCreate(_3: *mut Display, _2: Drawable, _1: c_int) -> Damage,
  pub fn XDamageDestroy(_2: *mut Display, _1: Damage) -> (),
  pub fn XDamageSubtract(_4: *mut Display, _3: Damage, _2: XserverRegion, _1: XserverRegion) -> (),
  pub fn XDamageAdd(_3: *mut Display, _2: Drawable, _1: XserverRegion) -> (),

variadic:
globals:
}


//
// types
//

pub type Damage = XID;

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
struct XDamageNotifyEvent {
  _type: c_int,
  serial: c_ulong,
  send_event: Bool,
  display: *mut Display,
  drawable: Drawable,
  damage: Damage,
  level: c_int,
  more: Bool,
  timestamp: Time,
  area: XRectangle,
  geometry: XRectangle,
}


//
// constants
//

pub const DAMAGE_NAME: &'static str = "DAMAGE";
pub const DAMAGE_MAJOR: c_int = 1;
pub const DAMAGE_MINOR: c_int = 1;

pub const XDamageReportRawRectangles:   c_int = 0;
pub const XDamageReportDeltaRectangles: c_int = 1;
pub const XDamageReportBoundingBox:     c_int = 2;
pub const XDamageReportNonEmpty:        c_int = 3;

pub const X_DamageQueryVersion:  c_int = 0;
pub const X_DamageCreate:        c_int = 1;
pub const X_DamageDestroy:       c_int = 2;
pub const X_DamageSubtract:      c_int = 3;
pub const X_DamageAdd:           c_int = 4;
pub const XDamageNumberRequests: c_int = 5;

pub const XDamageNotify: c_int = 0;
pub const XDamageNumberEvents: c_int = 1;

pub const BadDamage: c_int = 0;
pub const XDamageNumberErrors: c_int = 0;
