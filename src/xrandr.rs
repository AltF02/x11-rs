// x11-rs: Rust bindings for X11 libraries
// The X11 libraries are available under the MIT license.
// These bindings are public domain.

use std::os::raw::{ c_char, c_int, c_long, c_short, c_uchar, c_uint, c_ulong, c_ushort };

use xlib::{ Atom, Bool, Display, Drawable, Status, Time, XEvent, XID, Window };
use xrender::{ XFixed, XTransform };


//
// functions
//


x11_link! { Xrandr, ["libXrandr.so.2", "libXrandr.so"], 70,
    pub fn XRRAddOutputMode (dpy: *mut Display, output: RROutput, mode: RRMode) -> (),
    pub fn XRRAllocGamma (size: c_int) -> *mut XRRCrtcGamma,
    pub fn XRRAllocModeInfo (name: *const c_char, nameLength: c_int) -> *mut XRRModeInfo,
    pub fn XRRAllocateMonitor (dpy: *mut Display, noutput: c_int) -> *mut XRRMonitorInfo,
    pub fn XRRChangeOutputProperty (dpy: *mut Display, output: RROutput, property: Atom, type_: Atom, format: c_int, mode: c_int, data: *const c_uchar, nelements: c_int) -> (),
    pub fn XRRChangeProviderProperty (dpy: *mut Display, provider: RRProvider, property: Atom, type_: Atom, format: c_int, mode: c_int, data: *const c_uchar, nelements: c_int) -> (),
    pub fn XRRConfigCurrentConfiguration (config: *mut XRRScreenConfiguration, rotation: *mut Rotation) -> SizeID,
    pub fn XRRConfigCurrentRate (config: *mut XRRScreenConfiguration) -> c_short,
    pub fn XRRConfigRates (config: *mut XRRScreenConfiguration, sizeID: c_int, nrates: *mut c_int) -> *mut c_short,
    pub fn XRRConfigRotations (config: *mut XRRScreenConfiguration, current_rotation: *mut Rotation) -> Rotation,
    pub fn XRRConfigSizes (config: *mut XRRScreenConfiguration, nsizes: *mut c_int) -> *mut XRRScreenSize,
    pub fn XRRConfigTimes (config: *mut XRRScreenConfiguration, config_timestamp: *mut Time) -> Time,
    pub fn XRRConfigureOutputProperty (dpy: *mut Display, output: RROutput, property: Atom, pending: Bool, range: Bool, num_values: c_int, values: *mut c_long) -> (),
    pub fn XRRConfigureProviderProperty (dpy: *mut Display, provider: RRProvider, property: Atom, pending: Bool, range: Bool, num_values: c_int, values: *mut c_long) -> (),
    pub fn XRRCreateMode (dpy: *mut Display, window: Window, modeInfo: *mut XRRModeInfo) -> RRMode,
    pub fn XRRDeleteMonitor (dpy: *mut Display, window: Window, name: Atom) -> (),
    pub fn XRRDeleteOutputMode (dpy: *mut Display, output: RROutput, mode: RRMode) -> (),
    pub fn XRRDeleteOutputProperty (dpy: *mut Display, output: RROutput, property: Atom) -> (),
    pub fn XRRDeleteProviderProperty (dpy: *mut Display, provider: RRProvider, property: Atom) -> (),
    pub fn XRRDestroyMode (dpy: *mut Display, mode: RRMode) -> (),
    pub fn XRRFreeCrtcInfo (crtcInfo: *mut XRRCrtcInfo) -> (),
    pub fn XRRFreeGamma (gamma: *mut XRRCrtcGamma) -> (),
    pub fn XRRFreeModeInfo (modeInfo: *mut XRRModeInfo) -> (),
    pub fn XRRFreeMonitors (monitors: *mut XRRMonitorInfo) -> (),
    pub fn XRRFreeOutputInfo (outputInfo: *mut XRROutputInfo) -> (),
    pub fn XRRFreePanning (panning: *mut XRRPanning) -> (),
    pub fn XRRFreeProviderInfo (provider: *mut XRRProviderInfo) -> (),
    pub fn XRRFreeProviderResources (resources: *mut XRRProviderResources) -> (),
    pub fn XRRFreeScreenConfigInfo (config: *mut XRRScreenConfiguration) -> (),
    pub fn XRRFreeScreenResources (resources: *mut XRRScreenResources) -> (),
    pub fn XRRGetCrtcGamma (dpy: *mut Display, crtc: RRCrtc) -> *mut XRRCrtcGamma,
    pub fn XRRGetCrtcGammaSize (dpy: *mut Display, crtc: RRCrtc) -> c_int,
    pub fn XRRGetCrtcInfo (dpy: *mut Display, resources: *mut XRRScreenResources, crtc: RRCrtc) -> *mut XRRCrtcInfo,
    pub fn XRRGetCrtcTransform (dpy: *mut Display, crtc: RRCrtc, attributes: *mut *mut XRRCrtcTransformAttributes) -> Status,
    pub fn XRRGetMonitors (dpy: *mut Display, window: Window, get_active: Bool, nmonitors: *mut c_int) -> *mut XRRMonitorInfo,
    pub fn XRRGetOutputInfo (dpy: *mut Display, resources: *mut XRRScreenResources, output: RROutput) -> *mut XRROutputInfo,
    pub fn XRRGetOutputPrimary (dpy: *mut Display, window: Window) -> RROutput,
    pub fn XRRGetOutputProperty (dpy: *mut Display, output: RROutput, property: Atom, offset: c_long, length: c_long, _delete: Bool, pending: Bool, req_type: Atom, actual_type: *mut Atom, actual_format: *mut c_int, nitems: *mut c_ulong, bytes_after: *mut c_ulong, prop: *mut *mut c_uchar) -> c_int,
    pub fn XRRGetPanning (dpy: *mut Display, resources: *mut XRRScreenResources, crtc: RRCrtc) -> *mut XRRPanning,
    pub fn XRRGetProviderInfo (dpy: *mut Display, resources: *mut XRRScreenResources, provider: RRProvider) -> *mut XRRProviderInfo,
    pub fn XRRGetProviderProperty (dpy: *mut Display, provider: RRProvider, property: Atom, offset: c_long, length: c_long, _delete: Bool, pending: Bool, req_type: Atom, actual_type: *mut Atom, actual_format: *mut c_int, nitems: *mut c_ulong, bytes_after: *mut c_ulong, prop: *mut *mut c_uchar) -> c_int,
    pub fn XRRGetProviderResources (dpy: *mut Display, window: Window) -> *mut XRRProviderResources,
    pub fn XRRGetScreenInfo (dpy: *mut Display, window: Window) -> *mut XRRScreenConfiguration,
    pub fn XRRGetScreenResources (dpy: *mut Display, window: Window) -> *mut XRRScreenResources,
    pub fn XRRGetScreenResourcesCurrent (dpy: *mut Display, window: Window) -> *mut XRRScreenResources,
    pub fn XRRGetScreenSizeRange (dpy: *mut Display, window: Window, minWidth: *mut c_int, minHeight: *mut c_int, maxWidth: *mut c_int, maxHeight: *mut c_int) -> Status,
    pub fn XRRListOutputProperties (dpy: *mut Display, output: RROutput, nprop: *mut c_int) -> *mut Atom,
    pub fn XRRListProviderProperties (dpy: *mut Display, provider: RRProvider, nprop: *mut c_int) -> *mut Atom,
    pub fn XRRQueryExtension (dpy: *mut Display, event_base_return: *mut c_int, error_base_return: *mut c_int) -> Bool,
    pub fn XRRQueryOutputProperty (dpy: *mut Display, output: RROutput, property: Atom) -> *mut XRRPropertyInfo,
    pub fn XRRQueryProviderProperty (dpy: *mut Display, provider: RRProvider, property: Atom) -> *mut XRRPropertyInfo,
    pub fn XRRQueryVersion (dpy: *mut Display, major_version_return: *mut c_int, minor_version_return: *mut c_int) -> Status,
    pub fn XRRRates (dpy: *mut Display, screen: c_int, sizeID: c_int, nrates: *mut c_int) -> *mut c_short,
    pub fn XRRRootToScreen (dpy: *mut Display, root: Window) -> c_int,
    pub fn XRRRotations (dpy: *mut Display, screen: c_int, current_rotation: *mut Rotation) -> Rotation,
    pub fn XRRSelectInput (dpy: *mut Display, window: Window, mask: c_int) -> (),
    pub fn XRRSetCrtcConfig (dpy: *mut Display, resources: *mut XRRScreenResources, crtc: RRCrtc, timestamp: Time, x: c_int, y: c_int, mode: RRMode, rotation: Rotation, outputs: *mut RROutput, noutputs: c_int) -> Status,
    pub fn XRRSetCrtcGamma (dpy: *mut Display, crtc: RRCrtc, gamma: *mut XRRCrtcGamma) -> (),
    pub fn XRRSetCrtcTransform (dpy: *mut Display, crtc: RRCrtc, transform: *mut XTransform, filter: *const c_char, params: *mut XFixed, nparams: c_int) -> (),
    pub fn XRRSetMonitor (dpy: *mut Display, window: Window, monitor: *mut XRRMonitorInfo) -> (),
    pub fn XRRSetOutputPrimary (dpy: *mut Display, window: Window, output: RROutput) -> (),
    pub fn XRRSetPanning (dpy: *mut Display, resources: *mut XRRScreenResources, crtc: RRCrtc, panning: *mut XRRPanning) -> Status,
    pub fn XRRSetProviderOffloadSink (dpy: *mut Display, provider: XID, sink_provider: XID) -> c_int,
    pub fn XRRSetProviderOutputSource (dpy: *mut Display, provider: XID, source_provider: XID) -> c_int,
    pub fn XRRSetScreenConfig (dpy: *mut Display, config: *mut XRRScreenConfiguration, draw: Drawable, size_index: c_int, rotation: Rotation, timestamp: Time) -> Status,
    pub fn XRRSetScreenConfigAndRate (dpy: *mut Display, config: *mut XRRScreenConfiguration, draw: Drawable, size_index: c_int, rotation: Rotation, rate: c_short, timestamp: Time) -> Status,
    pub fn XRRSetScreenSize (dpy: *mut Display, window: Window, width: c_int, height: c_int, mmWidth: c_int, mmHeight: c_int) -> (),
    pub fn XRRSizes (dpy: *mut Display, screen: c_int, nsizes: *mut c_int) -> *mut XRRScreenSize,
    pub fn XRRTimes (dpy: *mut Display, screen: c_int, config_timestamp: *mut Time) -> Time,
    pub fn XRRUpdateConfiguration (event: *mut XEvent) -> c_int,
variadic:
globals:
}


//
// types
//


pub type Connection = c_ushort;
pub type Rotation = c_ushort;
pub type SizeID = c_ushort;
pub type SubpixelOrder = c_ushort;

pub type RROutput = XID;
pub type RRCrtc = XID;
pub type RRMode = XID;
pub type RRProvider = XID;

#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XRRScreenSize {
    pub width: c_int,
    pub height: c_int,
    pub mwidth: c_int,
    pub mheight: c_int,
}

#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XRRScreenChangeNotifyEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: Bool,
    pub display: *mut Display,
    pub window: Window,
    pub root: Window,
    pub timestamp: Time,
    pub config_timestamp: Time,
    pub size_index: SizeID,
    pub subpixel_order: SubpixelOrder,
    pub rotation: Rotation,
    pub width: c_int,
    pub height: c_int,
    pub mwidth: c_int,
    pub mheight: c_int,
}

#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XRRNotifyEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: Bool,
    pub display: *mut Display,
    pub window: Window,
    pub subtype: c_int,
}

#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XRROutputChangeNotifyEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: Bool,
    pub display: *mut Display,
    pub window: Window,
    pub subtype: c_int,
    pub output: RROutput,
    pub crtc: RRCrtc,
    pub mode: RRMode,
    pub rotation: Rotation,
    pub connection: Connection,
    pub subpixel_order: SubpixelOrder,
}

#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XRRCrtcChangeNotifyEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: Bool,
    pub display: *mut Display,
    pub window: Window,
    pub subtype: c_int,
    pub crtc: RRCrtc,
    pub mode: RRMode,
    pub rotation: Rotation,
    pub x: c_int,
    pub y: c_int,
    pub width: c_uint,
    pub height: c_uint,
}

#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XRROutputPropertyNotifyEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: Bool,
    pub display: *mut Display,
    pub window: Window,
    pub subtype: c_int,
    pub output: RROutput,
    pub property: Atom,
    pub timestamp: Time,
    pub state: c_int,
}

#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XRRProviderChangeNotifyEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: Bool,
    pub display: *mut Display,
    pub window: Window,
    pub subtype: c_int,
    pub provider: RRProvider,
    pub timestamp: Time,
    pub current_role: c_uint,
}

#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XRRProviderPropertyNotifyEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: Bool,
    pub display: *mut Display,
    pub window: Window,
    pub subtype: c_int,
    pub provider: RRProvider,
    pub property: Atom,
    pub timestamp: Time,
    pub state: c_int,
}

#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XRRResourceChangeNotifyEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: Bool,
    pub display: *mut Display,
    pub window: Window,
    pub subtype: c_int,
    pub timestamp: Time,
}

#[repr(C)] pub struct XRRScreenConfiguration;

pub type XRRModeFlags = c_ulong;

#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XRRModeInfo {
    pub id: RRMode,
    pub width: c_uint,
    pub height: c_uint,
    pub dotClock: c_ulong,
    pub hSyncStart: c_uint,
    pub hSyncEnd: c_uint,
    pub hTotal: c_uint,
    pub hSkew: c_uint,
    pub vSyncStart: c_uint,
    pub vSyncEnd: c_uint,
    pub vTotal: c_uint,
    pub name: *mut c_char,
    pub nameLength: c_uint,
    pub modeFlags: XRRModeFlags,
}

#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XRRScreenResources {
    pub timestamp: Time,
    pub configTimestamp: Time,
    pub ncrtc: c_int,
    pub crtcs: *mut RRCrtc,
    pub noutput: c_int,
    pub outputs: *mut RROutput,
    pub nmode: c_int,
    pub modes: *mut XRRModeInfo,
}

#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XRROutputInfo {
    pub timestamp: Time,
    pub crtc: RRCrtc,
    pub name: *mut c_char,
    pub nameLen: c_int,
    pub mm_width: c_ulong,
    pub mm_height: c_ulong,
    pub connection: Connection,
    pub subpixel_order: SubpixelOrder,
    pub ncrtc: c_int,
    pub crtcs: *mut RRCrtc,
    pub nclone: c_int,
    pub clones: *mut RROutput,
    pub nmode: c_int,
    pub npreferred: c_int,
    pub modes: *mut RRMode,
}

#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XRRPropertyInfo {
    pub pending: Bool,
    pub range: Bool,
    pub immutable: Bool,
    pub num_values: c_int,
    pub values: *mut c_long,
}

#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XRRCrtcInfo {
    pub timestamp: Time,
    pub x: c_int,
    pub y: c_int,
    pub width: c_uint,
    pub height: c_uint,
    pub mode: RRMode,
    pub rotation: Rotation,
    pub noutput: c_int,
    pub outputs: *mut RROutput,
    pub rotations: Rotation,
    pub npossible: c_int,
    pub possible: *mut RROutput,
}

#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XRRCrtcGamma {
    pub size: c_int,
    pub red: *mut c_ushort,
    pub green: *mut c_ushort,
    pub blue: *mut c_ushort,
}

#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XRRCrtcTransformAttributes {
    pub pendingTransform: XTransform,
    pub pendingFilter: *mut c_char,
    pub pendingNparams: c_int,
    pub pendingParams: *mut XFixed,
    pub currentTransform: XTransform,
    pub currentFilter: *mut c_char,
    pub currentNparams: c_int,
    pub currentParams: *mut XFixed,
}

#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XRRPanning {
    pub timestamp: Time,
    pub left: c_uint,
    pub top: c_uint,
    pub width: c_uint,
    pub height: c_uint,
    pub track_left: c_uint,
    pub track_top: c_uint,
    pub track_width: c_uint,
    pub track_height: c_uint,
    pub border_left: c_int,
    pub border_top: c_int,
    pub border_right: c_int,
    pub border_bottom: c_int,
}

#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XRRProviderResources {
    pub timestamp: Time,
    pub nproviders: c_int,
    pub providers: *mut RRProvider,
}

#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XRRProviderInfo {
    pub capabilities: c_uint,
    pub ncrtcs: c_int,
    pub crtcs: *mut RRCrtc,
    pub noutputs: c_int,
    pub outputs: *mut RROutput,
    pub name: *mut c_char,
    pub nassociatedproviders: c_int,
    pub associated_providers: *mut RRProvider,
    pub associated_capability: *mut c_uint,
    pub nameLen: c_int,
}

#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XRRMonitorInfo {
    pub name: Atom,
    pub primary: Bool,
    pub automatic: Bool,
    pub noutput: c_int,
    pub x: c_int,
    pub y: c_int,
    pub width: c_int,
    pub height: c_int,
    pub mwidth: c_int,
    pub mheight: c_int,
    pub outputs: *mut RROutput,
}
