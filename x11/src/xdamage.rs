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
struct XDamageNotifyEvent{
    type: c_int;			/* event base */
    long serial: c_ulong;
    send_event: Bool;
    display: *mut Display;
    drawable: Drawable;
    damage: Damage;
    level: c_int;
    more: Bool;			/* more events will be delivered immediately */
    timestamp: Time;
    area: XRectangle;
    geometry: XRectange;
};
