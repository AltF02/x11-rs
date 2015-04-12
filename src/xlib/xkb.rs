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

use ::xlib::{
  Atom,
  Display,
  KeyCode,
};


//
// functions
//


#[link(name="X11")]
extern "C" {
  pub fn XkbAddDeviceLedInfo (_3: XkbDeviceInfoPtr, _2: c_uint, _1: c_uint) -> XkbDeviceLedInfoPtr;
  pub fn XkbAddGeomColor (_3: XkbGeometryPtr, _2: *mut c_char, _1: c_uint) -> XkbColorPtr;
  pub fn XkbAddGeomDoodad (_3: XkbGeometryPtr, _2: XkbSectionPtr, _1: c_ulong) -> XkbDoodadPtr;
  pub fn XkbAddGeomKey (_1: XkbRowPtr) -> XkbKeyPtr;
  pub fn XkbAddGeomKeyAlias (_3: XkbGeometryPtr, _2: *mut c_char, _1: *mut c_char) -> XkbKeyAliasPtr;
  pub fn XkbAddGeomOutline (_2: XkbShapePtr, _1: c_int) -> XkbOutlinePtr;
  pub fn XkbAddGeomOverlay (_3: XkbSectionPtr, _2: c_ulong, _1: c_int) -> XkbOverlayPtr;
  pub fn XkbAddGeomOverlayKey (_4: XkbOverlayPtr, _3: XkbOverlayRowPtr, _2: *mut c_char, _1: *mut c_char) -> XkbOverlayKeyPtr;
  pub fn XkbAddGeomOverlayRow (_3: XkbOverlayPtr, _2: c_int, _1: c_int) -> XkbOverlayRowPtr;
  pub fn XkbAddGeomProperty (_3: XkbGeometryPtr, _2: *mut c_char, _1: *mut c_char) -> XkbPropertyPtr;
  pub fn XkbAddGeomRow (_2: XkbSectionPtr, _1: c_int) -> XkbRowPtr;
  pub fn XkbAddGeomSection (_5: XkbGeometryPtr, _4: c_ulong, _3: c_int, _2: c_int, _1: c_int) -> XkbSectionPtr;
  pub fn XkbAddGeomShape (_3: XkbGeometryPtr, _2: c_ulong, _1: c_int) -> XkbShapePtr;
  pub fn XkbAddKeyType (_5: XkbDescPtr, _4: c_ulong, _3: c_int, _2: c_int, _1: c_int) -> XkbKeyTypePtr;
  pub fn XkbAllocClientMap (_3: XkbDescPtr, _2: c_uint, _1: c_uint) -> c_int;
  pub fn XkbAllocCompatMap (_3: XkbDescPtr, _2: c_uint, _1: c_uint) -> c_int;
  pub fn XkbAllocControls (_2: XkbDescPtr, _1: c_uint) -> c_int;
  pub fn XkbAllocDeviceInfo (_3: c_uint, _2: c_uint, _1: c_uint) -> XkbDeviceInfoPtr;
  pub fn XkbAllocGeomColors (_2: XkbGeometryPtr, _1: c_int) -> c_int;
  pub fn XkbAllocGeomDoodads (_2: XkbGeometryPtr, _1: c_int) -> c_int;
  pub fn XkbAllocGeometry (_2: XkbDescPtr, _1: XkbGeometrySizesPtr) -> c_int;
  pub fn XkbAllocGeomKeyAliases (_2: XkbGeometryPtr, _1: c_int) -> c_int;
  pub fn XkbAllocGeomKeys (_2: XkbRowPtr, _1: c_int) -> c_int;
  pub fn XkbAllocGeomOutlines (_2: XkbShapePtr, _1: c_int) -> c_int;
  pub fn XkbAllocGeomOverlayKeys (_2: XkbOverlayRowPtr, _1: c_int) -> c_int;
  pub fn XkbAllocGeomOverlayRows (_2: XkbOverlayPtr, _1: c_int) -> c_int;
  pub fn XkbAllocGeomOverlays (_2: XkbSectionPtr, _1: c_int) -> c_int;
  pub fn XkbAllocGeomPoints (_2: XkbOutlinePtr, _1: c_int) -> c_int;
  pub fn XkbAllocGeomProps (_2: XkbGeometryPtr, _1: c_int) -> c_int;
  pub fn XkbAllocGeomRows (_2: XkbSectionPtr, _1: c_int) -> c_int;
  pub fn XkbAllocGeomSectionDoodads (_2: XkbSectionPtr, _1: c_int) -> c_int;
  pub fn XkbAllocGeomSections (_2: XkbGeometryPtr, _1: c_int) -> c_int;
  pub fn XkbAllocGeomShapes (_2: XkbGeometryPtr, _1: c_int) -> c_int;
  pub fn XkbAllocIndicatorMaps (_1: XkbDescPtr) -> c_int;
  pub fn XkbAllocKeyboard () -> XkbDescPtr;
  pub fn XkbAllocNames (_4: XkbDescPtr, _3: c_uint, _2: c_int, _1: c_int) -> c_int;
  pub fn XkbAllocServerMap (_3: XkbDescPtr, _2: c_uint, _1: c_uint) -> c_int;
  pub fn XkbApplyCompatMapToKey (_3: XkbDescPtr, _2: c_uchar, _1: XkbChangesPtr) -> c_int;
  pub fn XkbApplyVirtualModChanges (_3: XkbDescPtr, _2: c_uint, _1: XkbChangesPtr) -> c_int;
  pub fn XkbBell (_4: *mut Display, _3: c_ulong, _2: c_int, _1: c_ulong) -> c_int;
  pub fn XkbBellEvent (_4: *mut Display, _3: c_ulong, _2: c_int, _1: c_ulong) -> c_int;
  pub fn XkbChangeDeviceInfo (_3: *mut Display, _2: XkbDeviceInfoPtr, _1: XkbDeviceChangesPtr) -> c_int;
  pub fn XkbChangeEnabledControls (_4: *mut Display, _3: c_uint, _2: c_uint, _1: c_uint) -> c_int;
  pub fn XkbChangeKeycodeRange (_4: XkbDescPtr, _3: c_int, _2: c_int, _1: XkbChangesPtr) -> c_int;
  pub fn XkbChangeMap (_3: *mut Display, _2: XkbDescPtr, _1: XkbMapChangesPtr) -> c_int;
  pub fn XkbChangeNames (_3: *mut Display, _2: XkbDescPtr, _1: XkbNameChangesPtr) -> c_int;
  pub fn XkbChangeTypesOfKey (_6: XkbDescPtr, _5: c_int, _4: c_int, _3: c_uint, _2: *mut c_int, _1: XkbMapChangesPtr) -> c_int;
  pub fn XkbComputeEffectiveMap (_3: XkbDescPtr, _2: XkbKeyTypePtr, _1: *mut c_uchar) -> c_int;
  pub fn XkbComputeRowBounds (_3: XkbGeometryPtr, _2: XkbSectionPtr, _1: XkbRowPtr) -> c_int;
  pub fn XkbComputeSectionBounds (_2: XkbGeometryPtr, _1: XkbSectionPtr) -> c_int;
  pub fn XkbComputeShapeBounds (_1: XkbShapePtr) -> c_int;
  pub fn XkbComputeShapeTop (_2: XkbShapePtr, _1: XkbBoundsPtr) -> c_int;
  pub fn XkbCopyKeyType (_2: XkbKeyTypePtr, _1: XkbKeyTypePtr) -> c_int;
  pub fn XkbCopyKeyTypes (_3: XkbKeyTypePtr, _2: XkbKeyTypePtr, _1: c_int) -> c_int;
  pub fn XkbDeviceBell (_7: *mut Display, _6: c_ulong, _5: c_int, _4: c_int, _3: c_int, _2: c_int, _1: c_ulong) -> c_int;
  pub fn XkbDeviceBellEvent (_7: *mut Display, _6: c_ulong, _5: c_int, _4: c_int, _3: c_int, _2: c_int, _1: c_ulong) -> c_int;
  pub fn XkbFindOverlayForKey (_3: XkbGeometryPtr, _2: XkbSectionPtr, _1: *mut c_char) -> *mut c_char;
  pub fn XkbForceBell (_2: *mut Display, _1: c_int) -> c_int;
  pub fn XkbForceDeviceBell (_5: *mut Display, _4: c_int, _3: c_int, _2: c_int, _1: c_int) -> c_int;
  pub fn XkbFreeClientMap (_3: XkbDescPtr, _2: c_uint, _1: c_int);
  pub fn XkbFreeCompatMap (_3: XkbDescPtr, _2: c_uint, _1: c_int);
  pub fn XkbFreeComponentList (_1: XkbComponentListPtr);
  pub fn XkbFreeControls (_3: XkbDescPtr, _2: c_uint, _1: c_int);
  pub fn XkbFreeDeviceInfo (_3: XkbDeviceInfoPtr, _2: c_uint, _1: c_int);
  pub fn XkbFreeGeomColors (_4: XkbGeometryPtr, _3: c_int, _2: c_int, _1: c_int);
  pub fn XkbFreeGeomDoodads (_3: XkbDoodadPtr, _2: c_int, _1: c_int);
  pub fn XkbFreeGeometry (_3: XkbGeometryPtr, _2: c_uint, _1: c_int);
  pub fn XkbFreeGeomKeyAliases (_4: XkbGeometryPtr, _3: c_int, _2: c_int, _1: c_int);
  pub fn XkbFreeGeomKeys (_4: XkbRowPtr, _3: c_int, _2: c_int, _1: c_int);
  pub fn XkbFreeGeomOutlines (_4: XkbShapePtr, _3: c_int, _2: c_int, _1: c_int);
  pub fn XkbFreeGeomOverlayKeys (_4: XkbOverlayRowPtr, _3: c_int, _2: c_int, _1: c_int);
  pub fn XkbFreeGeomOverlayRows (_4: XkbOverlayPtr, _3: c_int, _2: c_int, _1: c_int);
  pub fn XkbFreeGeomOverlays (_4: XkbSectionPtr, _3: c_int, _2: c_int, _1: c_int);
  pub fn XkbFreeGeomPoints (_4: XkbOutlinePtr, _3: c_int, _2: c_int, _1: c_int);
  pub fn XkbFreeGeomProperties (_4: XkbGeometryPtr, _3: c_int, _2: c_int, _1: c_int);
  pub fn XkbFreeGeomRows (_4: XkbSectionPtr, _3: c_int, _2: c_int, _1: c_int);
  pub fn XkbFreeGeomSections (_4: XkbGeometryPtr, _3: c_int, _2: c_int, _1: c_int);
  pub fn XkbFreeGeomShapes (_4: XkbGeometryPtr, _3: c_int, _2: c_int, _1: c_int);
  pub fn XkbFreeIndicatorMaps (_1: XkbDescPtr);
  pub fn XkbFreeKeyboard (_3: XkbDescPtr, _2: c_uint, _1: c_int);
  pub fn XkbFreeNames (_3: XkbDescPtr, _2: c_uint, _1: c_int);
  pub fn XkbFreeServerMap (_3: XkbDescPtr, _2: c_uint, _1: c_int);
  pub fn XkbGetAutoRepeatRate (_4: *mut Display, _3: c_uint, _2: *mut c_uint, _1: *mut c_uint) -> c_int;
  pub fn XkbGetAutoResetControls (_3: *mut Display, _2: *mut c_uint, _1: *mut c_uint) -> c_int;
  pub fn XkbGetCompatMap (_3: *mut Display, _2: c_uint, _1: XkbDescPtr) -> c_int;
  pub fn XkbGetControls (_3: *mut Display, _2: c_ulong, _1: XkbDescPtr) -> c_int;
  pub fn XkbGetDetectableAutoRepeat (_2: *mut Display, _1: *mut c_int) -> c_int;
  pub fn XkbGetDeviceButtonActions (_5: *mut Display, _4: XkbDeviceInfoPtr, _3: c_int, _2: c_uint, _1: c_uint) -> c_int;
  pub fn XkbGetDeviceInfo (_5: *mut Display, _4: c_uint, _3: c_uint, _2: c_uint, _1: c_uint) -> XkbDeviceInfoPtr;
  pub fn XkbGetDeviceInfoChanges (_3: *mut Display, _2: XkbDeviceInfoPtr, _1: XkbDeviceChangesPtr) -> c_int;
  pub fn XkbGetDeviceLedInfo (_5: *mut Display, _4: XkbDeviceInfoPtr, _3: c_uint, _2: c_uint, _1: c_uint) -> c_int;
  pub fn XkbGetGeometry (_2: *mut Display, _1: XkbDescPtr) -> c_int;
  pub fn XkbGetIndicatorMap (_3: *mut Display, _2: c_ulong, _1: XkbDescPtr) -> c_int;
  pub fn XkbGetIndicatorState (_3: *mut Display, _2: c_uint, _1: *mut c_uint) -> c_int;
  pub fn XkbGetKeyActions (_4: *mut Display, _3: c_uint, _2: c_uint, _1: XkbDescPtr) -> c_int;
  pub fn XkbGetKeyBehaviors (_4: *mut Display, _3: c_uint, _2: c_uint, _1: XkbDescPtr) -> c_int;
  pub fn XkbGetKeyboard (_3: *mut Display, _2: c_uint, _1: c_uint) -> XkbDescPtr;
  pub fn XkbGetKeyboardByName (_6: *mut Display, _5: c_uint, _4: XkbComponentNamesPtr, _3: c_uint, _2: c_uint, _1: c_int) -> XkbDescPtr;
  pub fn XkbGetKeyExplicitComponents (_4: *mut Display, _3: c_uint, _2: c_uint, _1: XkbDescPtr) -> c_int;
  pub fn XkbGetKeyModifierMap (_4: *mut Display, _3: c_uint, _2: c_uint, _1: XkbDescPtr) -> c_int;
  pub fn XkbGetKeySyms (_4: *mut Display, _3: c_uint, _2: c_uint, _1: XkbDescPtr) -> c_int;
  pub fn XkbGetKeyTypes (_4: *mut Display, _3: c_uint, _2: c_uint, _1: XkbDescPtr) -> c_int;
  pub fn XkbGetKeyVirtualModMap (_4: *mut Display, _3: c_uint, _2: c_uint, _1: XkbDescPtr) -> c_int;
  pub fn XkbGetMap (_3: *mut Display, _2: c_uint, _1: c_uint) -> XkbDescPtr;
  pub fn XkbGetMapChanges (_3: *mut Display, _2: XkbDescPtr, _1: XkbMapChangesPtr) -> c_int;
  pub fn XkbGetNamedDeviceIndicator (_9: *mut Display, _8: c_uint, _7: c_uint, _6: c_uint, _5: c_ulong, _4: *mut c_int, _3: *mut c_int, _2: XkbIndicatorMapPtr, _1: *mut c_int) -> c_int;
  pub fn XkbGetNamedGeometry (_3: *mut Display, _2: XkbDescPtr, _1: c_ulong) -> c_int;
  pub fn XkbGetNamedIndicator (_6: *mut Display, _5: c_ulong, _4: *mut c_int, _3: *mut c_int, _2: XkbIndicatorMapPtr, _1: *mut c_int) -> c_int;
  pub fn XkbGetNames (_3: *mut Display, _2: c_uint, _1: XkbDescPtr) -> c_int;
  pub fn XkbGetPerClientControls (_2: *mut Display, _1: *mut c_uint) -> c_int;
  pub fn XkbGetState (_3: *mut Display, _2: c_uint, _1: XkbStatePtr) -> c_int;
  pub fn XkbGetUpdatedMap (_3: *mut Display, _2: c_uint, _1: XkbDescPtr) -> c_int;
  pub fn XkbGetVirtualMods (_3: *mut Display, _2: c_uint, _1: XkbDescPtr) -> c_int;
  pub fn XkbGetXlibControls (_1: *mut Display) -> c_uint;
  pub fn XkbIgnoreExtension (_1: c_int) -> c_int;
  pub fn XkbInitCanonicalKeyTypes (_3: XkbDescPtr, _2: c_uint, _1: c_int) -> c_int;
  pub fn XkbKeycodeToKeysym (_4: *mut Display, _3: c_uchar, _2: c_int, _1: c_int) -> c_ulong;
  pub fn XkbKeysymToModifiers (_2: *mut Display, _1: c_ulong) -> c_uint;
  pub fn XkbKeyTypesForCoreSymbols (_6: XkbDescPtr, _5: c_int, _4: *mut c_ulong, _3: c_uint, _2: *mut c_int, _1: *mut c_ulong) -> c_int;
  pub fn XkbLatchGroup (_3: *mut Display, _2: c_uint, _1: c_uint) -> c_int;
  pub fn XkbLatchModifiers (_4: *mut Display, _3: c_uint, _2: c_uint, _1: c_uint) -> c_int;
  pub fn XkbLibraryVersion (_2: *mut c_int, _1: *mut c_int) -> c_int;
  pub fn XkbListComponents (_4: *mut Display, _3: c_uint, _2: XkbComponentNamesPtr, _1: *mut c_int) -> XkbComponentListPtr;
  pub fn XkbLockGroup (_3: *mut Display, _2: c_uint, _1: c_uint) -> c_int;
  pub fn XkbLockModifiers (_4: *mut Display, _3: c_uint, _2: c_uint, _1: c_uint) -> c_int;
  pub fn XkbLookupKeyBinding (_6: *mut Display, _5: c_ulong, _4: c_uint, _3: *mut c_char, _2: c_int, _1: *mut c_int) -> c_int;
  pub fn XkbLookupKeySym (_5: *mut Display, _4: c_uchar, _3: c_uint, _2: *mut c_uint, _1: *mut c_ulong) -> c_int;
  pub fn XkbNoteControlsChanges (_3: XkbControlsChangesPtr, _2: *mut XkbControlsNotifyEvent, _1: c_uint);
  pub fn XkbNoteDeviceChanges (_3: XkbDeviceChangesPtr, _2: *mut XkbExtensionDeviceNotifyEvent, _1: c_uint);
  pub fn XkbNoteMapChanges (_3: XkbMapChangesPtr, _2: *mut XkbMapNotifyEvent, _1: c_uint);
  pub fn XkbNoteNameChanges (_3: XkbNameChangesPtr, _2: *mut XkbNamesNotifyEvent, _1: c_uint);
  pub fn XkbOpenDisplay (_6: *mut c_char, _5: *mut c_int, _4: *mut c_int, _3: *mut c_int, _2: *mut c_int, _1: *mut c_int) -> *mut Display;
  pub fn XkbQueryExtension (_6: *mut Display, _5: *mut c_int, _4: *mut c_int, _3: *mut c_int, _2: *mut c_int, _1: *mut c_int) -> c_int;
  pub fn XkbRefreshKeyboardMapping (_1: *mut XkbMapNotifyEvent) -> c_int;
  pub fn XkbResizeDeviceButtonActions (_2: XkbDeviceInfoPtr, _1: c_uint) -> c_int;
  pub fn XkbResizeKeyActions (_3: XkbDescPtr, _2: c_int, _1: c_int) -> *mut XkbAction;
  pub fn XkbResizeKeySyms (_3: XkbDescPtr, _2: c_int, _1: c_int) -> *mut c_ulong;
  pub fn XkbResizeKeyType (_5: XkbDescPtr, _4: c_int, _3: c_int, _2: c_int, _1: c_int) -> c_int;
  pub fn XkbSelectEventDetails (_5: *mut Display, _4: c_uint, _3: c_uint, _2: c_ulong, _1: c_ulong) -> c_int;
  pub fn XkbSelectEvents (_4: *mut Display, _3: c_uint, _2: c_uint, _1: c_uint) -> c_int;
  pub fn XkbSetAtomFuncs (_2: Option<unsafe extern "C" fn (*mut Display, *const c_char, c_int) -> c_ulong>, _1: Option<unsafe extern "C" fn (*mut Display, c_ulong) -> *mut c_char>);
  pub fn XkbSetAutoRepeatRate (_4: *mut Display, _3: c_uint, _2: c_uint, _1: c_uint) -> c_int;
  pub fn XkbSetAutoResetControls (_4: *mut Display, _3: c_uint, _2: *mut c_uint, _1: *mut c_uint) -> c_int;
  pub fn XkbSetCompatMap (_4: *mut Display, _3: c_uint, _2: XkbDescPtr, _1: c_int) -> c_int;
  pub fn XkbSetControls (_3: *mut Display, _2: c_ulong, _1: XkbDescPtr) -> c_int;
  pub fn XkbSetDebuggingFlags (_8: *mut Display, _7: c_uint, _6: c_uint, _5: *mut c_char, _4: c_uint, _3: c_uint, _2: *mut c_uint, _1: *mut c_uint) -> c_int;
  pub fn XkbSetDetectableAutoRepeat (_3: *mut Display, _2: c_int, _1: *mut c_int) -> c_int;
  pub fn XkbSetDeviceButtonActions (_4: *mut Display, _3: XkbDeviceInfoPtr, _2: c_uint, _1: c_uint) -> c_int;
  pub fn XkbSetDeviceInfo (_3: *mut Display, _2: c_uint, _1: XkbDeviceInfoPtr) -> c_int;
  pub fn XkbSetDeviceLedInfo (_5: *mut Display, _4: XkbDeviceInfoPtr, _3: c_uint, _2: c_uint, _1: c_uint) -> c_int;
  pub fn XkbSetGeometry (_3: *mut Display, _2: c_uint, _1: XkbGeometryPtr) -> c_int;
  pub fn XkbSetIgnoreLockMods (_6: *mut Display, _5: c_uint, _4: c_uint, _3: c_uint, _2: c_uint, _1: c_uint) -> c_int;
  pub fn XkbSetIndicatorMap (_3: *mut Display, _2: c_ulong, _1: XkbDescPtr) -> c_int;
  pub fn XkbSetMap (_3: *mut Display, _2: c_uint, _1: XkbDescPtr) -> c_int;
  pub fn XkbSetNamedDeviceIndicator (_9: *mut Display, _8: c_uint, _7: c_uint, _6: c_uint, _5: c_ulong, _4: c_int, _3: c_int, _2: c_int, _1: XkbIndicatorMapPtr) -> c_int;
  pub fn XkbSetNamedIndicator (_6: *mut Display, _5: c_ulong, _4: c_int, _3: c_int, _2: c_int, _1: XkbIndicatorMapPtr) -> c_int;
  pub fn XkbSetNames (_5: *mut Display, _4: c_uint, _3: c_uint, _2: c_uint, _1: XkbDescPtr) -> c_int;
  pub fn XkbSetPerClientControls (_3: *mut Display, _2: c_uint, _1: *mut c_uint) -> c_int;
  pub fn XkbSetServerInternalMods (_6: *mut Display, _5: c_uint, _4: c_uint, _3: c_uint, _2: c_uint, _1: c_uint) -> c_int;
  pub fn XkbSetXlibControls (_3: *mut Display, _2: c_uint, _1: c_uint) -> c_uint;
  pub fn XkbToControl (_1: c_char) -> c_char;
  pub fn XkbTranslateKeyCode (_5: XkbDescPtr, _4: c_uchar, _3: c_uint, _2: *mut c_uint, _1: *mut c_ulong) -> c_int;
  pub fn XkbTranslateKeySym (_6: *mut Display, _5: *mut c_ulong, _4: c_uint, _3: *mut c_char, _2: c_int, _1: *mut c_int) -> c_int;
  pub fn XkbUpdateActionVirtualMods (_3: XkbDescPtr, _2: *mut XkbAction, _1: c_uint) -> c_int;
  pub fn XkbUpdateKeyTypeVirtualMods (_4: XkbDescPtr, _3: XkbKeyTypePtr, _2: c_uint, _1: XkbChangesPtr);
  pub fn XkbUpdateMapFromCore (_6: XkbDescPtr, _5: c_uchar, _4: c_int, _3: c_int, _2: *mut c_ulong, _1: XkbChangesPtr) -> c_int;
  pub fn XkbUseExtension (_3: *mut Display, _2: *mut c_int, _1: *mut c_int) -> c_int;
  pub fn XkbVirtualModsToReal (_3: XkbDescPtr, _2: c_uint, _1: *mut c_uint) -> c_int;
  pub fn XkbXlibControlsImplemented () -> c_uint;
}


//
// types
//


// TODO structs
#[repr(C)] pub struct _XkbAction;
#[repr(C)] pub struct _XkbBounds;
#[repr(C)] pub struct _XkbChanges;
#[repr(C)] pub struct _XkbClientMapRec;
#[repr(C)] pub struct _XkbColor;
#[repr(C)] pub struct _XkbComponentList;
#[repr(C)] pub struct _XkbComponentNames;
#[repr(C)] pub struct _XkbControls;
#[repr(C)] pub struct _XkbControlsChanges;
#[repr(C)] pub struct _XkbControlsNotify;
#[repr(C)] pub struct _XkbDeviceChanges;
#[repr(C)] pub struct _XkbDeviceInfo;
#[repr(C)] pub struct _XkbDeviceLedInfo;
#[repr(C)] pub struct _XkbDoodad;
#[repr(C)] pub struct _XkbExtensionDeviceNotify;
#[repr(C)] pub struct _XkbGeometry;
#[repr(C)] pub struct _XkbGeometrySizes;
#[repr(C)] pub struct _XkbIndicatorMapRec;
#[repr(C)] pub struct _XkbKey;
#[repr(C)] pub struct _XkbKeyType;
#[repr(C)] pub struct _XkbMapChanges;
#[repr(C)] pub struct _XkbMapNotifyEvent;
#[repr(C)] pub struct _XkbMods;
#[repr(C)] pub struct _XkbNameChanges;
#[repr(C)] pub struct _XkbNamesNotify;
#[repr(C)] pub struct _XkbOutline;
#[repr(C)] pub struct _XkbOverlay;
#[repr(C)] pub struct _XkbOverlayKey;
#[repr(C)] pub struct _XkbOverlayRow;
#[repr(C)] pub struct _XkbProperty;
#[repr(C)] pub struct _XkbRow;
#[repr(C)] pub struct _XkbSection;
#[repr(C)] pub struct _XkbServerMapRec;
#[repr(C)] pub struct _XkbShape;
#[repr(C)] pub struct _XkbStateRec;
#[repr(C)] pub struct _XkbSymInterpretRec;

// types
pub type XkbAction = _XkbAction;
pub type XkbBoundsPtr = *mut _XkbBounds;
pub type XkbChangesPtr = *mut _XkbChanges;
pub type XkbClientMapPtr = *mut _XkbClientMapRec;
pub type XkbColorPtr = *mut _XkbColor;
pub type XkbCompatMapPtr = *mut _XkbCompatMapRec;
pub type XkbComponentListPtr = *mut _XkbComponentList;
pub type XkbComponentNamesPtr = *mut _XkbComponentNames;
pub type XkbControlsChangesPtr = *mut _XkbControlsChanges;
pub type XkbControlsNotifyEvent = _XkbControlsNotify;
pub type XkbControlsPtr = *mut _XkbControls;
pub type XkbDescPtr = *mut _XkbDesc;
pub type XkbDeviceChangesPtr = *mut _XkbDeviceChanges;
pub type XkbDeviceInfoPtr = *mut _XkbDeviceInfo;
pub type XkbDeviceLedInfoPtr = *mut _XkbDeviceLedInfo;
pub type XkbDoodadPtr = *mut _XkbDoodad;
pub type XkbExtensionDeviceNotifyEvent = _XkbExtensionDeviceNotify;
pub type XkbGeometryPtr = *mut _XkbGeometry;
pub type XkbGeometrySizesPtr = *mut _XkbGeometrySizes;
pub type XkbIndicatorMapPtr = *mut _XkbIndicatorMapRec;
pub type XkbIndicatorMapRec = _XkbIndicatorMapRec;
pub type XkbIndicatorPtr = *mut _XkbIndicatorRec;
pub type XkbKeyTypePtr = *mut _XkbKeyType;
pub type XkbMapChangesPtr = *mut _XkbMapChanges;
pub type XkbMapNotifyEvent = _XkbMapNotifyEvent;
pub type XkbModsPtr = *mut _XkbMods;
pub type XkbModsRec = _XkbMods;
pub type XkbNameChangesPtr = *mut _XkbNameChanges;
pub type XkbNamesNotifyEvent = _XkbNamesNotify;
pub type XkbNamesPtr = *mut _XkbNamesRec;
pub type XkbKeyAliasPtr = *mut _XkbKeyAliasRec;
pub type XkbKeyNamePtr = *mut _XkbKeyNameRec;
pub type XkbKeyPtr = *mut _XkbKey;
pub type XkbOutlinePtr = *mut _XkbOutline;
pub type XkbOverlayKeyPtr = *mut _XkbOverlayKey;
pub type XkbOverlayPtr = *mut _XkbOverlay;
pub type XkbOverlayRowPtr = *mut _XkbOverlayRow;
pub type XkbPropertyPtr = *mut _XkbProperty;
pub type XkbRowPtr = *mut _XkbRow;
pub type XkbSectionPtr = *mut _XkbSection;
pub type XkbServerMapPtr = *mut _XkbServerMapRec;
pub type XkbShapePtr = *mut _XkbShape;
pub type XkbStatePtr = *mut _XkbStateRec;
pub type XkbSymInterpretPtr = *mut _XkbSymInterpretRec;


//
// structs
//


#[allow(raw_pointer_derive)]
#[repr(C)]
pub struct _XkbCompatMapRec {
  pub sym_interpret: XkbSymInterpretPtr,
  pub groups: [XkbModsRec; XkbNumKbdGroups],
  pub num_si: c_ushort,
  pub size_si: c_ushort,
}

#[allow(raw_pointer_derive)]
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct _XkbDesc {
  pub dpy: *mut Display,
  pub flags: c_ushort,
  pub device_spec: c_ushort,
  pub min_key_code: KeyCode,
  pub max_key_code: KeyCode,
  pub ctrls: XkbControlsPtr,
  pub server: XkbServerMapPtr,
  pub map: XkbClientMapPtr,
  pub indicators: XkbIndicatorPtr,
  pub names: XkbNamesPtr,
  pub compat: XkbCompatMapPtr,
  pub geom: XkbGeometryPtr,
}

#[repr(C)]
pub struct _XkbIndicatorRec {
  pub phys_indicators: c_ulong,
  pub maps: [XkbIndicatorMapRec; XkbNumIndicators],
}

#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct _XkbKeyAliasRec {
  pub real: [c_char; XkbKeyNameLength],
  pub alias: [c_char; XkbKeyNameLength],
}

#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct _XkbKeyNameRec {
  pub name: [c_char; XkbKeyNameLength],
}

#[allow(raw_pointer_derive)]
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct _XkbNamesRec {
  pub keycodes: Atom,
  pub geometry: Atom,
  pub symbols: Atom,
  pub types: Atom,
  pub compat: Atom,
  pub vmods: [Atom; XkbNumVirtualMods],
  pub indicators: [Atom; XkbNumIndicators],
  pub groups: [Atom; XkbNumKbdGroups],
  pub keys: XkbKeyNamePtr,
  pub key_aliases: XkbKeyAliasPtr,
  pub radio_groups: *mut Atom,
  pub phys_symbols: Atom,
  pub num_keys: c_uchar,
  pub num_key_aliases: c_uchar,
  pub num_rg: c_ushort,
}


//
// constants
//


// array sizes
pub const XkbKeyNameLength: usize = 4;
pub const XkbNumIndicators: usize = 32;
pub const XkbNumKbdGroups: usize = 4;
pub const XkbNumVirtualMods: usize = 16;
