// x11-rs: Rust bindings for X11 libraries
// The X11 libraries are available under the MIT license.
// These bindings are public domain.

use std::cmp::min;
use std::convert::From;
use std::mem::{
  size_of,
  zeroed,
};
use std::slice::{
  from_raw_parts,
  from_raw_parts_mut,
};

use libc::{
  c_char,
  c_int,
  c_long,
  c_short,
  c_uchar,
  c_uint,
  c_ulong,
  c_ushort,
  c_void,
  wchar_t,
};

pub mod xcms;
pub mod xkb;

//pub use self::xcms::*;
//pub use self::xkb::*;


//
// functions
//


#[link(name="X11")]
extern "C" {
  pub fn XActivateScreenSaver (_1: *mut Display) -> c_int;
  pub fn XAddConnectionWatch (_3: *mut Display, _2: Option<unsafe extern "C" fn (*mut Display, *mut c_char, c_int, c_int, *mut *mut c_char) -> ()>, _1: *mut c_char) -> c_int;
  pub fn XAddExtension (_1: *mut Display) -> *mut XExtCodes;
  pub fn XAddHost (_2: *mut Display, _1: *mut XHostAddress) -> c_int;
  pub fn XAddHosts (_3: *mut Display, _2: *mut XHostAddress, _1: c_int) -> c_int;
  pub fn XAddPixel (_2: *mut XImage, _1: c_long) -> c_int;
  pub fn XAddToExtensionList (_2: *mut *mut XExtData, _1: *mut XExtData) -> c_int;
  pub fn XAddToSaveSet (_2: *mut Display, _1: c_ulong) -> c_int;
  pub fn XAllocClassHint () -> *mut XClassHint;
  pub fn XAllocColor (_3: *mut Display, _2: c_ulong, _1: *mut XColor) -> c_int;
  pub fn XAllocColorCells (_7: *mut Display, _6: c_ulong, _5: c_int, _4: *mut c_ulong, _3: c_uint, _2: *mut c_ulong, _1: c_uint) -> c_int;
  pub fn XAllocColorPlanes (_11: *mut Display, _10: c_ulong, _9: c_int, _8: *mut c_ulong, _7: c_int, _6: c_int, _5: c_int, _4: c_int, _3: *mut c_ulong, _2: *mut c_ulong, _1: *mut c_ulong) -> c_int;
  pub fn XAllocIconSize () -> *mut XIconSize;
  pub fn XAllocNamedColor (_5: *mut Display, _4: c_ulong, _3: *const c_char, _2: *mut XColor, _1: *mut XColor) -> c_int;
  pub fn XAllocSizeHints () -> *mut XSizeHints;
  pub fn XAllocStandardColormap () -> *mut XStandardColormap;
  pub fn XAllocWMHints () -> *mut XWMHints;
  pub fn XAllowEvents (_3: *mut Display, _2: c_int, _1: c_ulong) -> c_int;
  pub fn XAllPlanes () -> c_ulong;
  pub fn XAutoRepeatOff (_1: *mut Display) -> c_int;
  pub fn XAutoRepeatOn (_1: *mut Display) -> c_int;
  pub fn XBaseFontNameListOfFontSet (_1: XFontSet) -> *mut c_char;
  pub fn XBell (_2: *mut Display, _1: c_int) -> c_int;
  pub fn XBitmapBitOrder (_1: *mut Display) -> c_int;
  pub fn XBitmapPad (_1: *mut Display) -> c_int;
  pub fn XBitmapUnit (_1: *mut Display) -> c_int;
  pub fn XBlackPixel (_2: *mut Display, _1: c_int) -> c_ulong;
  pub fn XBlackPixelOfScreen (_1: *mut Screen) -> c_ulong;
  pub fn XCellsOfScreen (_1: *mut Screen) -> c_int;
  pub fn XChangeActivePointerGrab (_4: *mut Display, _3: c_uint, _2: c_ulong, _1: c_ulong) -> c_int;
  pub fn XChangeGC (_4: *mut Display, _3: GC, _2: c_ulong, _1: *mut XGCValues) -> c_int;
  pub fn XChangeKeyboardControl (_3: *mut Display, _2: c_ulong, _1: *mut XKeyboardControl) -> c_int;
  pub fn XChangeKeyboardMapping (_5: *mut Display, _4: c_int, _3: c_int, _2: *mut c_ulong, _1: c_int) -> c_int;
  pub fn XChangePointerControl (_6: *mut Display, _5: c_int, _4: c_int, _3: c_int, _2: c_int, _1: c_int) -> c_int;
  pub fn XChangeProperty (_8: *mut Display, _7: c_ulong, _6: c_ulong, _5: c_ulong, _4: c_int, _3: c_int, _2: *const c_uchar, _1: c_int) -> c_int;
  pub fn XChangeSaveSet (_3: *mut Display, _2: c_ulong, _1: c_int) -> c_int;
  pub fn XChangeWindowAttributes (_4: *mut Display, _3: c_ulong, _2: c_ulong, _1: *mut XSetWindowAttributes) -> c_int;
  pub fn XCheckIfEvent (_4: *mut Display, _3: *mut XEvent, _2: Option<unsafe extern "C" fn (*mut Display, *mut XEvent, *mut c_char) -> c_int>, _1: *mut c_char) -> c_int;
  pub fn XCheckMaskEvent (_3: *mut Display, _2: c_long, _1: *mut XEvent) -> c_int;
  pub fn XCheckTypedEvent (_3: *mut Display, _2: c_int, _1: *mut XEvent) -> c_int;
  pub fn XCheckTypedWindowEvent (_4: *mut Display, _3: c_ulong, _2: c_int, _1: *mut XEvent) -> c_int;
  pub fn XCheckWindowEvent (_4: *mut Display, _3: c_ulong, _2: c_long, _1: *mut XEvent) -> c_int;
  pub fn XCirculateSubwindows (_3: *mut Display, _2: c_ulong, _1: c_int) -> c_int;
  pub fn XCirculateSubwindowsDown (_2: *mut Display, _1: c_ulong) -> c_int;
  pub fn XCirculateSubwindowsUp (_2: *mut Display, _1: c_ulong) -> c_int;
  pub fn XClearArea (_7: *mut Display, _6: c_ulong, _5: c_int, _4: c_int, _3: c_uint, _2: c_uint, _1: c_int) -> c_int;
  pub fn XClearWindow (_2: *mut Display, _1: c_ulong) -> c_int;
  pub fn XClipBox (_2: Region, _1: *mut XRectangle) -> c_int;
  pub fn XCloseDisplay (_1: *mut Display) -> c_int;
  pub fn XCloseIM (_1: XIM) -> c_int;
  pub fn XCloseOM (_1: XOM) -> c_int;
  pub fn XConfigureWindow (_4: *mut Display, _3: c_ulong, _2: c_uint, _1: *mut XWindowChanges) -> c_int;
  pub fn XConnectionNumber (_1: *mut Display) -> c_int;
  pub fn XContextDependentDrawing (_1: XFontSet) -> c_int;
  pub fn XContextualDrawing (_1: XFontSet) -> c_int;
  pub fn XConvertCase (_3: c_ulong, _2: *mut c_ulong, _1: *mut c_ulong) -> ();
  pub fn XConvertSelection (_6: *mut Display, _5: c_ulong, _4: c_ulong, _3: c_ulong, _2: c_ulong, _1: c_ulong) -> c_int;
  pub fn XCopyArea (_10: *mut Display, _9: c_ulong, _8: c_ulong, _7: GC, _6: c_int, _5: c_int, _4: c_uint, _3: c_uint, _2: c_int, _1: c_int) -> c_int;
  pub fn XCopyColormapAndFree (_2: *mut Display, _1: c_ulong) -> c_ulong;
  pub fn XCopyGC (_4: *mut Display, _3: GC, _2: c_ulong, _1: GC) -> c_int;
  pub fn XCopyPlane (_11: *mut Display, _10: c_ulong, _9: c_ulong, _8: GC, _7: c_int, _6: c_int, _5: c_uint, _4: c_uint, _3: c_int, _2: c_int, _1: c_ulong) -> c_int;
  pub fn XCreateBitmapFromData (_5: *mut Display, _4: c_ulong, _3: *const c_char, _2: c_uint, _1: c_uint) -> c_ulong;
  pub fn XCreateColormap (_4: *mut Display, _3: c_ulong, _2: *mut Visual, _1: c_int) -> c_ulong;
  pub fn XCreateFontCursor (_2: *mut Display, _1: c_uint) -> c_ulong;
  pub fn XCreateFontSet (_5: *mut Display, _4: *const c_char, _3: *mut *mut *mut c_char, _2: *mut c_int, _1: *mut *mut c_char) -> XFontSet;
  pub fn XCreateGC (_4: *mut Display, _3: c_ulong, _2: c_ulong, _1: *mut XGCValues) -> GC;
  pub fn XCreateGlyphCursor (_7: *mut Display, _6: c_ulong, _5: c_ulong, _4: c_uint, _3: c_uint, _2: *const XColor, _1: *const XColor) -> c_ulong;
  pub fn XCreateIC (_1: XIM, ...) -> XIC;
  pub fn XCreateImage (_10: *mut Display, _9: *mut Visual, _8: c_uint, _7: c_int, _6: c_int, _5: *mut c_char, _4: c_uint, _3: c_uint, _2: c_int, _1: c_int) -> *mut XImage;
  pub fn XCreateOC (_1: XOM, ...) -> XFontSet;
  pub fn XCreatePixmap (_5: *mut Display, _4: c_ulong, _3: c_uint, _2: c_uint, _1: c_uint) -> c_ulong;
  pub fn XCreatePixmapCursor (_7: *mut Display, _6: c_ulong, _5: c_ulong, _4: *mut XColor, _3: *mut XColor, _2: c_uint, _1: c_uint) -> c_ulong;
  pub fn XCreatePixmapFromBitmapData (_8: *mut Display, _7: c_ulong, _6: *mut c_char, _5: c_uint, _4: c_uint, _3: c_ulong, _2: c_ulong, _1: c_uint) -> c_ulong;
  pub fn XCreateRegion () -> Region;
  pub fn XCreateSimpleWindow (_9: *mut Display, _8: c_ulong, _7: c_int, _6: c_int, _5: c_uint, _4: c_uint, _3: c_uint, _2: c_ulong, _1: c_ulong) -> c_ulong;
  pub fn XCreateWindow (_12: *mut Display, _11: c_ulong, _10: c_int, _9: c_int, _8: c_uint, _7: c_uint, _6: c_uint, _5: c_int, _4: c_uint, _3: *mut Visual, _2: c_ulong, _1: *mut XSetWindowAttributes) -> c_ulong;
  pub fn XDefaultColormap (_2: *mut Display, _1: c_int) -> c_ulong;
  pub fn XDefaultColormapOfScreen (_1: *mut Screen) -> c_ulong;
  pub fn XDefaultDepth (_2: *mut Display, _1: c_int) -> c_int;
  pub fn XDefaultDepthOfScreen (_1: *mut Screen) -> c_int;
  pub fn XDefaultGC (_2: *mut Display, _1: c_int) -> GC;
  pub fn XDefaultGCOfScreen (_1: *mut Screen) -> GC;
  pub fn XDefaultRootWindow (_1: *mut Display) -> c_ulong;
  pub fn XDefaultScreen (_1: *mut Display) -> c_int;
  pub fn XDefaultScreenOfDisplay (_1: *mut Display) -> *mut Screen;
  pub fn XDefaultString () -> *const c_char;
  pub fn XDefaultVisual (_2: *mut Display, _1: c_int) -> *mut Visual;
  pub fn XDefaultVisualOfScreen (_1: *mut Screen) -> *mut Visual;
  pub fn XDefineCursor (_3: *mut Display, _2: c_ulong, _1: c_ulong) -> c_int;
  pub fn XDeleteContext (_3: *mut Display, _2: c_ulong, _1: c_int) -> c_int;
  pub fn XDeleteModifiermapEntry (_3: *mut XModifierKeymap, _2: c_uchar, _1: c_int) -> *mut XModifierKeymap;
  pub fn XDeleteProperty (_3: *mut Display, _2: c_ulong, _1: c_ulong) -> c_int;
  pub fn XDestroyIC (_1: XIC) -> ();
  pub fn XDestroyImage (_1: *mut XImage) -> c_int;
  pub fn XDestroyOC (_1: XFontSet) -> ();
  pub fn XDestroyRegion (_1: Region) -> c_int;
  pub fn XDestroySubwindows (_2: *mut Display, _1: c_ulong) -> c_int;
  pub fn XDestroyWindow (_2: *mut Display, _1: c_ulong) -> c_int;
  pub fn XDirectionalDependentDrawing (_1: XFontSet) -> c_int;
  pub fn XDisableAccessControl (_1: *mut Display) -> c_int;
  pub fn XDisplayCells (_2: *mut Display, _1: c_int) -> c_int;
  pub fn XDisplayHeight (_2: *mut Display, _1: c_int) -> c_int;
  pub fn XDisplayHeightMM (_2: *mut Display, _1: c_int) -> c_int;
  pub fn XDisplayKeycodes (_3: *mut Display, _2: *mut c_int, _1: *mut c_int) -> c_int;
  pub fn XDisplayMotionBufferSize (_1: *mut Display) -> c_ulong;
  pub fn XDisplayName (_1: *const c_char) -> *mut c_char;
  pub fn XDisplayOfIM (_1: XIM) -> *mut Display;
  pub fn XDisplayOfOM (_1: XOM) -> *mut Display;
  pub fn XDisplayOfScreen (_1: *mut Screen) -> *mut Display;
  pub fn XDisplayPlanes (_2: *mut Display, _1: c_int) -> c_int;
  pub fn XDisplayString (_1: *mut Display) -> *mut c_char;
  pub fn XDisplayWidth (_2: *mut Display, _1: c_int) -> c_int;
  pub fn XDisplayWidthMM (_2: *mut Display, _1: c_int) -> c_int;
  pub fn XDoesBackingStore (_1: *mut Screen) -> c_int;
  pub fn XDoesSaveUnders (_1: *mut Screen) -> c_int;
  pub fn XDrawArc (_9: *mut Display, _8: c_ulong, _7: GC, _6: c_int, _5: c_int, _4: c_uint, _3: c_uint, _2: c_int, _1: c_int) -> c_int;
  pub fn XDrawArcs (_5: *mut Display, _4: c_ulong, _3: GC, _2: *mut XArc, _1: c_int) -> c_int;
  pub fn XDrawImageString (_7: *mut Display, _6: c_ulong, _5: GC, _4: c_int, _3: c_int, _2: *const c_char, _1: c_int) -> c_int;
  pub fn XDrawImageString16 (_7: *mut Display, _6: c_ulong, _5: GC, _4: c_int, _3: c_int, _2: *const XChar2b, _1: c_int) -> c_int;
  pub fn XDrawLine (_7: *mut Display, _6: c_ulong, _5: GC, _4: c_int, _3: c_int, _2: c_int, _1: c_int) -> c_int;
  pub fn XDrawLines (_6: *mut Display, _5: c_ulong, _4: GC, _3: *mut XPoint, _2: c_int, _1: c_int) -> c_int;
  pub fn XDrawPoint (_5: *mut Display, _4: c_ulong, _3: GC, _2: c_int, _1: c_int) -> c_int;
  pub fn XDrawPoints (_6: *mut Display, _5: c_ulong, _4: GC, _3: *mut XPoint, _2: c_int, _1: c_int) -> c_int;
  pub fn XDrawRectangle (_7: *mut Display, _6: c_ulong, _5: GC, _4: c_int, _3: c_int, _2: c_uint, _1: c_uint) -> c_int;
  pub fn XDrawRectangles (_5: *mut Display, _4: c_ulong, _3: GC, _2: *mut XRectangle, _1: c_int) -> c_int;
  pub fn XDrawSegments (_5: *mut Display, _4: c_ulong, _3: GC, _2: *mut XSegment, _1: c_int) -> c_int;
  pub fn XDrawString (_7: *mut Display, _6: c_ulong, _5: GC, _4: c_int, _3: c_int, _2: *const c_char, _1: c_int) -> c_int;
  pub fn XDrawString16 (_7: *mut Display, _6: c_ulong, _5: GC, _4: c_int, _3: c_int, _2: *const XChar2b, _1: c_int) -> c_int;
  pub fn XDrawText (_7: *mut Display, _6: c_ulong, _5: GC, _4: c_int, _3: c_int, _2: *mut XTextItem, _1: c_int) -> c_int;
  pub fn XDrawText16 (_7: *mut Display, _6: c_ulong, _5: GC, _4: c_int, _3: c_int, _2: *mut XTextItem16, _1: c_int) -> c_int;
  pub fn XEHeadOfExtensionList (_1: XEDataObject) -> *mut *mut XExtData;
  pub fn XEmptyRegion (_1: Region) -> c_int;
  pub fn XEnableAccessControl (_1: *mut Display) -> c_int;
  pub fn XEqualRegion (_2: Region, _1: Region) -> c_int;
  pub fn XESetBeforeFlush (_3: *mut Display, _2: c_int, _1: Option<unsafe extern "C" fn (*mut Display, *mut XExtCodes, *const c_char, c_long) -> ()>) -> Option<unsafe extern "C" fn (*mut Display, *mut XExtCodes, *const c_char, c_long) -> ()>;
  pub fn XESetCloseDisplay (_3: *mut Display, _2: c_int, _1: Option<unsafe extern "C" fn (*mut Display, *mut XExtCodes) -> c_int>) -> Option<unsafe extern "C" fn (*mut Display, *mut XExtCodes) -> c_int>;
  pub fn XESetCopyEventCookie (_3: *mut Display, _2: c_int, _1: Option<unsafe extern "C" fn (*mut Display, *mut XGenericEventCookie, *mut XGenericEventCookie) -> c_int>) -> Option<unsafe extern "C" fn (*mut Display, *mut XGenericEventCookie, *mut XGenericEventCookie) -> c_int>;
  pub fn XESetCopyGC (_3: *mut Display, _2: c_int, _1: Option<unsafe extern "C" fn (*mut Display, GC, *mut XExtCodes) -> c_int>) -> Option<unsafe extern "C" fn (*mut Display, GC, *mut XExtCodes) -> c_int>;
  pub fn XESetCreateFont (_3: *mut Display, _2: c_int, _1: Option<unsafe extern "C" fn (*mut Display, *mut XFontStruct, *mut XExtCodes) -> c_int>) -> Option<unsafe extern "C" fn (*mut Display, *mut XFontStruct, *mut XExtCodes) -> c_int>;
  pub fn XESetCreateGC (_3: *mut Display, _2: c_int, _1: Option<unsafe extern "C" fn (*mut Display, GC, *mut XExtCodes) -> c_int>) -> Option<unsafe extern "C" fn (*mut Display, GC, *mut XExtCodes) -> c_int>;
  pub fn XESetError (_3: *mut Display, _2: c_int, _1: Option<unsafe extern "C" fn (*mut Display, *mut xError, *mut XExtCodes, *mut c_int) -> c_int>) -> Option<unsafe extern "C" fn (*mut Display, *mut xError, *mut XExtCodes, *mut c_int) -> c_int>;
  pub fn XESetErrorString (_3: *mut Display, _2: c_int, _1: Option<unsafe extern "C" fn (*mut Display, c_int, *mut XExtCodes, *mut c_char, c_int) -> *mut c_char>) -> Option<unsafe extern "C" fn (*mut Display, c_int, *mut XExtCodes, *mut c_char, c_int) -> *mut c_char>;
  pub fn XESetEventToWire (_3: *mut Display, _2: c_int, _1: Option<unsafe extern "C" fn (*mut Display, *mut XEvent, *mut xEvent) -> c_int>) -> Option<unsafe extern "C" fn (*mut Display, *mut XEvent, *mut xEvent) -> c_int>;
  pub fn XESetFlushGC (_3: *mut Display, _2: c_int, _1: Option<unsafe extern "C" fn (*mut Display, GC, *mut XExtCodes) -> c_int>) -> Option<unsafe extern "C" fn (*mut Display, GC, *mut XExtCodes) -> c_int>;
  pub fn XESetFreeFont (_3: *mut Display, _2: c_int, _1: Option<unsafe extern "C" fn (*mut Display, *mut XFontStruct, *mut XExtCodes) -> c_int>) -> Option<unsafe extern "C" fn (*mut Display, *mut XFontStruct, *mut XExtCodes) -> c_int>;
  pub fn XESetFreeGC (_3: *mut Display, _2: c_int, _1: Option<unsafe extern "C" fn (*mut Display, GC, *mut XExtCodes) -> c_int>) -> Option<unsafe extern "C" fn (*mut Display, GC, *mut XExtCodes) -> c_int>;
  pub fn XESetPrintErrorValues (_3: *mut Display, _2: c_int, _1: Option<unsafe extern "C" fn (*mut Display, *mut XErrorEvent, *mut ()) -> ()>) -> Option<unsafe extern "C" fn (*mut Display, *mut XErrorEvent, *mut ()) -> ()>;
  pub fn XESetWireToError (_3: *mut Display, _2: c_int, _1: Option<unsafe extern "C" fn (*mut Display, *mut XErrorEvent, *mut xError) -> c_int>) -> Option<unsafe extern "C" fn (*mut Display, *mut XErrorEvent, *mut xError) -> c_int>;
  pub fn XESetWireToEvent (_3: *mut Display, _2: c_int, _1: Option<unsafe extern "C" fn (*mut Display, *mut XEvent, *mut xEvent) -> c_int>) -> Option<unsafe extern "C" fn (*mut Display, *mut XEvent, *mut xEvent) -> c_int>;
  pub fn XESetWireToEventCookie (_3: *mut Display, _2: c_int, _1: Option<unsafe extern "C" fn (*mut Display, *mut XGenericEventCookie, *mut xEvent) -> c_int>) -> Option<unsafe extern "C" fn (*mut Display, *mut XGenericEventCookie, *mut xEvent) -> c_int>;
  pub fn XEventMaskOfScreen (_1: *mut Screen) -> c_long;
  pub fn XEventsQueued (_2: *mut Display, _1: c_int) -> c_int;
  pub fn XExtendedMaxRequestSize (_1: *mut Display) -> c_long;
  pub fn XExtentsOfFontSet (_1: XFontSet) -> *mut XFontSetExtents;
  pub fn XFetchBuffer (_3: *mut Display, _2: *mut c_int, _1: c_int) -> *mut c_char;
  pub fn XFetchBytes (_2: *mut Display, _1: *mut c_int) -> *mut c_char;
  pub fn XFetchName (_3: *mut Display, _2: c_ulong, _1: *mut *mut c_char) -> c_int;
  pub fn XFillArc (_9: *mut Display, _8: c_ulong, _7: GC, _6: c_int, _5: c_int, _4: c_uint, _3: c_uint, _2: c_int, _1: c_int) -> c_int;
  pub fn XFillArcs (_5: *mut Display, _4: c_ulong, _3: GC, _2: *mut XArc, _1: c_int) -> c_int;
  pub fn XFillPolygon (_7: *mut Display, _6: c_ulong, _5: GC, _4: *mut XPoint, _3: c_int, _2: c_int, _1: c_int) -> c_int;
  pub fn XFillRectangle (_7: *mut Display, _6: c_ulong, _5: GC, _4: c_int, _3: c_int, _2: c_uint, _1: c_uint) -> c_int;
  pub fn XFillRectangles (_5: *mut Display, _4: c_ulong, _3: GC, _2: *mut XRectangle, _1: c_int) -> c_int;
  pub fn XFilterEvent (_2: *mut XEvent, _1: c_ulong) -> c_int;
  pub fn XFindContext (_4: *mut Display, _3: c_ulong, _2: c_int, _1: *mut *mut c_char) -> c_int;
  pub fn XFindOnExtensionList (_2: *mut *mut XExtData, _1: c_int) -> *mut XExtData;
  pub fn XFlush (_1: *mut Display) -> c_int;
  pub fn XFlushGC (_2: *mut Display, _1: GC) -> ();
  pub fn XFontsOfFontSet (_3: XFontSet, _2: *mut *mut *mut XFontStruct, _1: *mut *mut *mut c_char) -> c_int;
  pub fn XForceScreenSaver (_2: *mut Display, _1: c_int) -> c_int;
  pub fn XFree (_1: *mut ()) -> c_int;
  pub fn XFreeColormap (_2: *mut Display, _1: c_ulong) -> c_int;
  pub fn XFreeColors (_5: *mut Display, _4: c_ulong, _3: *mut c_ulong, _2: c_int, _1: c_ulong) -> c_int;
  pub fn XFreeCursor (_2: *mut Display, _1: c_ulong) -> c_int;
  pub fn XFreeEventData (_2: *mut Display, _1: *mut XGenericEventCookie) -> ();
  pub fn XFreeExtensionList (_1: *mut *mut c_char) -> c_int;
  pub fn XFreeFont (_2: *mut Display, _1: *mut XFontStruct) -> c_int;
  pub fn XFreeFontInfo (_3: *mut *mut c_char, _2: *mut XFontStruct, _1: c_int) -> c_int;
  pub fn XFreeFontNames (_1: *mut *mut c_char) -> c_int;
  pub fn XFreeFontPath (_1: *mut *mut c_char) -> c_int;
  pub fn XFreeFontSet (_2: *mut Display, _1: XFontSet) -> ();
  pub fn XFreeGC (_2: *mut Display, _1: GC) -> c_int;
  pub fn XFreeModifiermap (_1: *mut XModifierKeymap) -> c_int;
  pub fn XFreePixmap (_2: *mut Display, _1: c_ulong) -> c_int;
  pub fn XFreeStringList (_1: *mut *mut c_char) -> ();
  pub fn XGContextFromGC (_1: GC) -> c_ulong;
  pub fn XGeometry (_13: *mut Display, _12: c_int, _11: *const c_char, _10: *const c_char, _9: c_uint, _8: c_uint, _7: c_uint, _6: c_int, _5: c_int, _4: *mut c_int, _3: *mut c_int, _2: *mut c_int, _1: *mut c_int) -> c_int;
  pub fn XGetAtomName (_2: *mut Display, _1: c_ulong) -> *mut c_char;
  pub fn XGetAtomNames (_4: *mut Display, _3: *mut c_ulong, _2: c_int, _1: *mut *mut c_char) -> c_int;
  pub fn XGetClassHint (_3: *mut Display, _2: c_ulong, _1: *mut XClassHint) -> c_int;
  pub fn XGetCommand (_4: *mut Display, _3: c_ulong, _2: *mut *mut *mut c_char, _1: *mut c_int) -> c_int;
  pub fn XGetDefault (_3: *mut Display, _2: *const c_char, _1: *const c_char) -> *mut c_char;
  pub fn XGetErrorDatabaseText (_6: *mut Display, _5: *const c_char, _4: *const c_char, _3: *const c_char, _2: *mut c_char, _1: c_int) -> c_int;
  pub fn XGetErrorText (_4: *mut Display, _3: c_int, _2: *mut c_char, _1: c_int) -> c_int;
  pub fn XGetEventData (_2: *mut Display, _1: *mut XGenericEventCookie) -> c_int;
  pub fn XGetFontPath (_2: *mut Display, _1: *mut c_int) -> *mut *mut c_char;
  pub fn XGetFontProperty (_3: *mut XFontStruct, _2: c_ulong, _1: *mut c_ulong) -> c_int;
  pub fn XGetGCValues (_4: *mut Display, _3: GC, _2: c_ulong, _1: *mut XGCValues) -> c_int;
  pub fn XGetGeometry (_9: *mut Display, _8: c_ulong, _7: *mut c_ulong, _6: *mut c_int, _5: *mut c_int, _4: *mut c_uint, _3: *mut c_uint, _2: *mut c_uint, _1: *mut c_uint) -> c_int;
  pub fn XGetIconName (_3: *mut Display, _2: c_ulong, _1: *mut *mut c_char) -> c_int;
  pub fn XGetIconSizes (_4: *mut Display, _3: c_ulong, _2: *mut *mut XIconSize, _1: *mut c_int) -> c_int;
  pub fn XGetICValues (_1: XIC, ...) -> *mut c_char;
  pub fn XGetImage (_8: *mut Display, _7: c_ulong, _6: c_int, _5: c_int, _4: c_uint, _3: c_uint, _2: c_ulong, _1: c_int) -> *mut XImage;
  pub fn XGetIMValues (_1: XIM, ...) -> *mut c_char;
  pub fn XGetInputFocus (_3: *mut Display, _2: *mut c_ulong, _1: *mut c_int) -> c_int;
  pub fn XGetKeyboardControl (_2: *mut Display, _1: *mut XKeyboardState) -> c_int;
  pub fn XGetKeyboardMapping (_4: *mut Display, _3: c_uchar, _2: c_int, _1: *mut c_int) -> *mut c_ulong;
  pub fn XGetModifierMapping (_1: *mut Display) -> *mut XModifierKeymap;
  pub fn XGetMotionEvents (_5: *mut Display, _4: c_ulong, _3: c_ulong, _2: c_ulong, _1: *mut c_int) -> *mut XTimeCoord;
  pub fn XGetNormalHints (_3: *mut Display, _2: c_ulong, _1: *mut XSizeHints) -> c_int;
  pub fn XGetOCValues (_1: XFontSet, ...) -> *mut c_char;
  pub fn XGetOMValues (_1: XOM, ...) -> *mut c_char;
  pub fn XGetPixel (_3: *mut XImage, _2: c_int, _1: c_int) -> c_ulong;
  pub fn XGetPointerControl (_4: *mut Display, _3: *mut c_int, _2: *mut c_int, _1: *mut c_int) -> c_int;
  pub fn XGetPointerMapping (_3: *mut Display, _2: *mut c_uchar, _1: c_int) -> c_int;
  pub fn XGetRGBColormaps (_5: *mut Display, _4: c_ulong, _3: *mut *mut XStandardColormap, _2: *mut c_int, _1: c_ulong) -> c_int;
  pub fn XGetScreenSaver (_5: *mut Display, _4: *mut c_int, _3: *mut c_int, _2: *mut c_int, _1: *mut c_int) -> c_int;
  pub fn XGetSelectionOwner (_2: *mut Display, _1: c_ulong) -> c_ulong;
  pub fn XGetSizeHints (_4: *mut Display, _3: c_ulong, _2: *mut XSizeHints, _1: c_ulong) -> c_int;
  pub fn XGetStandardColormap (_4: *mut Display, _3: c_ulong, _2: *mut XStandardColormap, _1: c_ulong) -> c_int;
  pub fn XGetSubImage (_11: *mut Display, _10: c_ulong, _9: c_int, _8: c_int, _7: c_uint, _6: c_uint, _5: c_ulong, _4: c_int, _3: *mut XImage, _2: c_int, _1: c_int) -> *mut XImage;
  pub fn XGetTextProperty (_4: *mut Display, _3: c_ulong, _2: *mut XTextProperty, _1: c_ulong) -> c_int;
  pub fn XGetTransientForHint (_3: *mut Display, _2: c_ulong, _1: *mut c_ulong) -> c_int;
  pub fn XGetVisualInfo (_4: *mut Display, _3: c_long, _2: *mut XVisualInfo, _1: *mut c_int) -> *mut XVisualInfo;
  pub fn XGetWindowAttributes (_3: *mut Display, _2: c_ulong, _1: *mut XWindowAttributes) -> c_int;
  pub fn XGetWindowProperty (_12: *mut Display, _11: c_ulong, _10: c_ulong, _9: c_long, _8: c_long, _7: c_int, _6: c_ulong, _5: *mut c_ulong, _4: *mut c_int, _3: *mut c_ulong, _2: *mut c_ulong, _1: *mut *mut c_uchar) -> c_int;
  pub fn XGetWMClientMachine (_3: *mut Display, _2: c_ulong, _1: *mut XTextProperty) -> c_int;
  pub fn XGetWMColormapWindows (_4: *mut Display, _3: c_ulong, _2: *mut *mut c_ulong, _1: *mut c_int) -> c_int;
  pub fn XGetWMHints (_2: *mut Display, _1: c_ulong) -> *mut XWMHints;
  pub fn XGetWMIconName (_3: *mut Display, _2: c_ulong, _1: *mut XTextProperty) -> c_int;
  pub fn XGetWMName (_3: *mut Display, _2: c_ulong, _1: *mut XTextProperty) -> c_int;
  pub fn XGetWMNormalHints (_4: *mut Display, _3: c_ulong, _2: *mut XSizeHints, _1: *mut c_long) -> c_int;
  pub fn XGetWMProtocols (_4: *mut Display, _3: c_ulong, _2: *mut *mut c_ulong, _1: *mut c_int) -> c_int;
  pub fn XGetWMSizeHints (_5: *mut Display, _4: c_ulong, _3: *mut XSizeHints, _2: *mut c_long, _1: c_ulong) -> c_int;
  pub fn XGetZoomHints (_3: *mut Display, _2: c_ulong, _1: *mut XSizeHints) -> c_int;
  pub fn XGrabButton (_10: *mut Display, _9: c_uint, _8: c_uint, _7: c_ulong, _6: c_int, _5: c_uint, _4: c_int, _3: c_int, _2: c_ulong, _1: c_ulong) -> c_int;
  pub fn XGrabKey (_7: *mut Display, _6: c_int, _5: c_uint, _4: c_ulong, _3: c_int, _2: c_int, _1: c_int) -> c_int;
  pub fn XGrabKeyboard (_6: *mut Display, _5: c_ulong, _4: c_int, _3: c_int, _2: c_int, _1: c_ulong) -> c_int;
  pub fn XGrabPointer (_9: *mut Display, _8: c_ulong, _7: c_int, _6: c_uint, _5: c_int, _4: c_int, _3: c_ulong, _2: c_ulong, _1: c_ulong) -> c_int;
  pub fn XGrabServer (_1: *mut Display) -> c_int;
  pub fn XHeightMMOfScreen (_1: *mut Screen) -> c_int;
  pub fn XHeightOfScreen (_1: *mut Screen) -> c_int;
  pub fn XIconifyWindow (_3: *mut Display, _2: c_ulong, _1: c_int) -> c_int;
  pub fn XIfEvent (_4: *mut Display, _3: *mut XEvent, _2: Option<unsafe extern "C" fn (*mut Display, *mut XEvent, *mut c_char) -> c_int>, _1: *mut c_char) -> c_int;
  pub fn XImageByteOrder (_1: *mut Display) -> c_int;
  pub fn XIMOfIC (_1: XIC) -> XIM;
  pub fn XInitExtension (_2: *mut Display, _1: *const c_char) -> *mut XExtCodes;
  pub fn XInitImage (_1: *mut XImage) -> c_int;
  pub fn XInitThreads () -> c_int;
  pub fn XInsertModifiermapEntry (_3: *mut XModifierKeymap, _2: c_uchar, _1: c_int) -> *mut XModifierKeymap;
  pub fn XInstallColormap (_2: *mut Display, _1: c_ulong) -> c_int;
  pub fn XInternalConnectionNumbers (_3: *mut Display, _2: *mut *mut c_int, _1: *mut c_int) -> c_int;
  pub fn XInternAtom (_3: *mut Display, _2: *const c_char, _1: c_int) -> c_ulong;
  pub fn XInternAtoms (_5: *mut Display, _4: *mut *mut c_char, _3: c_int, _2: c_int, _1: *mut c_ulong) -> c_int;
  pub fn XIntersectRegion (_3: Region, _2: Region, _1: Region) -> c_int;
  pub fn XKeycodeToKeysym (_3: *mut Display, _2: c_uchar, _1: c_int) -> c_ulong;
  pub fn XKeysymToKeycode (_2: *mut Display, _1: c_ulong) -> c_uchar;
  pub fn XKeysymToString (_1: c_ulong) -> *mut c_char;
  pub fn XKillClient (_2: *mut Display, _1: c_ulong) -> c_int;
  pub fn XLastKnownRequestProcessed (_1: *mut Display) -> c_ulong;
  pub fn XListDepths (_3: *mut Display, _2: c_int, _1: *mut c_int) -> *mut c_int;
  pub fn XListExtensions (_2: *mut Display, _1: *mut c_int) -> *mut *mut c_char;
  pub fn XListFonts (_4: *mut Display, _3: *const c_char, _2: c_int, _1: *mut c_int) -> *mut *mut c_char;
  pub fn XListFontsWithInfo (_5: *mut Display, _4: *const c_char, _3: c_int, _2: *mut c_int, _1: *mut *mut XFontStruct) -> *mut *mut c_char;
  pub fn XListHosts (_3: *mut Display, _2: *mut c_int, _1: *mut c_int) -> *mut XHostAddress;
  pub fn XListInstalledColormaps (_3: *mut Display, _2: c_ulong, _1: *mut c_int) -> *mut c_ulong;
  pub fn XListPixmapFormats (_2: *mut Display, _1: *mut c_int) -> *mut XPixmapFormatValues;
  pub fn XListProperties (_3: *mut Display, _2: c_ulong, _1: *mut c_int) -> *mut c_ulong;
  pub fn XLoadFont (_2: *mut Display, _1: *const c_char) -> c_ulong;
  pub fn XLoadQueryFont (_2: *mut Display, _1: *const c_char) -> *mut XFontStruct;
  pub fn XLocaleOfFontSet (_1: XFontSet) -> *mut c_char;
  pub fn XLocaleOfIM (_1: XIM) -> *mut c_char;
  pub fn XLocaleOfOM (_1: XOM) -> *mut c_char;
  pub fn XLockDisplay (_1: *mut Display) -> ();
  pub fn XLookupColor (_5: *mut Display, _4: c_ulong, _3: *const c_char, _2: *mut XColor, _1: *mut XColor) -> c_int;
  pub fn XLookupKeysym (_2: *mut XKeyEvent, _1: c_int) -> c_ulong;
  pub fn XLookupString (_5: *mut XKeyEvent, _4: *mut c_char, _3: c_int, _2: *mut c_ulong, _1: *mut XComposeStatus) -> c_int;
  pub fn XLowerWindow (_2: *mut Display, _1: c_ulong) -> c_int;
  pub fn XMapRaised (_2: *mut Display, _1: c_ulong) -> c_int;
  pub fn XMapSubwindows (_2: *mut Display, _1: c_ulong) -> c_int;
  pub fn XMapWindow (_2: *mut Display, _1: c_ulong) -> c_int;
  pub fn XMaskEvent (_3: *mut Display, _2: c_long, _1: *mut XEvent) -> c_int;
  pub fn XMatchVisualInfo (_5: *mut Display, _4: c_int, _3: c_int, _2: c_int, _1: *mut XVisualInfo) -> c_int;
  pub fn XMaxCmapsOfScreen (_1: *mut Screen) -> c_int;
  pub fn XMaxRequestSize (_1: *mut Display) -> c_long;
  pub fn XmbDrawImageString (_8: *mut Display, _7: c_ulong, _6: XFontSet, _5: GC, _4: c_int, _3: c_int, _2: *const c_char, _1: c_int) -> ();
  pub fn XmbDrawString (_8: *mut Display, _7: c_ulong, _6: XFontSet, _5: GC, _4: c_int, _3: c_int, _2: *const c_char, _1: c_int) -> ();
  pub fn XmbDrawText (_7: *mut Display, _6: c_ulong, _5: GC, _4: c_int, _3: c_int, _2: *mut XmbTextItem, _1: c_int) -> ();
  pub fn XmbLookupString (_6: XIC, _5: *mut XKeyEvent, _4: *mut c_char, _3: c_int, _2: *mut c_ulong, _1: *mut c_int) -> c_int;
  pub fn XmbResetIC (_1: XIC) -> *mut c_char;
  pub fn XmbSetWMProperties (_9: *mut Display, _8: c_ulong, _7: *const c_char, _6: *const c_char, _5: *mut *mut c_char, _4: c_int, _3: *mut XSizeHints, _2: *mut XWMHints, _1: *mut XClassHint) -> ();
  pub fn XmbTextEscapement (_3: XFontSet, _2: *const c_char, _1: c_int) -> c_int;
  pub fn XmbTextExtents (_5: XFontSet, _4: *const c_char, _3: c_int, _2: *mut XRectangle, _1: *mut XRectangle) -> c_int;
  pub fn XmbTextListToTextProperty (_5: *mut Display, _4: *mut *mut c_char, _3: c_int, _2: XICCEncodingStyle, _1: *mut XTextProperty) -> c_int;
  pub fn XmbTextPerCharExtents (_9: XFontSet, _8: *const c_char, _7: c_int, _6: *mut XRectangle, _5: *mut XRectangle, _4: c_int, _3: *mut c_int, _2: *mut XRectangle, _1: *mut XRectangle) -> c_int;
  pub fn XmbTextPropertyToTextList (_4: *mut Display, _3: *const XTextProperty, _2: *mut *mut *mut c_char, _1: *mut c_int) -> c_int;
  pub fn XMinCmapsOfScreen (_1: *mut Screen) -> c_int;
  pub fn XMoveResizeWindow (_6: *mut Display, _5: c_ulong, _4: c_int, _3: c_int, _2: c_uint, _1: c_uint) -> c_int;
  pub fn XMoveWindow (_4: *mut Display, _3: c_ulong, _2: c_int, _1: c_int) -> c_int;
  pub fn XNewModifiermap (_1: c_int) -> *mut XModifierKeymap;
  pub fn XNextEvent (_2: *mut Display, _1: *mut XEvent) -> c_int;
  pub fn XNextRequest (_1: *mut Display) -> c_ulong;
  pub fn XNoOp (_1: *mut Display) -> c_int;
  pub fn XOffsetRegion (_3: Region, _2: c_int, _1: c_int) -> c_int;
  pub fn XOMOfOC (_1: XFontSet) -> XOM;
  pub fn XOpenDisplay (_1: *const c_char) -> *mut Display;
  pub fn XOpenIM (_4: *mut Display, _3: XrmDatabase, _2: *mut c_char, _1: *mut c_char) -> XIM;
  pub fn XOpenOM (_4: *mut Display, _3: XrmDatabase, _2: *const c_char, _1: *const c_char) -> XOM;
  pub fn XParseColor (_4: *mut Display, _3: c_ulong, _2: *const c_char, _1: *mut XColor) -> c_int;
  pub fn XParseGeometry (_5: *const c_char, _4: *mut c_int, _3: *mut c_int, _2: *mut c_uint, _1: *mut c_uint) -> c_int;
  pub fn XPeekEvent (_2: *mut Display, _1: *mut XEvent) -> c_int;
  pub fn XPeekIfEvent (_4: *mut Display, _3: *mut XEvent, _2: Option<unsafe extern "C" fn (*mut Display, *mut XEvent, *mut c_char) -> c_int>, _1: *mut c_char) -> c_int;
  pub fn XPending (_1: *mut Display) -> c_int;
  pub fn Xpermalloc (_1: c_uint) -> *mut c_char;
  pub fn XPlanesOfScreen (_1: *mut Screen) -> c_int;
  pub fn XPointInRegion (_3: Region, _2: c_int, _1: c_int) -> c_int;
  pub fn XPolygonRegion (_3: *mut XPoint, _2: c_int, _1: c_int) -> Region;
  pub fn XProcessInternalConnection (_2: *mut Display, _1: c_int) -> ();
  pub fn XProtocolRevision (_1: *mut Display) -> c_int;
  pub fn XProtocolVersion (_1: *mut Display) -> c_int;
  pub fn XPutBackEvent (_2: *mut Display, _1: *mut XEvent) -> c_int;
  pub fn XPutImage (_10: *mut Display, _9: c_ulong, _8: GC, _7: *mut XImage, _6: c_int, _5: c_int, _4: c_int, _3: c_int, _2: c_uint, _1: c_uint) -> c_int;
  pub fn XPutPixel (_4: *mut XImage, _3: c_int, _2: c_int, _1: c_ulong) -> c_int;
  pub fn XQLength (_1: *mut Display) -> c_int;
  pub fn XQueryBestCursor (_6: *mut Display, _5: c_ulong, _4: c_uint, _3: c_uint, _2: *mut c_uint, _1: *mut c_uint) -> c_int;
  pub fn XQueryBestSize (_7: *mut Display, _6: c_int, _5: c_ulong, _4: c_uint, _3: c_uint, _2: *mut c_uint, _1: *mut c_uint) -> c_int;
  pub fn XQueryBestStipple (_6: *mut Display, _5: c_ulong, _4: c_uint, _3: c_uint, _2: *mut c_uint, _1: *mut c_uint) -> c_int;
  pub fn XQueryBestTile (_6: *mut Display, _5: c_ulong, _4: c_uint, _3: c_uint, _2: *mut c_uint, _1: *mut c_uint) -> c_int;
  pub fn XQueryColor (_3: *mut Display, _2: c_ulong, _1: *mut XColor) -> c_int;
  pub fn XQueryColors (_4: *mut Display, _3: c_ulong, _2: *mut XColor, _1: c_int) -> c_int;
  pub fn XQueryExtension (_5: *mut Display, _4: *const c_char, _3: *mut c_int, _2: *mut c_int, _1: *mut c_int) -> c_int;
  pub fn XQueryFont (_2: *mut Display, _1: c_ulong) -> *mut XFontStruct;
  pub fn XQueryKeymap (_2: *mut Display, _1: *mut c_char) -> c_int;
  pub fn XQueryPointer (_9: *mut Display, _8: c_ulong, _7: *mut c_ulong, _6: *mut c_ulong, _5: *mut c_int, _4: *mut c_int, _3: *mut c_int, _2: *mut c_int, _1: *mut c_uint) -> c_int;
  pub fn XQueryTextExtents (_8: *mut Display, _7: c_ulong, _6: *const c_char, _5: c_int, _4: *mut c_int, _3: *mut c_int, _2: *mut c_int, _1: *mut XCharStruct) -> c_int;
  pub fn XQueryTextExtents16 (_8: *mut Display, _7: c_ulong, _6: *const XChar2b, _5: c_int, _4: *mut c_int, _3: *mut c_int, _2: *mut c_int, _1: *mut XCharStruct) -> c_int;
  pub fn XQueryTree (_6: *mut Display, _5: c_ulong, _4: *mut c_ulong, _3: *mut c_ulong, _2: *mut *mut c_ulong, _1: *mut c_uint) -> c_int;
  pub fn XRaiseWindow (_2: *mut Display, _1: c_ulong) -> c_int;
  pub fn XReadBitmapFile (_8: *mut Display, _7: c_ulong, _6: *const c_char, _5: *mut c_uint, _4: *mut c_uint, _3: *mut c_ulong, _2: *mut c_int, _1: *mut c_int) -> c_int;
  pub fn XReadBitmapFileData (_6: *const c_char, _5: *mut c_uint, _4: *mut c_uint, _3: *mut *mut c_uchar, _2: *mut c_int, _1: *mut c_int) -> c_int;
  pub fn XRebindKeysym (_6: *mut Display, _5: c_ulong, _4: *mut c_ulong, _3: c_int, _2: *const c_uchar, _1: c_int) -> c_int;
  pub fn XRecolorCursor (_4: *mut Display, _3: c_ulong, _2: *mut XColor, _1: *mut XColor) -> c_int;
  pub fn XReconfigureWMWindow (_5: *mut Display, _4: c_ulong, _3: c_int, _2: c_uint, _1: *mut XWindowChanges) -> c_int;
  pub fn XRectInRegion (_5: Region, _4: c_int, _3: c_int, _2: c_uint, _1: c_uint) -> c_int;
  pub fn XRefreshKeyboardMapping (_1: *mut XMappingEvent) -> c_int;
  pub fn XRegisterIMInstantiateCallback (_6: *mut Display, _5: XrmDatabase, _4: *mut c_char, _3: *mut c_char, _2: Option<unsafe extern "C" fn (*mut Display, *mut c_char, *mut c_char) -> ()>, _1: *mut c_char) -> c_int;
  pub fn XRemoveConnectionWatch (_3: *mut Display, _2: Option<unsafe extern "C" fn (*mut Display, *mut c_char, c_int, c_int, *mut *mut c_char) -> ()>, _1: *mut c_char) -> ();
  pub fn XRemoveFromSaveSet (_2: *mut Display, _1: c_ulong) -> c_int;
  pub fn XRemoveHost (_2: *mut Display, _1: *mut XHostAddress) -> c_int;
  pub fn XRemoveHosts (_3: *mut Display, _2: *mut XHostAddress, _1: c_int) -> c_int;
  pub fn XReparentWindow (_5: *mut Display, _4: c_ulong, _3: c_ulong, _2: c_int, _1: c_int) -> c_int;
  pub fn XResetScreenSaver (_1: *mut Display) -> c_int;
  pub fn XResizeWindow (_4: *mut Display, _3: c_ulong, _2: c_uint, _1: c_uint) -> c_int;
  pub fn XResourceManagerString (_1: *mut Display) -> *mut c_char;
  pub fn XRestackWindows (_3: *mut Display, _2: *mut c_ulong, _1: c_int) -> c_int;
  pub fn XrmCombineDatabase (_3: XrmDatabase, _2: *mut XrmDatabase, _1: c_int) -> ();
  pub fn XrmCombineFileDatabase (_3: *const c_char, _2: *mut XrmDatabase, _1: c_int) -> c_int;
  pub fn XrmDestroyDatabase (_1: XrmDatabase) -> ();
  pub fn XrmEnumerateDatabase (_6: XrmDatabase, _5: *mut c_int, _4: *mut c_int, _3: c_int, _2: Option<unsafe extern "C" fn (*mut XrmDatabase, *mut XrmBinding, *mut c_int, *mut c_int, *mut XrmValue, *mut c_char) -> c_int>, _1: *mut c_char) -> c_int;
  pub fn XrmGetDatabase (_1: *mut Display) -> XrmDatabase;
  pub fn XrmGetFileDatabase (_1: *const c_char) -> XrmDatabase;
  pub fn XrmGetResource (_5: XrmDatabase, _4: *const c_char, _3: *const c_char, _2: *mut *mut c_char, _1: *mut XrmValue) -> c_int;
  pub fn XrmGetStringDatabase (_1: *const c_char) -> XrmDatabase;
  pub fn XrmInitialize () -> ();
  pub fn XrmLocaleOfDatabase (_1: XrmDatabase) -> *const c_char;
  pub fn XrmMergeDatabases (_2: XrmDatabase, _1: *mut XrmDatabase) -> ();
  pub fn XrmParseCommand (_6: *mut XrmDatabase, _5: XrmOptionDescList, _4: c_int, _3: *const c_char, _2: *mut c_int, _1: *mut *mut c_char) -> ();
  pub fn XrmPermStringToQuark (_1: *const c_char) -> c_int;
  pub fn XrmPutFileDatabase (_2: XrmDatabase, _1: *const c_char) -> ();
  pub fn XrmPutLineResource (_2: *mut XrmDatabase, _1: *const c_char) -> ();
  pub fn XrmPutResource (_4: *mut XrmDatabase, _3: *const c_char, _2: *const c_char, _1: *mut XrmValue) -> ();
  pub fn XrmPutStringResource (_3: *mut XrmDatabase, _2: *const c_char, _1: *const c_char) -> ();
  pub fn XrmQGetResource (_5: XrmDatabase, _4: *mut c_int, _3: *mut c_int, _2: *mut c_int, _1: *mut XrmValue) -> c_int;
  pub fn XrmQGetSearchList (_5: XrmDatabase, _4: *mut c_int, _3: *mut c_int, _2: *mut *mut XrmDatabase, _1: c_int) -> c_int;
  pub fn XrmQGetSearchResource (_5: *mut *mut XrmDatabase, _4: c_int, _3: c_int, _2: *mut c_int, _1: *mut XrmValue) -> c_int;
  pub fn XrmQPutResource (_5: *mut XrmDatabase, _4: *mut XrmBinding, _3: *mut c_int, _2: c_int, _1: *mut XrmValue) -> ();
  pub fn XrmQPutStringResource (_4: *mut XrmDatabase, _3: *mut XrmBinding, _2: *mut c_int, _1: *const c_char) -> ();
  pub fn XrmQuarkToString (_1: c_int) -> *mut c_char;
  pub fn XrmSetDatabase (_2: *mut Display, _1: XrmDatabase) -> ();
  pub fn XrmStringToBindingQuarkList (_3: *const c_char, _2: *mut XrmBinding, _1: *mut c_int) -> ();
  pub fn XrmStringToQuark (_1: *const c_char) -> c_int;
  pub fn XrmStringToQuarkList (_2: *const c_char, _1: *mut c_int) -> ();
  pub fn XrmUniqueQuark () -> c_int;
  pub fn XRootWindow (_2: *mut Display, _1: c_int) -> c_ulong;
  pub fn XRootWindowOfScreen (_1: *mut Screen) -> c_ulong;
  pub fn XRotateBuffers (_2: *mut Display, _1: c_int) -> c_int;
  pub fn XRotateWindowProperties (_5: *mut Display, _4: c_ulong, _3: *mut c_ulong, _2: c_int, _1: c_int) -> c_int;
  pub fn XSaveContext (_4: *mut Display, _3: c_ulong, _2: c_int, _1: *const c_char) -> c_int;
  pub fn XScreenCount (_1: *mut Display) -> c_int;
  pub fn XScreenNumberOfScreen (_1: *mut Screen) -> c_int;
  pub fn XScreenOfDisplay (_2: *mut Display, _1: c_int) -> *mut Screen;
  pub fn XScreenResourceString (_1: *mut Screen) -> *mut c_char;
  pub fn XSelectInput (_3: *mut Display, _2: c_ulong, _1: c_long) -> c_int;
  pub fn XSendEvent (_5: *mut Display, _4: c_ulong, _3: c_int, _2: c_long, _1: *mut XEvent) -> c_int;
  pub fn XServerVendor (_1: *mut Display) -> *mut c_char;
  pub fn XSetAccessControl (_2: *mut Display, _1: c_int) -> c_int;
  pub fn XSetAfterFunction (_2: *mut Display, _1: Option<unsafe extern "C" fn (*mut Display) -> c_int>) -> Option<unsafe extern "C" fn (*mut Display) -> c_int>;
  pub fn XSetArcMode (_3: *mut Display, _2: GC, _1: c_int) -> c_int;
  pub fn XSetAuthorization (_4: *mut c_char, _3: c_int, _2: *mut c_char, _1: c_int) -> ();
  pub fn XSetBackground (_3: *mut Display, _2: GC, _1: c_ulong) -> c_int;
  pub fn XSetClassHint (_3: *mut Display, _2: c_ulong, _1: *mut XClassHint) -> c_int;
  pub fn XSetClipMask (_3: *mut Display, _2: GC, _1: c_ulong) -> c_int;
  pub fn XSetClipOrigin (_4: *mut Display, _3: GC, _2: c_int, _1: c_int) -> c_int;
  pub fn XSetClipRectangles (_7: *mut Display, _6: GC, _5: c_int, _4: c_int, _3: *mut XRectangle, _2: c_int, _1: c_int) -> c_int;
  pub fn XSetCloseDownMode (_2: *mut Display, _1: c_int) -> c_int;
  pub fn XSetCommand (_4: *mut Display, _3: c_ulong, _2: *mut *mut c_char, _1: c_int) -> c_int;
  pub fn XSetDashes (_5: *mut Display, _4: GC, _3: c_int, _2: *const c_char, _1: c_int) -> c_int;
  pub fn XSetErrorHandler (_1: Option<unsafe extern "C" fn (*mut Display, *mut XErrorEvent) -> c_int>) -> Option<unsafe extern "C" fn (*mut Display, *mut XErrorEvent) -> c_int>;
  pub fn XSetFillRule (_3: *mut Display, _2: GC, _1: c_int) -> c_int;
  pub fn XSetFillStyle (_3: *mut Display, _2: GC, _1: c_int) -> c_int;
  pub fn XSetFont (_3: *mut Display, _2: GC, _1: c_ulong) -> c_int;
  pub fn XSetFontPath (_3: *mut Display, _2: *mut *mut c_char, _1: c_int) -> c_int;
  pub fn XSetForeground (_3: *mut Display, _2: GC, _1: c_ulong) -> c_int;
  pub fn XSetFunction (_3: *mut Display, _2: GC, _1: c_int) -> c_int;
  pub fn XSetGraphicsExposures (_3: *mut Display, _2: GC, _1: c_int) -> c_int;
  pub fn XSetICFocus (_1: XIC) -> ();
  pub fn XSetIconName (_3: *mut Display, _2: c_ulong, _1: *const c_char) -> c_int;
  pub fn XSetIconSizes (_4: *mut Display, _3: c_ulong, _2: *mut XIconSize, _1: c_int) -> c_int;
  pub fn XSetICValues (_1: XIC, ...) -> *mut c_char;
  pub fn XSetIMValues (_1: XIM, ...) -> *mut c_char;
  pub fn XSetInputFocus (_4: *mut Display, _3: c_ulong, _2: c_int, _1: c_ulong) -> c_int;
  pub fn XSetIOErrorHandler (_1: Option<unsafe extern "C" fn (*mut Display) -> c_int>) -> Option<unsafe extern "C" fn (*mut Display) -> c_int>;
  pub fn XSetLineAttributes (_6: *mut Display, _5: GC, _4: c_uint, _3: c_int, _2: c_int, _1: c_int) -> c_int;
  pub fn XSetLocaleModifiers (_1: *const c_char) -> *mut c_char;
  pub fn XSetModifierMapping (_2: *mut Display, _1: *mut XModifierKeymap) -> c_int;
  pub fn XSetNormalHints (_3: *mut Display, _2: c_ulong, _1: *mut XSizeHints) -> c_int;
  pub fn XSetOCValues (_1: XFontSet, ...) -> *mut c_char;
  pub fn XSetOMValues (_1: XOM, ...) -> *mut c_char;
  pub fn XSetPlaneMask (_3: *mut Display, _2: GC, _1: c_ulong) -> c_int;
  pub fn XSetPointerMapping (_3: *mut Display, _2: *const c_uchar, _1: c_int) -> c_int;
  pub fn XSetRegion (_3: *mut Display, _2: GC, _1: Region) -> c_int;
  pub fn XSetRGBColormaps (_5: *mut Display, _4: c_ulong, _3: *mut XStandardColormap, _2: c_int, _1: c_ulong) -> ();
  pub fn XSetScreenSaver (_5: *mut Display, _4: c_int, _3: c_int, _2: c_int, _1: c_int) -> c_int;
  pub fn XSetSelectionOwner (_4: *mut Display, _3: c_ulong, _2: c_ulong, _1: c_ulong) -> c_int;
  pub fn XSetSizeHints (_4: *mut Display, _3: c_ulong, _2: *mut XSizeHints, _1: c_ulong) -> c_int;
  pub fn XSetStandardColormap (_4: *mut Display, _3: c_ulong, _2: *mut XStandardColormap, _1: c_ulong) -> ();
  pub fn XSetStandardProperties (_8: *mut Display, _7: c_ulong, _6: *const c_char, _5: *const c_char, _4: c_ulong, _3: *mut *mut c_char, _2: c_int, _1: *mut XSizeHints) -> c_int;
  pub fn XSetState (_6: *mut Display, _5: GC, _4: c_ulong, _3: c_ulong, _2: c_int, _1: c_ulong) -> c_int;
  pub fn XSetStipple (_3: *mut Display, _2: GC, _1: c_ulong) -> c_int;
  pub fn XSetSubwindowMode (_3: *mut Display, _2: GC, _1: c_int) -> c_int;
  pub fn XSetTextProperty (_4: *mut Display, _3: c_ulong, _2: *mut XTextProperty, _1: c_ulong) -> ();
  pub fn XSetTile (_3: *mut Display, _2: GC, _1: c_ulong) -> c_int;
  pub fn XSetTransientForHint (_3: *mut Display, _2: c_ulong, _1: c_ulong) -> c_int;
  pub fn XSetTSOrigin (_4: *mut Display, _3: GC, _2: c_int, _1: c_int) -> c_int;
  pub fn XSetWindowBackground (_3: *mut Display, _2: c_ulong, _1: c_ulong) -> c_int;
  pub fn XSetWindowBackgroundPixmap (_3: *mut Display, _2: c_ulong, _1: c_ulong) -> c_int;
  pub fn XSetWindowBorder (_3: *mut Display, _2: c_ulong, _1: c_ulong) -> c_int;
  pub fn XSetWindowBorderPixmap (_3: *mut Display, _2: c_ulong, _1: c_ulong) -> c_int;
  pub fn XSetWindowBorderWidth (_3: *mut Display, _2: c_ulong, _1: c_uint) -> c_int;
  pub fn XSetWindowColormap (_3: *mut Display, _2: c_ulong, _1: c_ulong) -> c_int;
  pub fn XSetWMClientMachine (_3: *mut Display, _2: c_ulong, _1: *mut XTextProperty) -> ();
  pub fn XSetWMColormapWindows (_4: *mut Display, _3: c_ulong, _2: *mut c_ulong, _1: c_int) -> c_int;
  pub fn XSetWMHints (_3: *mut Display, _2: c_ulong, _1: *mut XWMHints) -> c_int;
  pub fn XSetWMIconName (_3: *mut Display, _2: c_ulong, _1: *mut XTextProperty) -> ();
  pub fn XSetWMName (_3: *mut Display, _2: c_ulong, _1: *mut XTextProperty) -> ();
  pub fn XSetWMNormalHints (_3: *mut Display, _2: c_ulong, _1: *mut XSizeHints) -> ();
  pub fn XSetWMProperties (_9: *mut Display, _8: c_ulong, _7: *mut XTextProperty, _6: *mut XTextProperty, _5: *mut *mut c_char, _4: c_int, _3: *mut XSizeHints, _2: *mut XWMHints, _1: *mut XClassHint) -> ();
  pub fn XSetWMProtocols (_4: *mut Display, _3: c_ulong, _2: *mut c_ulong, _1: c_int) -> c_int;
  pub fn XSetWMSizeHints (_4: *mut Display, _3: c_ulong, _2: *mut XSizeHints, _1: c_ulong) -> ();
  pub fn XSetZoomHints (_3: *mut Display, _2: c_ulong, _1: *mut XSizeHints) -> c_int;
  pub fn XShrinkRegion (_3: Region, _2: c_int, _1: c_int) -> c_int;
  pub fn XStoreBuffer (_4: *mut Display, _3: *const c_char, _2: c_int, _1: c_int) -> c_int;
  pub fn XStoreBytes (_3: *mut Display, _2: *const c_char, _1: c_int) -> c_int;
  pub fn XStoreColor (_3: *mut Display, _2: c_ulong, _1: *mut XColor) -> c_int;
  pub fn XStoreColors (_4: *mut Display, _3: c_ulong, _2: *mut XColor, _1: c_int) -> c_int;
  pub fn XStoreName (_3: *mut Display, _2: c_ulong, _1: *const c_char) -> c_int;
  pub fn XStoreNamedColor (_5: *mut Display, _4: c_ulong, _3: *const c_char, _2: c_ulong, _1: c_int) -> c_int;
  pub fn XStringListToTextProperty (_3: *mut *mut c_char, _2: c_int, _1: *mut XTextProperty) -> c_int;
  pub fn XStringToKeysym (_1: *const c_char) -> c_ulong;
  pub fn XSubImage (_5: *mut XImage, _4: c_int, _3: c_int, _2: c_uint, _1: c_uint) -> *mut XImage;
  pub fn XSubtractRegion (_3: Region, _2: Region, _1: Region) -> c_int;
  pub fn XSupportsLocale () -> c_int;
  pub fn XSync (_2: *mut Display, _1: c_int) -> c_int;
  pub fn XSynchronize (_2: *mut Display, _1: c_int) -> Option<unsafe extern "C" fn (*mut Display) -> c_int>;
  pub fn XTextExtents (_7: *mut XFontStruct, _6: *const c_char, _5: c_int, _4: *mut c_int, _3: *mut c_int, _2: *mut c_int, _1: *mut XCharStruct) -> c_int;
  pub fn XTextExtents16 (_7: *mut XFontStruct, _6: *const XChar2b, _5: c_int, _4: *mut c_int, _3: *mut c_int, _2: *mut c_int, _1: *mut XCharStruct) -> c_int;
  pub fn XTextPropertyToStringList (_3: *mut XTextProperty, _2: *mut *mut *mut c_char, _1: *mut c_int) -> c_int;
  pub fn XTextWidth (_3: *mut XFontStruct, _2: *const c_char, _1: c_int) -> c_int;
  pub fn XTextWidth16 (_3: *mut XFontStruct, _2: *const XChar2b, _1: c_int) -> c_int;
  pub fn XTranslateCoordinates (_8: *mut Display, _7: c_ulong, _6: c_ulong, _5: c_int, _4: c_int, _3: *mut c_int, _2: *mut c_int, _1: *mut c_ulong) -> c_int;
  pub fn XUndefineCursor (_2: *mut Display, _1: c_ulong) -> c_int;
  pub fn XUngrabButton (_4: *mut Display, _3: c_uint, _2: c_uint, _1: c_ulong) -> c_int;
  pub fn XUngrabKey (_4: *mut Display, _3: c_int, _2: c_uint, _1: c_ulong) -> c_int;
  pub fn XUngrabKeyboard (_2: *mut Display, _1: c_ulong) -> c_int;
  pub fn XUngrabPointer (_2: *mut Display, _1: c_ulong) -> c_int;
  pub fn XUngrabServer (_1: *mut Display) -> c_int;
  pub fn XUninstallColormap (_2: *mut Display, _1: c_ulong) -> c_int;
  pub fn XUnionRectWithRegion (_3: *mut XRectangle, _2: Region, _1: Region) -> c_int;
  pub fn XUnionRegion (_3: Region, _2: Region, _1: Region) -> c_int;
  pub fn XUnloadFont (_2: *mut Display, _1: c_ulong) -> c_int;
  pub fn XUnlockDisplay (_1: *mut Display) -> ();
  pub fn XUnmapSubwindows (_2: *mut Display, _1: c_ulong) -> c_int;
  pub fn XUnmapWindow (_2: *mut Display, _1: c_ulong) -> c_int;
  pub fn XUnregisterIMInstantiateCallback (_6: *mut Display, _5: XrmDatabase, _4: *mut c_char, _3: *mut c_char, _2: Option<unsafe extern "C" fn (*mut Display, *mut c_char, *mut c_char) -> ()>, _1: *mut c_char) -> c_int;
  pub fn XUnsetICFocus (_1: XIC) -> ();
  pub fn Xutf8DrawImageString (_8: *mut Display, _7: c_ulong, _6: XFontSet, _5: GC, _4: c_int, _3: c_int, _2: *const c_char, _1: c_int) -> ();
  pub fn Xutf8DrawString (_8: *mut Display, _7: c_ulong, _6: XFontSet, _5: GC, _4: c_int, _3: c_int, _2: *const c_char, _1: c_int) -> ();
  pub fn Xutf8DrawText (_7: *mut Display, _6: c_ulong, _5: GC, _4: c_int, _3: c_int, _2: *mut XmbTextItem, _1: c_int) -> ();
  pub fn Xutf8LookupString (_6: XIC, _5: *mut XKeyEvent, _4: *mut c_char, _3: c_int, _2: *mut c_ulong, _1: *mut c_int) -> c_int;
  pub fn Xutf8ResetIC (_1: XIC) -> *mut c_char;
  pub fn Xutf8SetWMProperties (_9: *mut Display, _8: c_ulong, _7: *const c_char, _6: *const c_char, _5: *mut *mut c_char, _4: c_int, _3: *mut XSizeHints, _2: *mut XWMHints, _1: *mut XClassHint) -> ();
  pub fn Xutf8TextEscapement (_3: XFontSet, _2: *const c_char, _1: c_int) -> c_int;
  pub fn Xutf8TextExtents (_5: XFontSet, _4: *const c_char, _3: c_int, _2: *mut XRectangle, _1: *mut XRectangle) -> c_int;
  pub fn Xutf8TextListToTextProperty (_5: *mut Display, _4: *mut *mut c_char, _3: c_int, _2: XICCEncodingStyle, _1: *mut XTextProperty) -> c_int;
  pub fn Xutf8TextPerCharExtents (_9: XFontSet, _8: *const c_char, _7: c_int, _6: *mut XRectangle, _5: *mut XRectangle, _4: c_int, _3: *mut c_int, _2: *mut XRectangle, _1: *mut XRectangle) -> c_int;
  pub fn Xutf8TextPropertyToTextList (_4: *mut Display, _3: *const XTextProperty, _2: *mut *mut *mut c_char, _1: *mut c_int) -> c_int;
  pub fn XVaCreateNestedList (_1: c_int, ...) -> *mut ();
  pub fn XVendorRelease (_1: *mut Display) -> c_int;
  pub fn XVisualIDFromVisual (_1: *mut Visual) -> c_ulong;
  pub fn XWarpPointer (_9: *mut Display, _8: c_ulong, _7: c_ulong, _6: c_int, _5: c_int, _4: c_uint, _3: c_uint, _2: c_int, _1: c_int) -> c_int;
  pub fn XwcDrawImageString (_8: *mut Display, _7: c_ulong, _6: XFontSet, _5: GC, _4: c_int, _3: c_int, _2: *const wchar_t, _1: c_int) -> ();
  pub fn XwcDrawString (_8: *mut Display, _7: c_ulong, _6: XFontSet, _5: GC, _4: c_int, _3: c_int, _2: *const wchar_t, _1: c_int) -> ();
  pub fn XwcDrawText (_7: *mut Display, _6: c_ulong, _5: GC, _4: c_int, _3: c_int, _2: *mut XwcTextItem, _1: c_int) -> ();
  pub fn XwcFreeStringList (_1: *mut *mut wchar_t) -> ();
  pub fn XwcLookupString (_6: XIC, _5: *mut XKeyEvent, _4: *mut wchar_t, _3: c_int, _2: *mut c_ulong, _1: *mut c_int) -> c_int;
  pub fn XwcResetIC (_1: XIC) -> *mut wchar_t;
  pub fn XwcTextEscapement (_3: XFontSet, _2: *const wchar_t, _1: c_int) -> c_int;
  pub fn XwcTextExtents (_5: XFontSet, _4: *const wchar_t, _3: c_int, _2: *mut XRectangle, _1: *mut XRectangle) -> c_int;
  pub fn XwcTextListToTextProperty (_5: *mut Display, _4: *mut *mut wchar_t, _3: c_int, _2: XICCEncodingStyle, _1: *mut XTextProperty) -> c_int;
  pub fn XwcTextPerCharExtents (_9: XFontSet, _8: *const wchar_t, _7: c_int, _6: *mut XRectangle, _5: *mut XRectangle, _4: c_int, _3: *mut c_int, _2: *mut XRectangle, _1: *mut XRectangle) -> c_int;
  pub fn XwcTextPropertyToTextList (_4: *mut Display, _3: *const XTextProperty, _2: *mut *mut *mut wchar_t, _1: *mut c_int) -> c_int;
  pub fn XWhitePixel (_2: *mut Display, _1: c_int) -> c_ulong;
  pub fn XWhitePixelOfScreen (_1: *mut Screen) -> c_ulong;
  pub fn XWidthMMOfScreen (_1: *mut Screen) -> c_int;
  pub fn XWidthOfScreen (_1: *mut Screen) -> c_int;
  pub fn XWindowEvent (_4: *mut Display, _3: c_ulong, _2: c_long, _1: *mut XEvent) -> c_int;
  pub fn XWithdrawWindow (_3: *mut Display, _2: c_ulong, _1: c_int) -> c_int;
  pub fn XWMGeometry (_11: *mut Display, _10: c_int, _9: *const c_char, _8: *const c_char, _7: c_uint, _6: *mut XSizeHints, _5: *mut c_int, _4: *mut c_int, _3: *mut c_int, _2: *mut c_int, _1: *mut c_int) -> c_int;
  pub fn XWriteBitmapFile (_7: *mut Display, _6: *const c_char, _5: c_ulong, _4: c_uint, _3: c_uint, _2: c_int, _1: c_int) -> c_int;
  pub fn XXorRegion (_3: Region, _2: Region, _1: Region) -> c_int;
}


//
// types
//


// common types
pub type Atom = XID;
pub type Bool = c_int;
pub type Colormap = XID;
pub type Cursor = XID;
pub type Drawable = XID;
pub type Font = XID;
pub type KeyCode = c_uchar;
pub type KeySym = XID;
pub type Mask = c_ulong;
pub type Pixmap = XID;
pub type Status = Bool;
pub type Time = c_ulong;
pub type VisualID = XID;
pub type Window = XID;
pub type XID = c_ulong;
pub type XPointer = *mut c_void;

// opaque structures
#[repr(C)] pub struct Screen;
#[repr(C)] pub struct Visual;
#[repr(C)] pub struct _XDisplay;
#[repr(C)] pub struct xError;
#[repr(C)] pub struct xEvent;
#[repr(C)] pub struct _XGC;
#[repr(C)] pub struct _XIC;
#[repr(C)] pub struct _XIM;
#[repr(C)] pub struct _XRegion;
#[repr(C)] pub struct _XOC;
#[repr(C)] pub struct _XOM;
#[repr(C)] pub struct _XrmHashBucketRec;

// placeholder structures
#[repr(C)] pub struct _XEDataObjectRec;

// opaque types
pub type Display = _XDisplay;
pub type GC = *mut _XGC;
pub type Region = *mut _XRegion;
pub type XEDataObject = *mut _XEDataObjectRec;
pub type XFontSet = *mut _XOC;
pub type XIC = *mut _XIC;
pub type XIM = *mut _XIM;
pub type XOM = *mut _XOM;
pub type XrmDatabase = *mut _XrmHashBucketRec;
pub type XrmOptionDescList = *mut XrmOptionDescRec;

// function pointers
pub type XConnectionWatchProc = Option<unsafe extern "C" fn (*mut Display, XPointer, c_int, Bool, XPointer)>;


//
// enums
//


#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub enum XICCEncodingStyle {
  XStringStyle,
  XCompoundTextStyle,
  XTextStyle,
  XStdICCTextStyle,
  XUTF8StringStyle,
}

#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub enum XrmBinding {
  XrmBindTightly,
  XrmBindLoosely,
}

#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub enum XrmOptionKind {
  XrmoptionNoArg,
  XrmoptionIsArg,
  XrmoptionStickyArg,
  XrmoptionSepArg,
  XrmoptionResArg,
  XrmoptionSkipArg,
  XrmoptionSkipLine,
  XrmoptionSkipNArgs,
}


//
// event structures
//


#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XEvent {
  pub pad: [c_long; 24],
}

impl XEvent {
  pub fn get_type (&self) -> c_int {
    unsafe {
      *(self as *const XEvent as *const c_int)
    }
  }
}

impl From<XAnyEvent> for XEvent {
  fn from (e: XAnyEvent) -> XEvent {
    unsafe { transmute_union(&e) }
  }
}

impl From<XButtonEvent> for XEvent {
  fn from (e: XButtonEvent) -> XEvent {
    unsafe { transmute_union(&e) }
  }
}

impl From<XCirculateEvent> for XEvent {
  fn from (e: XCirculateEvent) -> XEvent {
    unsafe { transmute_union(&e) }
  }
}

impl From<XCirculateRequestEvent> for XEvent {
  fn from (e: XCirculateRequestEvent) -> XEvent {
    unsafe { transmute_union(&e) }
  }
}

impl From<XClientMessageEvent> for XEvent {
  fn from (e: XClientMessageEvent) -> XEvent {
    unsafe { transmute_union(&e) }
  }
}

impl From<XColormapEvent> for XEvent {
  fn from (e: XColormapEvent) -> XEvent {
    unsafe { transmute_union(&e) }
  }
}

impl From<XConfigureEvent> for XEvent {
  fn from (e: XConfigureEvent) -> XEvent {
    unsafe { transmute_union(&e) }
  }
}

impl From<XConfigureRequestEvent> for XEvent {
  fn from (e: XConfigureRequestEvent) -> XEvent {
    unsafe { transmute_union(&e) }
  }
}

impl From<XCreateWindowEvent> for XEvent {
  fn from (e: XCreateWindowEvent) -> XEvent {
    unsafe { transmute_union(&e) }
  }
}

impl From<XCrossingEvent> for XEvent {
  fn from (e: XCrossingEvent) -> XEvent {
    unsafe { transmute_union(&e) }
  }
}

impl From<XDestroyWindowEvent> for XEvent {
  fn from (e: XDestroyWindowEvent) -> XEvent {
    unsafe { transmute_union(&e) }
  }
}

impl From<XErrorEvent> for XEvent {
  fn from (e: XErrorEvent) -> XEvent {
    unsafe { transmute_union(&e) }
  }
}

impl From<XExposeEvent> for XEvent {
  fn from (e: XExposeEvent) -> XEvent {
    unsafe { transmute_union(&e) }
  }
}

impl From<XFocusChangeEvent> for XEvent {
  fn from (e: XFocusChangeEvent) -> XEvent {
    unsafe { transmute_union(&e) }
  }
}

impl From<XGraphicsExposeEvent> for XEvent {
  fn from (e: XGraphicsExposeEvent) -> XEvent {
    unsafe { transmute_union(&e) }
  }
}

impl From<XGravityEvent> for XEvent {
  fn from (e: XGravityEvent) -> XEvent {
    unsafe { transmute_union(&e) }
  }
}

impl From<XKeyEvent> for XEvent {
  fn from (e: XKeyEvent) -> XEvent {
    unsafe { transmute_union(&e) }
  }
}

impl From<XKeymapEvent> for XEvent {
  fn from (e: XKeymapEvent) -> XEvent {
    unsafe { transmute_union(&e) }
  }
}

impl From<XMapEvent> for XEvent {
  fn from (e: XMapEvent) -> XEvent {
    unsafe { transmute_union(&e) }
  }
}

impl From<XMappingEvent> for XEvent {
  fn from (e: XMappingEvent) -> XEvent {
    unsafe { transmute_union(&e) }
  }
}

impl From<XMotionEvent> for XEvent {
  fn from (e: XMotionEvent) -> XEvent {
    unsafe { transmute_union(&e) }
  }
}

impl From<XNoExposeEvent> for XEvent {
  fn from (e: XNoExposeEvent) -> XEvent {
    unsafe { transmute_union(&e) }
  }
}

impl From<XPropertyEvent> for XEvent {
  fn from (e: XPropertyEvent) -> XEvent {
    unsafe { transmute_union(&e) }
  }
}

impl From<XReparentEvent> for XEvent {
  fn from (e: XReparentEvent) -> XEvent {
    unsafe { transmute_union(&e) }
  }
}

impl From<XResizeRequestEvent> for XEvent {
  fn from (e: XResizeRequestEvent) -> XEvent {
    unsafe { transmute_union(&e) }
  }
}

impl From<XSelectionClearEvent> for XEvent {
  fn from (e: XSelectionClearEvent) -> XEvent {
    unsafe { transmute_union(&e) }
  }
}

impl From<XSelectionEvent> for XEvent {
  fn from (e: XSelectionEvent) -> XEvent {
    unsafe { transmute_union(&e) }
  }
}

impl From<XSelectionRequestEvent> for XEvent {
  fn from (e: XSelectionRequestEvent) -> XEvent {
    unsafe { transmute_union(&e) }
  }
}

impl From<XUnmapEvent> for XEvent {
  fn from (e: XUnmapEvent) -> XEvent {
    unsafe { transmute_union(&e) }
  }
}

impl From<XVisibilityEvent> for XEvent {
  fn from (e: XVisibilityEvent) -> XEvent {
    unsafe { transmute_union(&e) }
  }
}

#[test]
fn xevent_size_test () {
  assert!(size_of::<XEvent>() >= size_of::<XAnyEvent>());
  assert!(size_of::<XEvent>() >= size_of::<XButtonEvent>());
  assert!(size_of::<XEvent>() >= size_of::<XCirculateEvent>());
  assert!(size_of::<XEvent>() >= size_of::<XCirculateRequestEvent>());
  assert!(size_of::<XEvent>() >= size_of::<XClientMessageEvent>());
  assert!(size_of::<XEvent>() >= size_of::<XColormapEvent>());
  assert!(size_of::<XEvent>() >= size_of::<XConfigureEvent>());
  assert!(size_of::<XEvent>() >= size_of::<XConfigureRequestEvent>());
  assert!(size_of::<XEvent>() >= size_of::<XCreateWindowEvent>());
  assert!(size_of::<XEvent>() >= size_of::<XCrossingEvent>());
  assert!(size_of::<XEvent>() >= size_of::<XDestroyWindowEvent>());
  assert!(size_of::<XEvent>() >= size_of::<XErrorEvent>());
  assert!(size_of::<XEvent>() >= size_of::<XExposeEvent>());
  assert!(size_of::<XEvent>() >= size_of::<XFocusChangeEvent>());
  assert!(size_of::<XEvent>() >= size_of::<XGraphicsExposeEvent>());
  assert!(size_of::<XEvent>() >= size_of::<XGravityEvent>());
  assert!(size_of::<XEvent>() >= size_of::<XKeyEvent>());
  assert!(size_of::<XEvent>() >= size_of::<XKeymapEvent>());
  assert!(size_of::<XEvent>() >= size_of::<XMapEvent>());
  assert!(size_of::<XEvent>() >= size_of::<XMappingEvent>());
  assert!(size_of::<XEvent>() >= size_of::<XMapRequestEvent>());
  assert!(size_of::<XEvent>() >= size_of::<XMotionEvent>());
  assert!(size_of::<XEvent>() >= size_of::<XNoExposeEvent>());
  assert!(size_of::<XEvent>() >= size_of::<XPropertyEvent>());
  assert!(size_of::<XEvent>() >= size_of::<XReparentEvent>());
  assert!(size_of::<XEvent>() >= size_of::<XResizeRequestEvent>());
  assert!(size_of::<XEvent>() >= size_of::<XSelectionClearEvent>());
  assert!(size_of::<XEvent>() >= size_of::<XSelectionEvent>());
  assert!(size_of::<XEvent>() >= size_of::<XSelectionRequestEvent>());
  assert!(size_of::<XEvent>() >= size_of::<XUnmapEvent>());
  assert!(size_of::<XEvent>() >= size_of::<XVisibilityEvent>());
}

#[allow(raw_pointer_derive)]
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XAnyEvent {
  pub _type: c_int,
  pub serial: c_ulong,
  pub send_event: Bool,
  pub display: *mut Display,
  pub window: Window,
}

impl From<XEvent> for XAnyEvent {
  fn from (e: XEvent) -> XAnyEvent {
    unsafe { transmute_union(&e) }
  }
}

#[allow(raw_pointer_derive)]
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XButtonEvent {
  pub _type: c_int,
  pub serial: c_ulong,
  pub send_event: Bool,
  pub display: *mut Display,
  pub window: Window,
  pub root: Window,
  pub subwindow: Window,
  pub time: Time,
  pub x: c_int,
  pub y: c_int,
  pub x_root: c_int,
  pub y_root: c_int,
  pub state: c_uint,
  pub button: c_uint,
  pub same_screen: Bool,
}
pub type XButtonPressedEvent = XButtonEvent;
pub type XButtonReleasedEvent = XButtonEvent;

impl From<XEvent> for XButtonEvent {
  fn from (e: XEvent) -> XButtonEvent {
    unsafe { transmute_union(&e) }
  }
}

#[allow(raw_pointer_derive)]
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XCirculateEvent {
  pub _type: c_int,
  pub serial: c_ulong,
  pub send_event: Bool,
  pub display: *mut Display,
  pub event: Window,
  pub window: Window,
  pub place: c_int,
}

impl From<XEvent> for XCirculateEvent {
  fn from (e: XEvent) -> XCirculateEvent {
    unsafe { transmute_union(&e) }
  }
}

#[allow(raw_pointer_derive)]
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XCirculateRequestEvent {
  pub _type: c_int,
  pub serial: c_ulong,
  pub send_event: Bool,
  pub display: *mut Display,
  pub parent: Window,
  pub window: Window,
  pub place: c_int,
}

impl From<XEvent> for XCirculateRequestEvent {
  fn from (e: XEvent) -> XCirculateRequestEvent {
    unsafe { transmute_union(&e) }
  }
}

#[allow(raw_pointer_derive)]
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XClientMessageEvent {
  pub _type: c_int,
  pub serial: c_ulong,
  pub send_event: Bool,
  pub display: *mut Display,
  pub window: Window,
  pub message_type: Atom,
  pub format: c_int,
  pub data: ClientMessageData,
}

impl From<XEvent> for XClientMessageEvent {
  fn from (e: XEvent) -> XClientMessageEvent {
    unsafe { transmute_union(&e) }
  }
}

#[allow(raw_pointer_derive)]
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XColormapEvent {
  pub _type: c_int,
  pub serial: c_ulong,
  pub send_event: Bool,
  pub display: *mut Display,
  pub window: Window,
  pub colormap: Colormap,
  pub new: Bool,
  pub state: c_int,
}

impl From<XEvent> for XColormapEvent {
  fn from (e: XEvent) -> XColormapEvent {
    unsafe { transmute_union(&e) }
  }
}

#[allow(raw_pointer_derive)]
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XConfigureEvent {
  pub _type: c_int,
  pub serial: c_ulong,
  pub send_event: Bool,
  pub display: *mut Display,
  pub event: Window,
  pub window: Window,
  pub x: c_int,
  pub y: c_int,
  pub width: c_int,
  pub height: c_int,
  pub border_width: c_int,
  pub above: Window,
  pub override_redirect: Bool,
}

impl From<XEvent> for XConfigureEvent {
  fn from (e: XEvent) -> XConfigureEvent {
    unsafe { transmute_union(&e) }
  }
}

#[allow(raw_pointer_derive)]
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XConfigureRequestEvent {
  pub _type: c_int,
  pub serial: c_ulong,
  pub send_event: Bool,
  pub display: *mut Display,
  pub parent: Window,
  pub window: Window,
  pub x: c_int,
  pub y: c_int,
  pub width: c_int,
  pub height: c_int,
  pub border_width: c_int,
  pub above: Window,
  pub detail: c_int,
  pub value_mask: c_ulong,
}

impl From<XEvent> for XConfigureRequestEvent {
  fn from (e: XEvent) -> XConfigureRequestEvent {
    unsafe { transmute_union(&e) }
  }
}

#[allow(raw_pointer_derive)]
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XCreateWindowEvent {
  pub _type: c_int,
  pub serial: c_ulong,
  pub send_event: Bool,
  pub display: *mut Display,
  pub parent: Window,
  pub window: Window,
  pub x: c_int,
  pub y: c_int,
  pub width: c_int,
  pub height: c_int,
  pub border_width: c_int,
  pub override_redirect: Bool,
}

impl From<XEvent> for XCreateWindowEvent {
  fn from (e: XEvent) -> XCreateWindowEvent {
    unsafe { transmute_union(&e) }
  }
}

#[allow(raw_pointer_derive)]
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XCrossingEvent {
  pub _type: c_int,
  pub serial: c_ulong,
  pub send_event: Bool,
  pub display: *mut Display,
  pub window: Window,
  pub root: Window,
  pub subwindow: Window,
  pub time: Time,
  pub x: c_int,
  pub y: c_int,
  pub x_root: c_int,
  pub y_root: c_int,
  pub mode: c_int,
  pub detail: c_int,
  pub same_screen: Bool,
  pub focus: Bool,
  pub state: c_uint,
}
pub type XEnterWindowEvent = XCrossingEvent;
pub type XLeaveWindowEvent = XCrossingEvent;

impl From<XEvent> for XCrossingEvent {
  fn from (e: XEvent) -> XCrossingEvent {
    unsafe { transmute_union(&e) }
  }
}

#[allow(raw_pointer_derive)]
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XDestroyWindowEvent {
  pub _type: c_int,
  pub serial: c_ulong,
  pub send_event: Bool,
  pub display: *mut Display,
  pub event: Window,
  pub window: Window,
}

impl From<XEvent> for XDestroyWindowEvent {
  fn from (e: XEvent) -> XDestroyWindowEvent {
    unsafe { transmute_union(&e) }
  }
}

#[allow(raw_pointer_derive)]
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XErrorEvent {
  pub _type: c_int,
  pub display: *mut Display,
  pub serial: c_ulong,
  pub error_code: c_uchar,
  pub request_code: c_uchar,
  pub minor_code: c_uchar,
  pub resourceid: XID,
}

impl From<XEvent> for XErrorEvent {
  fn from (e: XEvent) -> XErrorEvent {
    unsafe { transmute_union(&e) }
  }
}

#[allow(raw_pointer_derive)]
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XExposeEvent {
  pub _type: c_int,
  pub serial: c_ulong,
  pub send_event: Bool,
  pub display: *mut Display,
  pub window: Window,
  pub x: c_int,
  pub y: c_int,
  pub width: c_int,
  pub height: c_int,
  pub count: c_int,
}

impl From<XEvent> for XExposeEvent {
  fn from (e: XEvent) -> XExposeEvent {
    unsafe { transmute_union(&e) }
  }
}

#[allow(raw_pointer_derive)]
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XFocusChangeEvent {
  pub _type: c_int,
  pub serial: c_ulong,
  pub send_event: Bool,
  pub display: *mut Display,
  pub window: Window,
  pub mode: c_int,
  pub detail: c_int,
}
pub type XFocusInEvent = XFocusChangeEvent;
pub type XFocusOutEvent = XFocusChangeEvent;

impl From<XEvent> for XFocusChangeEvent {
  fn from (e: XEvent) -> XFocusChangeEvent {
    unsafe { transmute_union(&e) }
  }
}

#[allow(raw_pointer_derive)]
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XGraphicsExposeEvent {
  pub _type: c_int,
  pub serial: c_ulong,
  pub send_event: Bool,
  pub display: *mut Display,
  pub drawable: Drawable,
  pub x: c_int,
  pub y: c_int,
  pub width: c_int,
  pub height: c_int,
  pub count: c_int,
  pub major_code: c_int,
  pub minor_code: c_int,
}

impl From<XEvent> for XGraphicsExposeEvent {
  fn from (e: XEvent) -> XGraphicsExposeEvent {
    unsafe { transmute_union(&e) }
  }
}

#[allow(raw_pointer_derive)]
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XGravityEvent {
  pub _type: c_int,
  pub serial: c_ulong,
  pub send_event: Bool,
  pub display: *mut Display,
  pub event: Window,
  pub window: Window,
  pub x: c_int,
  pub y: c_int,
}

impl From<XEvent> for XGravityEvent {
  fn from (e: XEvent) -> XGravityEvent {
    unsafe { transmute_union(&e) }
  }
}

#[allow(raw_pointer_derive)]
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XKeyEvent {
  pub _type: c_int,
  pub serial: c_ulong,
  pub send_event: Bool,
  pub display: *mut Display,
  pub window: Window,
  pub root: Window,
  pub subwindow: Window,
  pub time: Time,
  pub x: c_int,
  pub y: c_int,
  pub x_root: c_int,
  pub y_root: c_int,
  pub state: c_uint,
  pub keycode: c_uint,
  pub same_screen: Bool,
}
pub type XKeyPressedEvent = XKeyEvent;
pub type XKeyReleasedEvent = XKeyEvent;

impl From<XEvent> for XKeyEvent {
  fn from (e: XEvent) -> XKeyEvent {
    unsafe { transmute_union(&e) }
  }
}

#[allow(raw_pointer_derive)]
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XKeymapEvent {
  pub _type: c_int,
  pub serial: c_ulong,
  pub send_event: Bool,
  pub display: *mut Display,
  pub window: Window,
  pub key_vector: [c_char; 32],
}

impl From<XEvent> for XKeymapEvent {
  fn from (e: XEvent) -> XKeymapEvent {
    unsafe { transmute_union(&e) }
  }
}

#[allow(raw_pointer_derive)]
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XMapEvent {
  pub _type: c_int,
  pub serial: c_ulong,
  pub send_event: Bool,
  pub display: *mut Display,
  pub event: Window,
  pub window: Window,
  pub override_redirect: Bool,
}

impl From<XEvent> for XMapEvent {
  fn from (e: XEvent) -> XMapEvent {
    unsafe { transmute_union(&e) }
  }
}

#[allow(raw_pointer_derive)]
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XMappingEvent {
  pub _type: c_int,
  pub serial: c_ulong,
  pub send_event: Bool,
  pub display: *mut Display,
  pub event: Window,
  pub request: c_int,
  pub first_keycode: c_int,
  pub count: c_int,
}

impl From<XEvent> for XMappingEvent {
  fn from (e: XEvent) -> XMappingEvent {
    unsafe { transmute_union(&e) }
  }
}

#[allow(raw_pointer_derive)]
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XMapRequestEvent {
  pub _type: c_int,
  pub serial: c_ulong,
  pub send_event: Bool,
  pub display: *mut Display,
  pub parent: Window,
  pub window: Window,
}

impl From<XEvent> for XMapRequestEvent {
  fn from (e: XEvent) -> XMapRequestEvent {
    unsafe { transmute_union(&e) }
  }
}

#[allow(raw_pointer_derive)]
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XMotionEvent {
  pub _type: c_int,
  pub serial: c_ulong,
  pub send_event: Bool,
  pub display: *mut Display,
  pub window: Window,
  pub root: Window,
  pub subwindow: Window,
  pub time: Time,
  pub x: c_int,
  pub y: c_int,
  pub x_root: c_int,
  pub y_root: c_int,
  pub state: c_uint,
  pub is_hint: c_char,
  pub same_screen: Bool,
}
pub type XPointerMovedEvent = XMotionEvent;

impl From<XEvent> for XMotionEvent {
  fn from (e: XEvent) -> XMotionEvent {
    unsafe { transmute_union(&e) }
  }
}

#[allow(raw_pointer_derive)]
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XNoExposeEvent {
  pub _type: c_int,
  pub serial: c_ulong,
  pub send_event: Bool,
  pub display: *mut Display,
  pub drawable: Drawable,
  pub major_code: c_int,
  pub minor_code: c_int,
}

impl From<XEvent> for XNoExposeEvent {
  fn from (e: XEvent) -> XNoExposeEvent {
    unsafe { transmute_union(&e) }
  }
}

#[allow(raw_pointer_derive)]
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XPropertyEvent {
  pub _type: c_int,
  pub serial: c_ulong,
  pub send_event: Bool,
  pub display: *mut Display,
  pub window: Window,
  pub atom: Atom,
  pub time: Time,
  pub state: c_int,
}

impl From<XEvent> for XPropertyEvent {
  fn from (e: XEvent) -> XPropertyEvent {
    unsafe { transmute_union(&e) }
  }
}

#[allow(raw_pointer_derive)]
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XReparentEvent {
  pub _type: c_int,
  pub serial: c_ulong,
  pub send_event: Bool,
  pub display: *mut Display,
  pub event: Window,
  pub window: Window,
  pub parent: Window,
  pub x: c_int,
  pub y: c_int,
  pub override_redirect: Bool,
}

impl From<XEvent> for XReparentEvent {
  fn from (e: XEvent) -> XReparentEvent {
    unsafe { transmute_union(&e) }
  }
}

#[allow(raw_pointer_derive)]
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XResizeRequestEvent {
  pub _type: c_int,
  pub serial: c_ulong,
  pub send_event: Bool,
  pub display: *mut Display,
  pub window: Window,
  pub width: c_int,
  pub height: c_int,
}

impl From<XEvent> for XResizeRequestEvent {
  fn from (e: XEvent) -> XResizeRequestEvent {
    unsafe { transmute_union(&e) }
  }
}

#[allow(raw_pointer_derive)]
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XSelectionClearEvent {
  pub _type: c_int,
  pub serial: c_ulong,
  pub send_event: Bool,
  pub display: *mut Display,
  pub window: Window,
  pub selection: Atom,
  pub time: Time,
}

impl From<XEvent> for XSelectionClearEvent {
  fn from (e: XEvent) -> XSelectionClearEvent {
    unsafe { transmute_union(&e) }
  }
}

#[allow(raw_pointer_derive)]
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XSelectionEvent {
  pub _type: c_int,
  pub serial: c_ulong,
  pub send_event: Bool,
  pub display: *mut Display,
  pub requestor: Window,
  pub selection: Atom,
  pub target: Atom,
  pub property: Atom,
  pub time: Time,
}

impl From<XEvent> for XSelectionEvent {
  fn from (e: XEvent) -> XSelectionEvent {
    unsafe { transmute_union(&e) }
  }
}

#[allow(raw_pointer_derive)]
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XSelectionRequestEvent {
  pub _type: c_int,
  pub serial: c_ulong,
  pub send_event: Bool,
  pub display: *mut Display,
  pub owner: Window,
  pub requestor: Window,
  pub selection: Atom,
  pub target: Atom,
  pub property: Atom,
  pub time: Time,
}

impl From<XEvent> for XSelectionRequestEvent {
  fn from (e: XEvent) -> XSelectionRequestEvent {
    unsafe { transmute_union(&e) }
  }
}

#[allow(raw_pointer_derive)]
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XUnmapEvent {
  pub _type: c_int,
  pub serial: c_ulong,
  pub send_event: Bool,
  pub display: *mut Display,
  pub event: Window,
  pub window: Window,
  pub from_configure: Bool,
}

impl From<XEvent> for XUnmapEvent {
  fn from (e: XEvent) -> XUnmapEvent {
    unsafe { transmute_union(&e) }
  }
}

#[allow(raw_pointer_derive)]
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XVisibilityEvent {
  pub _type: c_int,
  pub serial: c_ulong,
  pub send_event: Bool,
  pub display: *mut Display,
  pub window: Window,
  pub state: c_int,
}

impl From<XEvent> for XVisibilityEvent {
  fn from (e: XEvent) -> XVisibilityEvent {
    unsafe { transmute_union(&e) }
  }
}


//
// other structures
//


#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XArc {
  pub x: c_short,
  pub y: c_short,
  pub width: c_ushort,
  pub height: c_ushort,
  pub angle1: c_short,
  pub angle2: c_short,
}

#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XChar2b {
  pub byte1: c_uchar,
  pub byte2: c_uchar,
}

#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XCharStruct {
  pub lbearing: c_short,
  pub rbearing: c_short,
  pub width: c_short,
  pub ascent: c_short,
  pub descent: c_short,
  pub attributes: c_ushort,
}

#[allow(raw_pointer_derive)]
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XClassHint {
  pub res_name: *mut c_char,
  pub res_class: *mut c_char,
}

#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XColor {
  pub pixel: c_ulong,
  pub red: c_ushort,
  pub green: c_ushort,
  pub blue: c_ushort,
  pub flags: c_char,
  pub pad: c_char,
}

#[allow(raw_pointer_derive)]
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XComposeStatus {
  pub compose_ptr: XPointer,
  pub chars_matched: c_int,
}

#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XExtCodes {
  pub extension: c_int,
  pub major_opcode: c_int,
  pub first_event: c_int,
  pub first_error: c_int,
}

#[allow(raw_pointer_derive)]
#[repr(C)]
pub struct XExtData {
  pub number: c_int,
  pub next: *mut XExtData,
  pub free_private: Option<unsafe extern "C" fn () -> c_int>,
  pub private_data: XPointer,
}

#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XFontProp {
  pub name: Atom,
  pub card32: c_ulong,
}

#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XFontSetExtents {
  pub max_ink_extent: XRectangle,
  pub max_logical_extent: XRectangle,
}

#[allow(raw_pointer_derive)]
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XFontStruct {
  pub ext_data: *mut XExtData,
  pub fid: Font,
  pub direction: c_uint,
  pub min_char_or_byte2: c_uint,
  pub max_char_or_byte2: c_uint,
  pub min_byte1: c_uint,
  pub max_byte1: c_uint,
  pub all_chars_exist: Bool,
  pub default_char: c_uint,
  pub n_properties: c_int,
  pub properties: *mut XFontProp,
  pub min_bounds: XCharStruct,
  pub max_bounds: XCharStruct,
  pub per_char: *mut XCharStruct,
  pub ascent: c_int,
  pub descent: c_int,
}

#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XGCValues {
  pub function: c_int,
  pub plane_mask: c_ulong,
  pub foreground: c_ulong,
  pub background: c_ulong,
  pub line_width: c_int,
  pub line_style: c_int,
  pub cap_style: c_int,
  pub join_style: c_int,
  pub fill_style: c_int,
  pub fill_rule: c_int,
  pub arc_mode: c_int,
  pub tile: Pixmap,
  pub stipple: Pixmap,
  pub ts_x_origin: c_int,
  pub ts_y_origin: c_int,
  pub font: Font,
  pub subwindow_mode: c_int,
  pub graphics_exposures: Bool,
  pub clip_x_origin: c_int,
  pub clip_y_origin: c_int,
  pub clip_mask: Pixmap,
  pub dash_offset: c_int,
  pub dashes: c_char,
}

#[allow(raw_pointer_derive)]
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XGenericEventCookie {
  pub _type: c_int,
  pub serial: c_ulong,
  pub send_event: Bool,
  pub display: *mut Display,
  pub extension: c_int,
  pub evtype: c_int,
  pub cookie: c_uint,
  pub data: *mut c_void,
}

#[allow(raw_pointer_derive)]
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XHostAddress {
  pub family: c_int,
  pub length: c_int,
  pub address: *mut c_char,
}

#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XIconSize {
  pub min_width: c_int,
  pub min_height: c_int,
  pub max_width: c_int,
  pub max_height: c_int,
  pub width_inc: c_int,
  pub height_inc: c_int,
}

#[allow(raw_pointer_derive)]
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XImage {
  pub width: c_int,
  pub height: c_int,
  pub xoffset: c_int,
  pub format: c_int,
  pub data: *mut c_char,
  pub byte_order: c_int,
  pub bitmap_unity: c_int,
  pub bitmap_bit_order: c_int,
  pub bitmap_pad: c_int,
  pub depth: c_int,
  pub bytes_per_line: c_int,
  pub bits_per_pixel: c_int,
  pub red_mask: c_ulong,
  pub green_mask: c_ulong,
  pub blue_mask: c_ulong,
  pub obdata: XPointer,
  pub funcs: ImageFns,
}

#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XKeyboardControl {
  pub key_click_percent: c_int,
  pub bell_percent: c_int,
  pub bell_pitch: c_int,
  pub bell_duration: c_int,
  pub led: c_int,
  pub led_mode: c_int,
  pub key: c_int,
  pub auto_repeat_mode: c_int,
}

#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XKeyboardState {
  pub key_click_percent: c_int,
  pub bell_percent: c_int,
  pub bell_pitch: c_uint,
  pub bell_duration: c_uint,
  pub led_mask: c_ulong,
  pub global_auto_repeat: c_int,
  pub auto_repeats: [c_char; 32],
}

#[allow(raw_pointer_derive)]
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XmbTextItem {
  pub chars: *mut c_char,
  pub nchars: c_int,
  pub delta: c_int,
  pub font_set: XFontSet,
}

#[allow(raw_pointer_derive)]
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XModifierKeymap {
  pub max_keypermod: c_int,
  pub modifiermap: *mut KeyCode,
}

#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XPixmapFormatValues {
  pub depth: c_int,
  pub bits_per_pixel: c_int,
  pub scanline_pad: c_int,
}

#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XPoint {
  pub x: c_short,
  pub y: c_short,
}

#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XRectangle {
  pub x: c_short,
  pub y: c_short,
  pub width: c_ushort,
  pub height: c_ushort,
}

#[allow(raw_pointer_derive)]
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XrmOptionDescRec {
  pub option: *mut c_char,
  pub specifier: *mut c_char,
  pub argKind: XrmOptionKind,
  pub value: XPointer,
}

#[allow(raw_pointer_derive)]
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XrmValue {
  pub size: c_uint,
  pub addr: XPointer,
}

#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XSegment {
  pub x1: c_short,
  pub y1: c_short,
  pub x2: c_short,
  pub y2: c_short,
}

#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XSetWindowAttributes {
  pub background_pixmap: Pixmap,
  pub background_pixel: c_ulong,
  pub border_pixmap: Pixmap,
  pub border_pixel: c_ulong,
  pub bit_gravity: c_int,
  pub win_gravity: c_int,
  pub backing_store: c_int,
  pub backing_planes: c_ulong,
  pub backing_pixel: c_ulong,
  pub save_under: Bool,
  pub event_mask: c_long,
  pub do_not_propagate_mask: c_long,
  pub override_redirect: Bool,
  pub colormap: Colormap,
  pub cursor: Cursor,
}

#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XSizeHints {
  pub flags: c_long,
  pub x: c_int,
  pub y: c_int,
  pub width: c_int,
  pub height: c_int,
  pub min_width: c_int,
  pub min_height: c_int,
  pub max_width: c_int,
  pub max_height: c_int,
  pub width_inc: c_int,
  pub height_inc: c_int,
  pub min_aspect: AspectRatio,
  pub max_aspect: AspectRatio,
  pub base_width: c_int,
  pub base_height: c_int,
  pub win_gravity: c_int,
}

#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XStandardColormap {
  pub colormap: Colormap,
  pub red_max: c_ulong,
  pub red_mult: c_ulong,
  pub green_max: c_ulong,
  pub green_mult: c_ulong,
  pub blue_max: c_ulong,
  pub blue_mult: c_ulong,
  pub base_pixel: c_ulong,
  pub visualid: VisualID,
  pub killid: XID,
}

#[allow(raw_pointer_derive)]
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XTextItem {
  pub chars: *mut c_char,
  pub nchars: c_int,
  pub delta: c_int,
  pub font: Font,
}

#[allow(raw_pointer_derive)]
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XTextItem16 {
  pub chars: *mut XChar2b,
  pub nchars: c_int,
  pub delta: c_int,
  pub font: Font,
}

#[allow(raw_pointer_derive)]
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XTextProperty {
  pub value: *mut c_uchar,
  pub encoding: Atom,
  pub format: c_int,
  pub nitems: c_ulong,
}

#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XTimeCoord {
  pub time: Time,
  pub x: c_short,
  pub y: c_short,
}

#[allow(raw_pointer_derive)]
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XVisualInfo {
  pub visual: *mut Visual,
  pub visualid: VisualID,
  pub screen: c_int,
  pub depth: c_uint,
  pub class: c_int,
  pub red_mask: c_ulong,
  pub green_mask: c_ulong,
  pub blue_mask: c_ulong,
  pub colormap_size: c_int,
  pub bits_per_rgb: c_int,
}

#[allow(raw_pointer_derive)]
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XwcTextItem {
  pub chars: *mut wchar_t,
  pub nchars: c_int,
  pub delta: c_int,
  pub font_set: XFontSet,
}

#[allow(raw_pointer_derive)]
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XWindowAttributes {
  pub x: c_int,
  pub y: c_int,
  pub width: c_int,
  pub height: c_int,
  pub border_width: c_int,
  pub depth: c_int,
  pub visual: *mut Visual,
  pub root: Window,
  pub class: c_int,
  pub bit_gravity: c_int,
  pub win_gravity: c_int,
  pub backing_store: c_int,
  pub backing_planes: c_ulong,
  pub backing_pixel: c_ulong,
  pub save_under: Bool,
  pub colormap: Colormap,
  pub map_installed: Bool,
  pub map_state: c_int,
  pub all_event_masks: c_long,
  pub your_event_mask: c_long,
  pub do_not_propagate_mask: c_long,
  pub override_redirect: Bool,
  pub screen: *mut Screen,
}

#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XWindowChanges {
  pub x: c_int,
  pub y: c_int,
  pub width: c_int,
  pub height: c_int,
  pub border_width: c_int,
  pub sibling: Window,
  pub stack_mode: c_int,
}

#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XWMHints {
  pub flags: c_long,
  pub input: Bool,
  pub initial_state: c_int,
  pub icon_pixmap: Pixmap,
  pub icon_window: Window,
  pub icon_x: c_int,
  pub icon_y: c_int,
  pub icon_mask: Pixmap,
  pub window_group: XID,
}


//
// anonymous structures
//


#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct AspectRatio {
  pub x: c_int,
  pub y: c_int,
}

#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ClientMessageData {
  longs: [c_long; 5],
}

impl ClientMessageData {
  pub fn get_byte (&self, index: usize) -> c_char {
    unsafe {
      from_raw_parts(&self.longs[0] as *const c_long as *const c_char, 20)[index]
    }
  }

  pub fn get_long (&self, index: usize) -> c_long {
    self.longs[index]
  }

  pub fn get_short (&self, index: usize) -> c_short {
    unsafe {
      from_raw_parts(&self.longs[0] as *const c_long as *const c_short, 10)[index]
    }
  }

  pub fn set_byte (&mut self, index: usize, value: c_char) {
    unsafe {
      from_raw_parts_mut(&mut self.longs[0] as *mut c_long as *mut c_char, 20)[index] = value;
    }
  }

  pub fn set_long (&mut self, index: usize, value: c_long) {
    self.longs[index] = value;
  }

  pub fn set_short (&mut self, index: usize, value: c_short) {
    unsafe {
      from_raw_parts_mut(&mut self.longs[0] as *mut c_long as *mut c_short, 10)[index] = value;
    }
  }
}

#[test]
fn client_message_size_test () {
  assert!(size_of::<ClientMessageData>() >= size_of::<[c_char; 20]>());
  assert!(size_of::<ClientMessageData>() >= size_of::<[c_short; 10]>());
}

#[derive(Copy)]
#[repr(C)]
pub struct ImageFns {
  pub create_image: Option<unsafe extern "C" fn (*mut Display, *mut Visual, c_uint, c_int, c_int, *mut c_char, c_uint, c_uint, c_int, c_int) -> *mut XImage>,
  pub destroy_image: Option<unsafe extern "C" fn (*mut XImage) -> c_int>,
  pub get_pixel: Option<unsafe extern "C" fn (*mut XImage, c_int, c_int) -> c_ulong>,
  pub put_pixel: Option<unsafe extern "C" fn (*mut XImage, c_int, c_int, c_ulong) -> c_int>,
  pub sub_image: Option<unsafe extern "C" fn (*mut XImage, c_int, c_int, c_uint, c_uint) -> *mut XImage>,
  pub add_pixel: Option<unsafe extern "C" fn (&mut XImage, c_long) -> c_int>,
}

impl Clone for ImageFns {
  fn clone (&self) -> ImageFns {
    *self
  }
}

impl PartialEq for ImageFns {
  fn eq (&self, rhs: &ImageFns) -> bool {
    unsafe { mem_eq(self, rhs) }
  }
}


//
// constants
//


// allocate colormap
pub const AllocNone: c_int = 0;
pub const AllocAll: c_int = 1;

// atoms
pub const XA_PRIMARY: Atom = 1;
pub const XA_SECONDARY: Atom = 2;
pub const XA_ARC: Atom = 3;
pub const XA_ATOM: Atom = 4;
pub const XA_BITMAP: Atom = 5;
pub const XA_CARDINAL: Atom = 6;
pub const XA_COLORMAP: Atom = 7;
pub const XA_CURSOR: Atom = 8;
pub const XA_CUT_BUFFER0: Atom = 9;
pub const XA_CUT_BUFFER1: Atom = 10;
pub const XA_CUT_BUFFER2: Atom = 11;
pub const XA_CUT_BUFFER3: Atom = 12;
pub const XA_CUT_BUFFER4: Atom = 13;
pub const XA_CUT_BUFFER5: Atom = 14;
pub const XA_CUT_BUFFER6: Atom = 15;
pub const XA_CUT_BUFFER7: Atom = 16;
pub const XA_DRAWABLE: Atom = 17;
pub const XA_FONT: Atom = 18;
pub const XA_INTEGER: Atom = 19;
pub const XA_PIXMAP: Atom = 20;
pub const XA_POINT: Atom = 21;
pub const XA_RECTANGLE: Atom = 22;
pub const XA_RESOURCE_MANAGER: Atom = 23;
pub const XA_RGB_COLOR_MAP: Atom = 24;
pub const XA_RGB_BEST_MAP: Atom = 25;
pub const XA_RGB_BLUE_MAP: Atom = 26;
pub const XA_RGB_DEFAULT_MAP: Atom = 27;
pub const XA_RGB_GRAY_MAP: Atom = 28;
pub const XA_RGB_GREEN_MAP: Atom = 29;
pub const XA_RGB_RED_MAP: Atom = 30;
pub const XA_STRING: Atom = 31;
pub const XA_VISUALID: Atom = 32;
pub const XA_WINDOW: Atom = 33;
pub const XA_WM_COMMAND: Atom = 34;
pub const XA_WM_HINTS: Atom = 35;
pub const XA_WM_CLIENT_MACHINE: Atom = 36;
pub const XA_WM_ICON_NAME: Atom = 37;
pub const XA_WM_ICON_SIZE: Atom = 38;
pub const XA_WM_NAME: Atom = 39;
pub const XA_WM_NORMAL_HINTS: Atom = 40;
pub const XA_WM_SIZE_HINTS: Atom = 41;
pub const XA_WM_ZOOM_HINTS: Atom = 42;
pub const XA_MIN_SPACE: Atom = 43;
pub const XA_NORM_SPACE: Atom = 44;
pub const XA_MAX_SPACE: Atom = 45;
pub const XA_END_SPACE: Atom = 46;
pub const XA_SUPERSCRIPT_X: Atom = 47;
pub const XA_SUPERSCRIPT_Y: Atom = 48;
pub const XA_SUBSCRIPT_X: Atom = 49;
pub const XA_SUBSCRIPT_Y: Atom = 50;
pub const XA_UNDERLINE_POSITION: Atom = 51;
pub const XA_UNDERLINE_THICKNESS: Atom = 52;
pub const XA_STRIKEOUT_ASCENT: Atom = 53;
pub const XA_STRIKEOUT_DESCENT: Atom = 54;
pub const XA_ITALIC_ANGLE: Atom = 55;
pub const XA_X_HEIGHT: Atom = 56;
pub const XA_QUAD_WIDTH: Atom = 57;
pub const XA_WEIGHT: Atom = 58;
pub const XA_POINT_SIZE: Atom = 59;
pub const XA_RESOLUTION: Atom = 60;
pub const XA_COPYRIGHT: Atom = 61;
pub const XA_NOTICE: Atom = 62;
pub const XA_FONT_NAME: Atom = 63;
pub const XA_FAMILY_NAME: Atom = 64;
pub const XA_FULL_NAME: Atom = 65;
pub const XA_CAP_HEIGHT: Atom = 66;
pub const XA_WM_CLASS: Atom = 67;
pub const XA_WM_TRANSIENT_FOR: Atom = 68;

// boolean values
pub const False: Bool = 0;
pub const True: Bool = 1;

// clip rect ordering
pub const Unsorted: c_int = 0;
pub const YSorted: c_int = 1;
pub const YXSorted: c_int = 2;
pub const YXBanded: c_int = 3;

// color component mask
pub const DoRed: c_char = 1;
pub const DoGreen: c_char = 2;
pub const DoBlue: c_char = 4;

// error codes
pub const Success: c_int = 0;
pub const BadRequest: c_int = 1;
pub const BadValue: c_int = 2;
pub const BadWindow: c_int = 3;
pub const BadPixmap: c_int = 4;
pub const BadAtom: c_int = 5;
pub const BadCursor: c_int = 6;
pub const BadFont: c_int = 7;
pub const BadMatch: c_int = 8;
pub const BadDrawable: c_int = 9;
pub const BadAccess: c_int = 10;
pub const BadAlloc: c_int = 11;
pub const BadColor: c_int = 12;
pub const BadGC: c_int = 13;
pub const BadIDChoice: c_int = 14;
pub const BadName: c_int = 15;
pub const BadLength: c_int = 16;
pub const BadImplementation: c_int = 17;
pub const FirstExtensionError: c_int = 128;
pub const LastExtensionError: c_int = 255;

// event kinds
pub const KeyPress: c_int = 2;
pub const KeyRelease: c_int = 3;
pub const ButtonPress: c_int = 4;
pub const ButtonRelease: c_int = 5;
pub const MotionNotify: c_int = 6;
pub const EnterNotify: c_int = 7;
pub const LeaveNotify: c_int = 8;
pub const FocusIn: c_int = 9;
pub const FocusOut: c_int = 10;
pub const KeymapNotify: c_int = 11;
pub const Expose: c_int = 12;
pub const GraphicsExpose: c_int = 13;
pub const NoExpose: c_int = 14;
pub const VisibilityNotify: c_int = 15;
pub const CreateNotify: c_int = 16;
pub const DestroyNotify: c_int = 17;
pub const UnmapNotify: c_int = 18;
pub const MapNotify: c_int = 19;
pub const MapRequest: c_int = 20;
pub const ReparentNotify: c_int = 21;
pub const ConfigureNotify: c_int = 22;
pub const ConfigureRequest: c_int = 23;
pub const GravityNotify: c_int = 24;
pub const ResizeRequest: c_int = 25;
pub const CirculateNotify: c_int = 26;
pub const CirculateRequest: c_int = 27;
pub const PropertyNotify: c_int = 28;
pub const SelectionClear: c_int = 29;
pub const SelectionRequest: c_int = 30;
pub const SelectionNotify: c_int = 31;
pub const ColormapNotify: c_int = 32;
pub const ClientMessage: c_int = 33;
pub const MappingNotify: c_int = 34;

// event mask
pub const NoEventMask: c_long = 0;
pub const KeyPressMask: c_long = 0x0000_0001;
pub const KeyReleaseMask: c_long = 0x0000_0002;
pub const ButtonPressMask: c_long = 0x0000_0004;
pub const ButtonReleaseMask: c_long = 0x0000_0008;
pub const EnterWindowMask: c_long = 0x0000_0010;
pub const LeaveWindowMask: c_long = 0x0000_0020;
pub const PointerMotionMask: c_long = 0x0000_0040;
pub const PointerMotionHintMask: c_long = 0x0000_0080;
pub const Button1MotionMask: c_long = 0x0000_0100;
pub const Button2MotionMask: c_long = 0x0000_0200;
pub const Button3MotionMask: c_long = 0x0000_0400;
pub const Button4MotionMask: c_long = 0x0000_0800;
pub const Button5MotionMask: c_long = 0x0000_1000;
pub const ButtonMotionMask: c_long = 0x0000_2000;
pub const KeymapStateMask: c_long = 0x0000_4000;
pub const ExposureMask: c_long = 0x0000_8000;
pub const VisibilityChangeMask: c_long = 0x0001_0000;
pub const StructureNotifyMask: c_long = 0x0002_0000;
pub const ResizeRedirectMask: c_long = 0x0004_0000;
pub const SubstructureNotifyMask: c_long = 0x0008_0000;
pub const SubstructureRedirectMask: c_long = 0x0010_0000;
pub const FocusChangeMask: c_long = 0x0020_0000;
pub const PropertyChangeMask: c_long = 0x0040_0000;
pub const ColormapChangeMask: c_long = 0x0080_0000;
pub const OwnerGrabButtonMask: c_long = 0x0100_0000;

// grab modes
pub const GrabModeSync: c_int = 0;
pub const GrabModeAsync: c_int = 1;

// grab status
pub const GrabSuccess: c_int = 0;
pub const AlreadyGrabbed: c_int = 1;
pub const GrabInvalidTime: c_int = 2;
pub const GrabNotViewable: c_int = 3;
pub const GrabFrozen: c_int = 4;

// map state
pub const IsUnmapped: c_int = 0;
pub const IsUnviewable: c_int = 1;
pub const IsViewable: c_int = 2;

// mouse buttons
pub const Button1: c_uint = 1;
pub const Button2: c_uint = 2;
pub const Button3: c_uint = 3;
pub const Button4: c_uint = 4;
pub const Button5: c_uint = 5;

// size hints mask
pub const USPosition: c_long = 0x0001;
pub const USSize: c_long = 0x0002;
pub const PPosition: c_long = 0x0004;
pub const PSize: c_long = 0x0008;
pub const PMinSize: c_long = 0x0010;
pub const PMaxSize: c_long = 0x0020;
pub const PResizeInc: c_long = 0x0040;
pub const PAspect: c_long = 0x0080;
pub const PBaseSize: c_long = 0x0100;
pub const PWinGravity: c_long = 0x0200;
pub const PAllHints: c_long = PPosition | PSize | PMinSize | PMaxSize | PResizeInc | PAspect;

// time constants
pub const CurrentTime: Time = 0;

// visual class
pub const StaticGray: c_int = 0;
pub const GrayScale: c_int = 1;
pub const StaticColor: c_int = 2;
pub const PseudoColor: c_int = 3;
pub const TrueColor: c_int = 4;
pub const DirectColor: c_int = 5;

// visual info mask
pub const VisualNoMask: c_long = 0x0000;
pub const VisualIDMask: c_long = 0x0001;
pub const VisualScreenMask: c_long = 0x0002;
pub const VisualDepthMask: c_long = 0x0004;
pub const VisualClassMask: c_long = 0x0008;
pub const VisualRedMaskMask: c_long = 0x0010;
pub const VisualGreenMaskMask: c_long = 0x0020;
pub const VisualBlueMaskMask: c_long = 0x0040;
pub const VisualColormapSizeMask: c_long = 0x0080;
pub const VisualBitsPerRGBMask: c_long = 0x0100;
pub const VisualAllMask: c_long = 0x01ff;

// window attributes
pub const CWBackPixmap: c_ulong = 0x0001;
pub const CWBackPixel: c_ulong = 0x0002;
pub const CWBorderPixmap: c_ulong = 0x0004;
pub const CWBorderPixel: c_ulong = 0x0008;
pub const CWBitGravity: c_ulong = 0x0010;
pub const CWWinGravity: c_ulong = 0x0020;
pub const CWBackingStore: c_ulong = 0x0040;
pub const CWBackingPlanes: c_ulong = 0x0080;
pub const CWBackingPixel: c_ulong = 0x0100;
pub const CWOverrideRedirect: c_ulong = 0x0200;
pub const CWSaveUnder: c_ulong = 0x0400;
pub const CWEventMask: c_ulong = 0x0800;
pub const CWDontPropagate: c_ulong = 0x1000;
pub const CWColormap: c_ulong = 0x2000;
pub const CWCursor: c_ulong = 0x4000;

// window classes
pub const InputOutput: c_int = 1;
pub const InputOnly: c_int = 2;

// XCreateIC values
pub const XIMPreeditArea: c_int = 0x0001;
pub const XIMPreeditCallbacks: c_int = 0x0002;
pub const XIMPreeditPosition: c_int = 0x0004;
pub const XIMPreeditNothing: c_int = 0x0008;
pub const XIMPreeditNone: c_int = 0x0010;
pub const XIMStatusArea: c_int = 0x0100;
pub const XIMStatusCallbacks: c_int = 0x0200;
pub const XIMStatusNothing: c_int = 0x0400;
pub const XIMStatusNone: c_int = 0x0800;


//
// private functions
//


unsafe fn mem_eq<T: Sized> (a: &T, b: &T) -> bool {
  let a_addr = a as *const T as usize;
  let b_addr = b as *const T as usize;

  for i in (0..size_of::<T>()) {
    if *((a_addr + i) as *const u8) != *((b_addr + i) as *const u8) {
      return false;
    }
  }

  return true;
}

unsafe fn transmute_union<I, O> (input: &I) -> O
  where I : Sized, O : Sized
{
  let mut output: O = zeroed();
  let copy_len = min(size_of::<I>(), size_of::<O>());

  for i in 0..copy_len {
    *((&mut output as *mut O as usize + i) as *mut u8) = *((input as *const I as usize + i) as *const u8);
  }

  return output;
}
