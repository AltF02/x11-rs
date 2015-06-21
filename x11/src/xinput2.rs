use xfixes::PointerBarrier;
use xlib::{Atom, Display, Time, Window};

// generated by cmacros
include!(concat!(env!("OUT_DIR"),"/xinput_constants.rs"));

//
// macro translations
//
fn mask_byte(mask_flag: i32) -> usize {
    (mask_flag >> 3) as usize
}

pub fn XISetMask(mask: &mut [::libc::c_uchar], event: i32) {
    mask[mask_byte(event)] |= 1 << (event & 7);
}

pub fn XIClearMask(mask: &mut [::libc::c_uchar], event: i32) {
    mask[mask_byte(event)] &= 1 << (event & 7);
}

pub fn XIMaskIsSet(mask: &[::libc::c_uchar], event: i32) -> bool {
    (mask[mask_byte(event)] & (1 << (event & 7))) != 0
}

/* automatically generated by rust-bindgen */

#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed1 {
    pub _type: ::libc::c_int,
    pub name: *mut ::libc::c_char,
    pub send_core: ::libc::c_int,
    pub enable: ::libc::c_int,
}
impl ::std::clone::Clone for Struct_Unnamed1 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed1 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type XIAddMasterInfo = Struct_Unnamed1;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed2 {
    pub _type: ::libc::c_int,
    pub deviceid: ::libc::c_int,
    pub return_mode: ::libc::c_int,
    pub return_pointer: ::libc::c_int,
    pub return_keyboard: ::libc::c_int,
}
impl ::std::clone::Clone for Struct_Unnamed2 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed2 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type XIRemoveMasterInfo = Struct_Unnamed2;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed3 {
    pub _type: ::libc::c_int,
    pub deviceid: ::libc::c_int,
    pub new_master: ::libc::c_int,
}
impl ::std::clone::Clone for Struct_Unnamed3 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed3 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type XIAttachSlaveInfo = Struct_Unnamed3;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed4 {
    pub _type: ::libc::c_int,
    pub deviceid: ::libc::c_int,
}
impl ::std::clone::Clone for Struct_Unnamed4 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed4 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type XIDetachSlaveInfo = Struct_Unnamed4;
#[repr(C)]
#[derive(Copy)]
pub struct Union_Unnamed5 {
    pub _bindgen_data_: [u64; 3usize],
}
impl Union_Unnamed5 {
    pub unsafe fn _type(&mut self) -> *mut ::libc::c_int {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn add(&mut self) -> *mut XIAddMasterInfo {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn remove(&mut self) -> *mut XIRemoveMasterInfo {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn attach(&mut self) -> *mut XIAttachSlaveInfo {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn detach(&mut self) -> *mut XIDetachSlaveInfo {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
}
impl ::std::clone::Clone for Union_Unnamed5 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Union_Unnamed5 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type XIAnyHierarchyChangeInfo = Union_Unnamed5;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed6 {
    pub base: ::libc::c_int,
    pub latched: ::libc::c_int,
    pub locked: ::libc::c_int,
    pub effective: ::libc::c_int,
}
impl ::std::clone::Clone for Struct_Unnamed6 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed6 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type XIModifierState = Struct_Unnamed6;
pub type XIGroupState = XIModifierState;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed7 {
    pub mask_len: ::libc::c_int,
    pub mask: *mut ::libc::c_uchar,
}
impl ::std::clone::Clone for Struct_Unnamed7 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed7 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type XIButtonState = Struct_Unnamed7;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed8 {
    pub mask_len: ::libc::c_int,
    pub mask: *mut ::libc::c_uchar,
    pub values: *mut ::libc::c_double,
}
impl ::std::clone::Clone for Struct_Unnamed8 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed8 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type XIValuatorState = Struct_Unnamed8;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed9 {
    pub deviceid: ::libc::c_int,
    pub mask_len: ::libc::c_int,
    pub mask: *mut ::libc::c_uchar,
}
impl ::std::clone::Clone for Struct_Unnamed9 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed9 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type XIEventMask = Struct_Unnamed9;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed10 {
    pub _type: ::libc::c_int,
    pub sourceid: ::libc::c_int,
}
impl ::std::clone::Clone for Struct_Unnamed10 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed10 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type XIAnyClassInfo = Struct_Unnamed10;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed11 {
    pub _type: ::libc::c_int,
    pub sourceid: ::libc::c_int,
    pub num_buttons: ::libc::c_int,
    pub labels: *mut Atom,
    pub state: XIButtonState,
}
impl ::std::clone::Clone for Struct_Unnamed11 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed11 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type XIButtonClassInfo = Struct_Unnamed11;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed12 {
    pub _type: ::libc::c_int,
    pub sourceid: ::libc::c_int,
    pub num_keycodes: ::libc::c_int,
    pub keycodes: *mut ::libc::c_int,
}
impl ::std::clone::Clone for Struct_Unnamed12 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed12 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type XIKeyClassInfo = Struct_Unnamed12;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed13 {
    pub _type: ::libc::c_int,
    pub sourceid: ::libc::c_int,
    pub number: ::libc::c_int,
    pub label: Atom,
    pub min: ::libc::c_double,
    pub max: ::libc::c_double,
    pub value: ::libc::c_double,
    pub resolution: ::libc::c_int,
    pub mode: ::libc::c_int,
}
impl ::std::clone::Clone for Struct_Unnamed13 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed13 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type XIValuatorClassInfo = Struct_Unnamed13;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed14 {
    pub _type: ::libc::c_int,
    pub sourceid: ::libc::c_int,
    pub number: ::libc::c_int,
    pub scroll_type: ::libc::c_int,
    pub increment: ::libc::c_double,
    pub flags: ::libc::c_int,
}
impl ::std::clone::Clone for Struct_Unnamed14 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed14 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type XIScrollClassInfo = Struct_Unnamed14;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed15 {
    pub _type: ::libc::c_int,
    pub sourceid: ::libc::c_int,
    pub mode: ::libc::c_int,
    pub num_touches: ::libc::c_int,
}
impl ::std::clone::Clone for Struct_Unnamed15 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed15 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type XITouchClassInfo = Struct_Unnamed15;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed16 {
    pub deviceid: ::libc::c_int,
    pub name: *mut ::libc::c_char,
    pub _use: ::libc::c_int,
    pub attachment: ::libc::c_int,
    pub enabled: ::libc::c_int,
    pub num_classes: ::libc::c_int,
    pub classes: *mut *mut XIAnyClassInfo,
}
impl ::std::clone::Clone for Struct_Unnamed16 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed16 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type XIDeviceInfo = Struct_Unnamed16;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed17 {
    pub modifiers: ::libc::c_int,
    pub status: ::libc::c_int,
}
impl ::std::clone::Clone for Struct_Unnamed17 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed17 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type XIGrabModifiers = Struct_Unnamed17;
pub type BarrierEventID = ::libc::c_uint;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed18 {
    pub deviceid: ::libc::c_int,
    pub barrier: PointerBarrier,
    pub eventid: BarrierEventID,
}
impl ::std::clone::Clone for Struct_Unnamed18 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed18 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type XIBarrierReleasePointerInfo = Struct_Unnamed18;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed19 {
    pub _type: ::libc::c_int,
    pub serial: ::libc::c_ulong,
    pub send_event: ::libc::c_int,
    pub display: *mut Display,
    pub extension: ::libc::c_int,
    pub evtype: ::libc::c_int,
    pub time: Time,
}
impl ::std::clone::Clone for Struct_Unnamed19 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed19 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type XIEvent = Struct_Unnamed19;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed20 {
    pub deviceid: ::libc::c_int,
    pub attachment: ::libc::c_int,
    pub _use: ::libc::c_int,
    pub enabled: ::libc::c_int,
    pub flags: ::libc::c_int,
}
impl ::std::clone::Clone for Struct_Unnamed20 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed20 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type XIHierarchyInfo = Struct_Unnamed20;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed21 {
    pub _type: ::libc::c_int,
    pub serial: ::libc::c_ulong,
    pub send_event: ::libc::c_int,
    pub display: *mut Display,
    pub extension: ::libc::c_int,
    pub evtype: ::libc::c_int,
    pub time: Time,
    pub flags: ::libc::c_int,
    pub num_info: ::libc::c_int,
    pub info: *mut XIHierarchyInfo,
}
impl ::std::clone::Clone for Struct_Unnamed21 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed21 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type XIHierarchyEvent = Struct_Unnamed21;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed22 {
    pub _type: ::libc::c_int,
    pub serial: ::libc::c_ulong,
    pub send_event: ::libc::c_int,
    pub display: *mut Display,
    pub extension: ::libc::c_int,
    pub evtype: ::libc::c_int,
    pub time: Time,
    pub deviceid: ::libc::c_int,
    pub sourceid: ::libc::c_int,
    pub reason: ::libc::c_int,
    pub num_classes: ::libc::c_int,
    pub classes: *mut *mut XIAnyClassInfo,
}
impl ::std::clone::Clone for Struct_Unnamed22 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed22 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type XIDeviceChangedEvent = Struct_Unnamed22;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed23 {
    pub _type: ::libc::c_int,
    pub serial: ::libc::c_ulong,
    pub send_event: ::libc::c_int,
    pub display: *mut Display,
    pub extension: ::libc::c_int,
    pub evtype: ::libc::c_int,
    pub time: Time,
    pub deviceid: ::libc::c_int,
    pub sourceid: ::libc::c_int,
    pub detail: ::libc::c_int,
    pub root: Window,
    pub event: Window,
    pub child: Window,
    pub root_x: ::libc::c_double,
    pub root_y: ::libc::c_double,
    pub event_x: ::libc::c_double,
    pub event_y: ::libc::c_double,
    pub flags: ::libc::c_int,
    pub buttons: XIButtonState,
    pub valuators: XIValuatorState,
    pub mods: XIModifierState,
    pub group: XIGroupState,
}
impl ::std::clone::Clone for Struct_Unnamed23 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed23 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type XIDeviceEvent = Struct_Unnamed23;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed24 {
    pub _type: ::libc::c_int,
    pub serial: ::libc::c_ulong,
    pub send_event: ::libc::c_int,
    pub display: *mut Display,
    pub extension: ::libc::c_int,
    pub evtype: ::libc::c_int,
    pub time: Time,
    pub deviceid: ::libc::c_int,
    pub sourceid: ::libc::c_int,
    pub detail: ::libc::c_int,
    pub flags: ::libc::c_int,
    pub valuators: XIValuatorState,
    pub raw_values: *mut ::libc::c_double,
}
impl ::std::clone::Clone for Struct_Unnamed24 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed24 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type XIRawEvent = Struct_Unnamed24;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed25 {
    pub _type: ::libc::c_int,
    pub serial: ::libc::c_ulong,
    pub send_event: ::libc::c_int,
    pub display: *mut Display,
    pub extension: ::libc::c_int,
    pub evtype: ::libc::c_int,
    pub time: Time,
    pub deviceid: ::libc::c_int,
    pub sourceid: ::libc::c_int,
    pub detail: ::libc::c_int,
    pub root: Window,
    pub event: Window,
    pub child: Window,
    pub root_x: ::libc::c_double,
    pub root_y: ::libc::c_double,
    pub event_x: ::libc::c_double,
    pub event_y: ::libc::c_double,
    pub mode: ::libc::c_int,
    pub focus: ::libc::c_int,
    pub same_screen: ::libc::c_int,
    pub buttons: XIButtonState,
    pub mods: XIModifierState,
    pub group: XIGroupState,
}
impl ::std::clone::Clone for Struct_Unnamed25 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed25 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type XIEnterEvent = Struct_Unnamed25;
pub type XILeaveEvent = XIEnterEvent;
pub type XIFocusInEvent = XIEnterEvent;
pub type XIFocusOutEvent = XIEnterEvent;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed26 {
    pub _type: ::libc::c_int,
    pub serial: ::libc::c_ulong,
    pub send_event: ::libc::c_int,
    pub display: *mut Display,
    pub extension: ::libc::c_int,
    pub evtype: ::libc::c_int,
    pub time: Time,
    pub deviceid: ::libc::c_int,
    pub property: Atom,
    pub what: ::libc::c_int,
}
impl ::std::clone::Clone for Struct_Unnamed26 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed26 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type XIPropertyEvent = Struct_Unnamed26;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed27 {
    pub _type: ::libc::c_int,
    pub serial: ::libc::c_ulong,
    pub send_event: ::libc::c_int,
    pub display: *mut Display,
    pub extension: ::libc::c_int,
    pub evtype: ::libc::c_int,
    pub time: Time,
    pub deviceid: ::libc::c_int,
    pub sourceid: ::libc::c_int,
    pub touchid: ::libc::c_uint,
    pub root: Window,
    pub event: Window,
    pub child: Window,
    pub flags: ::libc::c_int,
}
impl ::std::clone::Clone for Struct_Unnamed27 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed27 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type XITouchOwnershipEvent = Struct_Unnamed27;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed28 {
    pub _type: ::libc::c_int,
    pub serial: ::libc::c_ulong,
    pub send_event: ::libc::c_int,
    pub display: *mut Display,
    pub extension: ::libc::c_int,
    pub evtype: ::libc::c_int,
    pub time: Time,
    pub deviceid: ::libc::c_int,
    pub sourceid: ::libc::c_int,
    pub event: Window,
    pub root: Window,
    pub root_x: ::libc::c_double,
    pub root_y: ::libc::c_double,
    pub dx: ::libc::c_double,
    pub dy: ::libc::c_double,
    pub dtime: ::libc::c_int,
    pub flags: ::libc::c_int,
    pub barrier: PointerBarrier,
    pub eventid: BarrierEventID,
}
impl ::std::clone::Clone for Struct_Unnamed28 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed28 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type XIBarrierEvent = Struct_Unnamed28;
