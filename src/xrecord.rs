// x11-rs: Rust bindings for X11 libraries
// The X11 libraries are available under the MIT license.
// These bindings are public domain.

use libc::{
  c_char,
  c_int,
  c_uchar,
  c_uint,
  c_ulong,
  c_ushort,
};

use ::xinput::XDevice;
use ::xlib::{
  Bool,
  Display,
  GC,
  Time,
  Visual,
  XID,
};


//
// functions
//


x11_link! { Xf86vmode, ["libXtst.so", "libXtst.so.6"],
  pub fn XRecordAllocRange () -> *mut XRecordRange,
  pub fn XRecordCreateContext (_6: *mut Display, _5: c_int, _4: *mut c_ulong, _3: c_int, _2: *mut *mut XRecordRange, _1: c_int) -> c_ulong,
  pub fn XRecordDisableContext (_2: *mut Display, _1: c_ulong) -> c_int,
  pub fn XRecordEnableContext (_4: *mut Display, _3: c_ulong, _2: Option<unsafe extern "C" fn (*mut c_char, *mut XRecordInterceptData)>, _1: *mut c_char) -> c_int,
  pub fn XRecordEnableContextAsync (_4: *mut Display, _3: c_ulong, _2: Option<unsafe extern "C" fn (*mut c_char, *mut XRecordInterceptData)>, _1: *mut c_char) -> c_int,
  pub fn XRecordFreeContext (_2: *mut Display, _1: c_ulong) -> c_int,
  pub fn XRecordFreeData (_1: *mut XRecordInterceptData) -> (),
  pub fn XRecordFreeState (_1: *mut XRecordState) -> (),
  pub fn XRecordGetContext (_3: *mut Display, _2: c_ulong, _1: *mut *mut XRecordState) -> c_int,
  pub fn XRecordIdBaseMask (_1: *mut Display) -> c_ulong,
  pub fn XRecordProcessReplies (_1: *mut Display) -> (),
  pub fn XRecordQueryVersion (_3: *mut Display, _2: *mut c_int, _1: *mut c_int) -> c_int,
  pub fn XRecordRegisterClients (_7: *mut Display, _6: c_ulong, _5: c_int, _4: *mut c_ulong, _3: c_int, _2: *mut *mut XRecordRange, _1: c_int) -> c_int,
  pub fn XRecordUnregisterClients (_4: *mut Display, _3: c_ulong, _2: *mut c_ulong, _1: c_int) -> c_int,
  pub fn XTestCompareCurrentCursorWithWindow (_2: *mut Display, _1: c_ulong) -> c_int,
  pub fn XTestCompareCursorWithWindow (_3: *mut Display, _2: c_ulong, _1: c_ulong) -> c_int,
  pub fn XTestDiscard (_1: *mut Display) -> c_int,
  pub fn XTestFakeButtonEvent (_4: *mut Display, _3: c_uint, _2: c_int, _1: c_ulong) -> c_int,
  pub fn XTestFakeDeviceButtonEvent (_7: *mut Display, _6: *mut XDevice, _5: c_uint, _4: c_int, _3: *mut c_int, _2: c_int, _1: c_ulong) -> c_int,
  pub fn XTestFakeDeviceKeyEvent (_7: *mut Display, _6: *mut XDevice, _5: c_uint, _4: c_int, _3: *mut c_int, _2: c_int, _1: c_ulong) -> c_int,
  pub fn XTestFakeDeviceMotionEvent (_7: *mut Display, _6: *mut XDevice, _5: c_int, _4: c_int, _3: *mut c_int, _2: c_int, _1: c_ulong) -> c_int,
  pub fn XTestFakeKeyEvent (_4: *mut Display, _3: c_uint, _2: c_int, _1: c_ulong) -> c_int,
  pub fn XTestFakeMotionEvent (_5: *mut Display, _4: c_int, _3: c_int, _2: c_int, _1: c_ulong) -> c_int,
  pub fn XTestFakeProximityEvent (_6: *mut Display, _5: *mut XDevice, _4: c_int, _3: *mut c_int, _2: c_int, _1: c_ulong) -> c_int,
  pub fn XTestFakeRelativeMotionEvent (_4: *mut Display, _3: c_int, _2: c_int, _1: c_ulong) -> c_int,
  pub fn XTestGrabControl (_2: *mut Display, _1: c_int) -> c_int,
  pub fn XTestQueryExtension (_5: *mut Display, _4: *mut c_int, _3: *mut c_int, _2: *mut c_int, _1: *mut c_int) -> c_int,
  pub fn XTestSetGContextOfGC (_2: GC, _1: c_ulong) -> (),
  pub fn XTestSetVisualIDOfVisual (_2: *mut Visual, _1: c_ulong) -> (),
variadic:
globals:
}


//
// types
//


pub type XRecordClientSpec = c_ulong;
pub type XRecordContext = c_ulong;

#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XRecordClientInfo {
  pub client: XRecordClientSpec,
  pub nranges: c_ulong,
  pub ranges: *mut *mut XRecordRange,
}

#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XRecordExtRange {
  pub ext_major: XRecordRange8,
  pub ext_minor: XRecordRange16,
}

#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XRecordInterceptData {
  pub id_base: XID,
  pub server_time: Time,
  pub client_seq: c_ulong,
  pub category: c_int,
  pub client_swapped: Bool,
  pub data: *mut c_uchar,
  pub data_len: c_ulong,
}

#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XRecordRange {
  pub core_requests: XRecordRange8,
  pub core_replies: XRecordRange8,
  pub ext_requests: XRecordExtRange,
  pub ext_replies: XRecordExtRange,
  pub delivered_events: XRecordRange8,
  pub device_events: XRecordRange8,
  pub errors: XRecordRange8,
  pub client_started: Bool,
  pub client_died: Bool,
}

#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XRecordRange8 {
  pub first: c_uchar,
  pub last: c_uchar,
}

#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XRecordRange16 {
  pub first: c_ushort,
  pub last: c_ushort,
}

#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XRecordState {
  pub enabled: Bool,
  pub datum_flags: c_int,
  pub nclients: c_ulong,
  pub client_info: *mut *mut XRecordClientInfo,
}

//
// constants
//


pub const XRecordFromServerTime: c_int = 0x01;
pub const XRecordFromClientTime: c_int = 0x02;
pub const XRecordFromClientSequence: c_int = 0x04;

pub const XRecordCurrentClients: c_ulong = 1;
pub const XRecordFutureClients: c_ulong = 2;
pub const XRecordAllClients: c_ulong = 3;

pub const XRecordFromServer: c_int = 0;
pub const XRecordFromClient: c_int = 1;
pub const XRecordClientStarted: c_int = 2;
pub const XRecordClientDied: c_int = 3;
pub const XRecordStartOfData: c_int = 4;
pub const XRecordEndOfData: c_int = 5;