// Copyright (c) 2015, <daggerbot@gmail.com>
// All rights reserved.

use std::cmp::min;
use std::mem::{
  size_of,
  zeroed,
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
};


//
// functions
//


#[link(name="X11")]
extern "C" {
  // _XSetLastRequestRead
  // AllPlanes
  // BitmapBitOrder
  // BitmapPad
  // BitmapUnit
  // BlackPixel
  // BlackPixelOfScreen
  // CellsOfScreen
  // ClientWhitePointOfCCC
  // ConnectionNumber
  // DefaultColormap
  // DefaultColormapOfScreen
  // DefaultDepth
  // DefaultDepthOfScreen
  // DefaultGC
  // DefaultGCOfScreen
  // DefaultRootWindow
  // DefaultScreen
  // DefaultScreenOfDisplay
  // DefaultVisual
  // DefaultVisualOfScreen
  // DisplayCells
  // DisplayHeight
  // DisplayHeightMM
  // DisplayOfCCC
  // DisplayOfScreen
  // DisplayPlanes
  // DisplayString
  // DisplayWidth
  // DisplayWidthMM
  // DoesBackingStore
  // DoesSaveUnders
  // EventMaskOfScreen
  // HeightMMOfScreen
  // HeightOfScreen
  // ImageByteOrder
  // InitExtension
  // IsCursorKey
  // IsFunctionKey
  // IsKeypadKey
  // IsMiscFunctionKey
  // IsModifierKey
  // IsPFKey
  // IsPrivateKeypadKey
  // LastKnownRequestProcessed
  // MaxCmapsOfScreen
  // MinCmapsOfScreen
  // NextRequest
  // PlanesOfScreen
  // ProtocolRevision
  // ProtocolVersion
  // QLength
  // RootWindow
  // RootWindowOfScreen
  // ScreenCount
  // ScreenNumberOfCCC
  // ScreensOfDisplay
  // ScreenWhitePointOfCCC
  // ServerVendor
  // VendorRelease
  // VertexDrawLastPoint
  // VisualOfCCC
  // WhitePixel
  // WhitePixelOfScreen
  // WidthMMOfScreen
  // WidthOfScreen
  // XActivateScreenSaver
  // XAddConnectionWatch
  // XAddExtension
  // XAddHost
  // XAddHosts
  // XAddPixel
  // XAddToSaveSet
  // XAllocClassHint
  pub fn XAllocColor (display: *mut Display, colormap: Colormap, color: *mut XColor) -> Status;
  // XAllocColorCells
  // XAllocColorPlanes
  // XAllocIconSize
  // XAllocNamedColor
  // XAllocSizeHints
  // XAllocStandardColormap
  // XAllocWMHints
  // XAllowEvents
  // XAllPlanes
  // XAppendVertex
  // XAutoRepeatOff
  // XAutoRepeatOn
  // XBell
  // XBitmapBitOrder
  // XBitmapPad
  // XBitmapUnit
  pub fn XBlackPixel (display: *mut Display, screen_num: c_int) -> c_ulong;
  // XBlackPixelOfScreen
  // XCellsOfScreen
  // XChangeActivePointerGrab
  // XChangeGC
  // XChangeKeyboardControl
  // XChangeKeyboardMapping
  // XChangePointerControl
  // XChangeProperty
  // XChangeSaveSet
  // XChangeWindowAttributes
  // XCheckIfEvent
  // XCheckMaskEvent
  // XCheckTypedEvent
  // XCheckTypedWindowEvent
  // XCheckWindowEvent
  // XCirculateSubwindows
  // XCirculateSubwindowsDown
  // XCirculateSubwindowsUp
  // XClearArea
  // XClearVertexFlag
  // XClearWindow
  // XClipBox
  pub fn XCloseDisplay (display: *mut Display);
  // XcmsAddColorSpace
  // XcmsAddFunctionSet
  // XcmsAllocColor
  // XcmsAllocNamedColor
  // XcmsCCCOfColormap
  // XcmsCIELabQueryMaxC
  // XcmsCIELabQueryMaxL
  // XcmsCIELabQueryMaxLC
  // XcmsCIELabQueryMinL
  // XcmsCIELabToCIEXYZ
  // XcmsCIELuvQueryMaxC
  // XcmsCIELuvQueryMaxL
  // XcmsCIELuvQueryMaxLC
  // XcmsCIELuvQueryMinL
  // XcmsCIELuvToCIEuvY
  // XcmsCIEuvYToCIELuv
  // XcmsCIEuvYToCIEXYZ
  // XcmsCIEuvYToTekHVC
  // XcmsCIExyYToCIEXYZ
  // XcmsCIEXYZToCIELab
  // XcmsCIEXYZToCIEuvY
  // XcmsCIEXYZToCIExyY
  // XcmsCIEXYZToRGBi
  // XcmsClientWhitePointOfCCC
  // XcmsConvertColors
  // XcmsCreateCCC
  // XcmsDefaultCCC
  // XcmsDisplayOfCCC
  // XcmsFormatOfPrefix
  // XcmsFreeCCC
  // XcmsLookupColor
  // XcmsPrefixOfFormat
  // XcmsQueryBlack
  // XcmsQueryBlue
  // XcmsQueryColor
  // XcmsQueryColors
  // XcmsQueryGreen
  // XcmsQueryRed
  // XcmsQueryWhite
  // XcmsRGBiToCIEXYZ
  // XcmsRGBiToRGB
  // XcmsRGBToRGBi
  // XcmsScreenNumberOfCCC
  // XcmsScreenWhitePointOfCCC
  // XcmsSetCCCOfColormap
  // XcmsSetCompressionProc
  // XcmsSetWhiteAdjustProc
  // XcmsSetWhitePoint
  // XcmsStoreColor
  // XcmsStoreColors
  // XcmsTekHVCQueryMaxC
  // XcmsTekHVCQueryMaxV
  // XcmsTekHVCQueryMaxVC
  // XcmsTekHVCQueryMaxVSamples
  // XcmsTekHVCQueryMinV
  // XcmsTekHVCToCIEuvY
  // XcmsVisualOfCCC
  // XConfigureWindow
  // XConnectionNumber
  // XConvertCase
  // XConvertSelection
  // XCopyArea
  // XCopyColormapAndFree
  // XCopyGC
  // XCopyPlane
  // XCreateAssocTable
  // XCreateBitmapFromData
  // XCreateColormap
  // XCreateFontCursor
  // XCreateFontSet
  // XCreateGC
  // XCreateGlyphCursor
  // XCreateImage
  // XCreatePixmap
  // XCreatePixmapCursor
  // XCreatePixmapFromBitmapData
  // XCreateRegion
  pub fn XCreateSimpleWindow (display: *mut Display, parent: Window, x: c_int, y: c_int, width: c_uint, height: c_uint,
      border_width: c_uint, border: c_ulong, background: c_ulong) -> Window;
  pub fn XCreateWindow (display: *mut Display, parent: Window, x: c_int, y: c_int, width: c_uint, height: c_uint,
      border_width: c_uint, depth: c_int, class: c_int, visual: *const Visual, attr_mask: c_ulong,
      attr: *const XSetWindowAttributes) -> Window;
  pub fn XDefaultColormap (display: *mut Display, screen_num: c_int) -> Colormap;
  // XDefaultColormapOfScreen
  // XDefaultDepthOfScreen
  // XDefaultGC
  // XDefaultGCOfScreen
  // XDefaultRootWindow
  pub fn XDefaultScreen (display: *mut Display) -> c_int;
  // XDefaultScreenOfDisplay
  // XDefaultString
  pub fn XDefaultVisual (display: *mut Display, screen_num: c_int) -> *const Visual;
  // XDefaultVisualOfScreen
  // XDefineCursor
  // XDeleteAssoc
  // XDeleteContext
  // XDeleteModifiermapEntry
  // XDeleteProperty
  // XDestroyAssocTable
  // XDestroyImage
  // XDestroyRegion
  // XDestroySubwindows
  pub fn XDestroyWindow (display: *mut Display, window: Window);
  // XDisableAccessControl
  // XDisplayCells
  // XDisplayHeight
  // XDisplayHeightMM
  // XDisplayKeycodes
  // XDisplayMotionBufferSize
  // XDisplayName
  // XDisplayOfScreen
  // XDisplayPlanes
  // XDisplayString
  // XDisplayWidth
  // XDisplayWidthMM
  // XDoesBackingStore
  // XDoesSaveUnders
  // XDraw
  // XDrawArc
  // XDrawArcs
  // XDrawDashed
  // XDrawFilled
  // XDrawImageString
  // XDrawImageString16
  // XDrawLine
  // XDrawLines
  // XDrawPatterned
  // XDrawPoint
  // XDrawPoints
  // XDrawRectangle
  // XDrawRectangles
  // XDrawSegments
  // XDrawString
  // XDrawString16
  // XDrawText
  // XDrawText16
  // XDrawTiled
  // XEmptyRegion
  // XEnableAccessControl
  // XEqualRegion
  // XESetBeforeFlush
  // XESetCloseDisplay
  // XESetCopyGC
  // XESetCreateFont
  // XESetCreateGC
  // XESetError
  // XESetErrorString
  // XESetEventToWire
  // XESetFlushGC
  // XESetFreeFont
  // XESetFreeGC
  // XESetPrintErrorValues
  // XESetWireToError
  // XESetWireToEvent
  // XEventMaskOfScreen
  // XEventsQueued
  // XExtendedMaxRequestSize
  // XFetchBuffer
  // XFetchBytes
  pub fn XFetchName (display: *mut Display, window: Window, name: *mut *mut c_char) -> Status;
  // XFillArc
  // XFillArcs
  // XFillPolygon
  // XFillRectangle
  // XFillRectangles
  // XFindContext
  pub fn XFlush (display: *mut Display);
  // XFlushGC
  // XForceScreenSaver
  pub fn XFree (mem: *mut c_void);
  pub fn XFreeColormap (display: *mut Display, colormap: Colormap);
  // XFreeColors
  // XFreeCursor
  // XFreeExtensionList
  // XFreeFont
  // XFreeFontInfo
  // XFreeFontNames
  // XFreeFontPath
  pub fn XFreeGC (display: *mut Display, gc: GC);
  // XFreeModifiermap
  // XFreePixmap
  // XFreeStringList
  // XGContextFromGC
  // XGeometry
  // XGetAtomName
  // XGetAtomNames
  // XGetClassHint
  // XGetCommand
  // XGetDefault
  // XGetErrorDatabaseText
  // XGetErrorText
  // XGetFontPath
  // XGetFontProperty
  // XGetGCValues
  pub fn XGetGeometry (display: *mut Display, drawable: Drawable, root: *mut Window, x: *mut c_int, y: *mut c_int,
      width: *mut c_uint, height: *mut c_uint, border_width: *mut c_uint, depth: *mut c_uint) -> Status;
  // XGetIconName
  // XGetIconSizes
  // XGetImage
  // XGetInputFocus
  // XGetKeyboardControl
  // XGetKeyboardMapping
  // XGetModifierMapping
  // XGetMotionEvents
  // XGetNormalHints
  // XGetPixel
  // XGetPointerControl
  // XGetPointerMapping
  // XGetRGBColormap
  // XGetRGBColormaps
  // XGetScreenSaver
  // XGetSelectionOwner
  // XGetSizeHints
  // XGetStandardColormap
  // XGetSubImage
  // XGetTextProperty
  // XGetTransientForHint
  pub fn XGetVisualInfo (display: *mut Display, mask: c_long, template: *const XVisualInfo, nitems: *mut c_int)
      -> *mut XVisualInfo;
  pub fn XGetWindowAttributes (display: *mut Display, window: Window, attr: *mut XWindowAttributes) -> Status;
  pub fn XGetWindowProperty (display: *mut Display, window: Window, property: Atom, long_offset: c_long,
      long_length: c_long, delete: Bool, requested_type: Atom, out_type: *mut Atom, out_format: *mut c_int,
      out_length: *mut c_ulong, out_remaining: *mut c_ulong, out_data: *mut c_char) -> c_int;
  // XGetWMClientMachine
  // XGetWMColormapWindows
  // XGetWMHints
  // XGetWMIconName
  pub fn XGetWMName (display: *mut Display, window: Window, name: *mut XTextProperty) -> Status;
  // XGetWMNormalHints
  // XGetWMProtocols
  // XGetWMSizeHints
  // XGetZoomHints
  // XGrabButton
  // XGrabKey
  // XGrabKeyboard
  // XGrabPointer
  // XGrabServer
  // XHeightMMOfScreen
  // XHeightOfScreen
  // XIconifyWindow
  // XIfEvent
  // XImageByteOrder
  // XInitExtension
  // XInitImage
  // XInitThreads
  // XInsertModifiermapEntry
  // XInstallColormap
  // XInternalConnectionNumbers
  pub fn XInternAtom (display: *mut Display, name: *const c_char, only_if_exists: Bool) -> Atom;
  // XInternAtoms
  // XIntersectRegion
  // XKeycodeToKeysym
  // XKeysymToKeycode
  // XKeysymToString
  // XKillClient
  // XLastKnownRequestProcessed
  // XListDepths
  // XListExtensions
  // XListFonts
  // XListFontsWithInfo
  // XListHosts
  // XListInstalledColormaps
  // XListPixmapFormats
  // XListProperties
  // XLoadFont
  // XLoadQueryFont
  // XLockDisplay
  // XLookUpAssoc
  // XLookupColor
  // XLookupKeysym
  // XLookupString
  // XLowerWindow
  // XMakeAssoc
  // XMapRaised
  // XMapSubwindows
  pub fn XMapWindow (display: *mut Display, window: Window);
  // XMaskEvent
  // XMatchVisualInfo
  // XMaxCmapsOfScreen
  // XMaxRequestSize
  // XmbSetWMProperties
  // XmbTextListToTextProperty
  // XmbTextPropertyToTextList
  // XMinCmapsOfScreen
  // XMoveResizeWindow
  pub fn XMoveWindow (display: *mut Display, window: Window, x: c_int, y: c_int);
  // XNewModifiermap
  pub fn XNextEvent (display: *mut Display, event: *mut XEvent);
  // XNextRequest
  // XNoOp
  // XOffsetRegion
  pub fn XOpenDisplay (name: *const c_char) -> *mut Display;
  // XParseColor
  // XParseGeometry
  // XPeekEvent
  // XPeekIfEvent
  pub fn XPending (display: *mut Display) -> c_int;
  // Xpermalloc
  // XPlanesOfScreen
  // XPointInRegion
  // XPolygonRegion
  // XProcessInternalConnection
  // XProtocolRevision
  // XProtocolVersion
  // XPutBackEvent
  // XPutImage
  // XPutPixel
  // XQLength
  // XQueryBestCursor
  // XQueryBestSize
  // XQueryBestStipple
  // XQueryBestTile
  // XQueryColor
  // XQueryColors
  // XQueryExtension
  // XQueryFont
  // XQueryKeymap
  // XQueryPointer
  // XQueryTextExtents
  // XQueryTextExtents16
  // XQueryTree
  // XRaiseWindow
  // XReadBitmapFile
  // XReadBitmapFileData
  // XRebindKeysym
  // XRecolorCursor
  // XReconfigureWMWindow
  // XRectInRegion
  // XRefreshKeyboardMapping
  // XRemoveConnectionWatch
  // XRemoveFromSaveSet
  // XRemoveHost
  // XRemoveHosts
  // XReparentWindow
  // XResetScreenSaver
  pub fn XResizeWindow (display: *mut Display, window: Window, width: c_uint, height: c_uint);
  // XResourceManagerString
  // XRestackWindows
  // XrmClassToString
  // XrmCombineDatabase
  // XrmCombineFileDatabase
  // XrmDestroyDatabase
  // XrmEnumerateDatabase
  // XrmGetDatabase
  // XrmGetFileDatabase
  // XrmGetResource
  // XrmGetStringDatabase
  // XrmInitialize
  // XrmLocaleOfDatabase
  // XrmMergeDatabases
  // XrmNameToString
  // XrmParseCommand
  // XrmPermStringToQuark
  // XrmPutFileDatabase
  // XrmPutLineResource
  // XrmPutResource
  // XrmPutStringResource
  // XrmQGetResource
  // XrmQGetSearchList
  // XrmQGetSearchResource
  // XrmQPutResource
  // XrmQPutStringResource
  // XrmQuarkToString
  // XrmRepresentationToString
  // XrmSetDataBase
  // XrmSetDatabase
  // XrmStringToBindingQuarkList
  // XrmStringToClass
  // XrmStringToClassList
  // XrmStringToName
  // XrmStringToNameList
  // XrmStringToQuark
  // XrmStringToQuarkList
  // XrmStringToRepresentation
  // XrmUniqueQuark
  pub fn XRootWindow (display: *mut Display, screen_num: c_int) -> Window;
  // XRootWindowOfScreen
  // XRotateBuffers
  // XRotateWindowProperties
  // XSaveContext
  pub fn XScreenCount (display: *mut Display) -> c_int;
  // XScreenNumberOfScreen
  // XScreenResourceString
  // XScreensOfDisplay
  // XSelectInput
  pub fn XSendEvent (display: *mut Display, window: Window, propagate: Bool, event_mask: c_long, event: *const XEvent)
      -> Status;
  // XServerVendor
  // XSetAccessControl
  // XSetAfterFunction
  // XSetArcMode
  // XSetBackground
  // XSetClassHint
  // XSetClipMask
  // XSetClipOrigin
  // XSetClipRectangles
  // XSetCloseDownMode
  // XSetCommand
  // XSetDashes
  // XSetErrorHandler
  // XSetFillRule
  // XSetFillStyle
  // XSetFont
  // XSetFontPath
  // XSetForeground
  // XSetFunction
  // XSetGraphicsExposures
  // XSetIconName
  // XSetIconSizes
  // XSetInputFocus
  // XSetIOErrorHandler
  // XSetLineAttributes
  // XSetModifierMapping
  // XSetNormalHints
  // XSetPlaneMask
  // XSetPointerMapping
  // XSetProperty
  // XSetRegion
  // XSetRGBColormaps
  // XSetScreenSaver
  // XSetSelectionOwner
  // XSetSizeHints
  // XSetStandardColormap
  // XSetStandardProperties
  // XSetState
  // XSetStipple
  // XSetSubwindowMode
  // XSetTextProperty
  // XSetTile
  // XSetTransientForHint
  // XSetTSOrigin
  // XSetWindowBackground
  // XSetWindowBackgroundPixmap
  // XSetWindowBorder
  // XSetWindowBorderPixmap
  // XSetWindowBorderWidth
  // XSetWindowColormap
  // XSetWMClientMachine
  // XSetWMColormapWindows
  // XSetWMHints
  // XSetWMIconName
  // XSetWMName
  pub fn XSetWMNormalHints (display: *mut Display, window: Window, hints: *const XSizeHints);
  // XSetWMProperties
  pub fn XSetWMProtocols (display: *mut Display, window: Window, protocols: *const Atom, count: c_int) -> Status;
  // XSetWMSizeHints
  // XSetZoomHints
  // XShrinkRegion
  // XStoreBuffer
  // XStoreBytes
  // XStoreColor
  // XStoreColors
  pub fn XStoreName (display: *mut Display, window: Window, name: *const c_char);
  // XStoreNamedColor
  // XStringListToTextProperty
  // XStringToKeysym
  // XSubImage
  // XSubtractRegion
  // XSync
  // XSynchronize
  // XTextExtents
  // XTextExtents16
  // XTextListToTextProperty
  // XTextPropertyToStringList
  // XTextWidth
  // XTextWidth16
  // XTranslateCoordinates
  // XUndefineCursor
  // XUngrabButton
  // XUngrabKey
  // XUngrabKeyboard
  // XUngrabPointer
  // XUngrabServer
  // XUninstallColormap
  // XUnionRectWithRegion
  // XUnionRegion
  // XUniqueContext
  // XUnloadFont
  // XUnlockDisplay
  // XUnmapSubwindows
  pub fn XUnmapWindow (display: *mut Display, window: Window);
  // XVendorRelease
  // XVisualIDFromVisual
  // XWarpPointer
  // XwcFreeStringList
  // XwcTextListToTextProperty
  // XwcTextPropertyToTextList
  pub fn XWhitePixel (display: *mut Display, screen_num: c_int) -> c_ulong;
  // XWhitePixelOfScreen
  // XWidthMMOfScreen
  // XWidthOfScreen
  // XWindowEvent
  // XWithdrawWindow
  // XWMGeometry
  // XWriteBitmapFile
  // XXorRegion
}


//
// types
//


// ID types
pub type Atom = XID;
pub type Colormap = XID;
pub type Cursor = XID;
pub type Drawable = XID;
pub type Font = XID;
pub type GC = XID;
pub type Pixmap = XID;
pub type VisualID = XID;
pub type Window = XID;
pub type XID = c_ulong;

// opaque structures
#[allow(missing_copy_implementations)]
#[repr(C)]
pub struct Display;

#[allow(missing_copy_implementations)]
#[repr(C)]
pub struct Screen;

#[allow(missing_copy_implementations)]
#[repr(C)]
pub struct Visual;

// boolean types
pub type Bool = c_int;
pub type Status = Bool;


//
// event structures
//


#[derive(Clone, Copy, Eq, PartialEq)]
#[repr(C)]
pub struct XEvent {
  pub pad: [c_long; 24],
}

impl XEvent {
  pub fn kind (&self) -> c_int {
    unsafe {
      *(self as *const XEvent as *const c_int)
    }
  }

  pub fn xany (&self) -> XAnyEvent {
    unsafe {
      xtransmute(self)
    }
  }

  pub fn xclient (&self) -> XClientMessageEvent {
    unsafe {
      xtransmute(self)
    }
  }

  pub fn xdestroy (&self) -> XDestroyWindowEvent {
    unsafe {
      xtransmute(self)
    }
  }
}

#[test]
fn xevent_size_test () {
  assert!(size_of::<XEvent>() >= size_of::<XAnyEvent>());
  // assert!(size_of::<XEvent>() >= size_of::<XButtonEvent>());
  // assert!(size_of::<XEvent>() >= size_of::<XCirculateEvent>());
  // assert!(size_of::<XEvent>() >= size_of::<XCirculateRequestEvent>());
  assert!(size_of::<XEvent>() >= size_of::<XClientMessageEvent>());
  // assert!(size_of::<XEvent>() >= size_of::<XColormapEvent>());
  // assert!(size_of::<XEvent>() >= size_of::<XConfigureEvent>());
  // assert!(size_of::<XEvent>() >= size_of::<XConfigureRequestEvent>());
  // assert!(size_of::<XEvent>() >= size_of::<XCreateWindowEvent>());
  // assert!(size_of::<XEvent>() >= size_of::<XCrossingEvent>());
  assert!(size_of::<XEvent>() >= size_of::<XDestroyWindowEvent>());
  // assert!(size_of::<XEvent>() >= size_of::<XErrorEvent>());
  // assert!(size_of::<XEvent>() >= size_of::<XExposeEvent>());
  // assert!(size_of::<XEvent>() >= size_of::<XFocusChangeEvent>());
  // assert!(size_of::<XEvent>() >= size_of::<XGraphicsExposeEvent>());
  // assert!(size_of::<XEvent>() >= size_of::<XGravityEvent>());
  // assert!(size_of::<XEvent>() >= size_of::<XKeyEvent>());
  // assert!(size_of::<XEvent>() >= size_of::<XKeymapEvent>());
  // assert!(size_of::<XEvent>() >= size_of::<XMapEvent>());
  // assert!(size_of::<XEvent>() >= size_of::<XMappingEvent>());
  // assert!(size_of::<XEvent>() >= size_of::<XMapRequestEvent>());
  // assert!(size_of::<XEvent>() >= size_of::<XMotionEvent>());
  // assert!(size_of::<XEvent>() >= size_of::<XNoExposeEvent>());
  // assert!(size_of::<XEvent>() >= size_of::<XPropertyEvent>());
  // assert!(size_of::<XEvent>() >= size_of::<XReparentEvent>());
  // assert!(size_of::<XEvent>() >= size_of::<XResizeRequestEvent>());
  // assert!(size_of::<XEvent>() >= size_of::<XSelectionClearEvent>());
  // assert!(size_of::<XEvent>() >= size_of::<XSelectionEvent>());
  // assert!(size_of::<XEvent>() >= size_of::<XSelectionRequestEvent>());
  // assert!(size_of::<XEvent>() >= size_of::<XUnmapEvent>());
  // assert!(size_of::<XEvent>() >= size_of::<XVisibilityEvent>());
}

#[allow(raw_pointer_derive)]
#[derive(Clone, Copy, Eq, PartialEq)]
#[repr(C)]
pub struct XAnyEvent {
  pub kind: c_int,
  pub serial: c_ulong,
  pub send_event: Bool,
  pub display: *mut Display,
  pub window: Window,
}

impl XAnyEvent {
  pub fn to_xevent (&self) -> XEvent {
    unsafe {
      xtransmute(self)
    }
  }
}

#[derive(Clone, Copy, Eq, PartialEq)]
#[repr(C)]
pub struct XClientMessageData {
  longs: [c_long; 5],
}

impl XClientMessageData {
  pub fn get_byte (&self, index: usize) -> c_char {
    unsafe {
      assert!(index < 20);
      return *(&self.longs[0] as *const c_long as *const c_char);
    }
  }

  pub fn get_long (&self, index: usize) -> c_long {
    self.longs[index]
  }

  pub fn get_short (&self, index: usize) -> c_short {
    unsafe {
      assert!(index < 10);
      return *(&self.longs[0] as *const c_long as *const c_short);
    }
  }

  pub fn set_byte (&mut self, index: usize, value: c_char) {
    unsafe {
      assert!(index < 20);
      *(&mut self.longs[0] as *mut c_long as *mut c_char) = value;
    }
  }

  pub fn set_long (&mut self, index: usize, value: c_long) {
    self.longs[index] = value;
  }

  pub fn set_short (&mut self, index: usize, value: c_short) {
    unsafe {
      assert!(index < 10);
      *(&mut self.longs[0] as *mut c_long as *mut c_short) = value;
    }
  }
}

#[test]
fn client_message_size_test () {
  assert!(size_of::<XClientMessageData>() >= size_of::<[c_char; 20]>());
  assert!(size_of::<XClientMessageData>() >= size_of::<[c_short; 10]>());
}

#[allow(raw_pointer_derive)]
#[derive(Clone, Copy, Eq, PartialEq)]
#[repr(C)]
pub struct XClientMessageEvent {
  pub kind: c_int,
  pub serial: c_ulong,
  pub send_event: Bool,
  pub display: *mut Display,
  pub window: Window,
  pub message_type: Atom,
  pub format: c_int,
  pub data: XClientMessageData,
}

impl XClientMessageEvent {
  pub fn to_xevent (&self) -> XEvent {
    unsafe {
      xtransmute(self)
    }
  }
}

#[allow(raw_pointer_derive)]
#[derive(Clone, Copy, Eq, PartialEq)]
#[repr(C)]
pub struct XDestroyWindowEvent {
  pub kind: c_int,
  pub serial: c_ulong,
  pub send_event: Bool,
  pub display: *mut Display,
  pub event: Window,
  pub window: Window,
}

impl XDestroyWindowEvent {
  pub fn to_xevent (&self) -> XEvent {
    unsafe {
      xtransmute(self)
    }
  }
}


//
// other structures
//


#[derive(Clone, Copy, Eq, PartialEq)]
#[repr(C)]
pub struct XColor {
  pub pixel: c_ulong,
  pub red: c_ushort,
  pub green: c_ushort,
  pub blue: c_ushort,
  pub flags: c_char,
  pub pad: c_char,
}

#[derive(Clone, Copy, Eq, PartialEq)]
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

#[derive(Clone, Copy, Eq, PartialEq)]
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

#[allow(raw_pointer_derive)]
#[derive(Clone, Copy, Eq, PartialEq)]
#[repr(C)]
pub struct XTextProperty {
  pub value: *mut c_uchar,
  pub encoding: Atom,
  pub format: c_int,
  pub nitems: c_ulong,
}

#[allow(raw_pointer_derive)]
#[derive(Clone, Copy, Eq, PartialEq)]
#[repr(C)]
pub struct XVisualInfo {
  pub visual: *const Visual,
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
#[derive(Clone, Copy, Eq, PartialEq)]
#[repr(C)]
pub struct XWindowAttributes {
  pub x: c_int,
  pub y: c_int,
  pub width: c_int,
  pub height: c_int,
  pub border_width: c_int,
  pub depth: c_int,
  pub visual: *const Visual,
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
  pub screen: *const Screen,
}


//
// anonymous structures
//


#[derive(Clone, Copy, Eq, PartialEq)]
#[repr(C)]
pub struct AspectRatio {
  pub x: c_int,
  pub y: c_int,
}


//
// constants
//


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

// map state
pub const IsUnmapped: c_int = 0;
pub const IsUnviewable: c_int = 1;
pub const IsViewable: c_int = 2;

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


//
// private functions
//


unsafe fn xtransmute<I, O> (input: &I) -> O
  where I : Sized, O : Sized
{
  let mut output: O = zeroed();
  let copy_len = min(size_of::<I>(), size_of::<O>());

  for i in 0..copy_len {
    *((&mut output as *mut O as usize + i) as *mut u8) = *((input as *const I as usize + i) as *const u8);
  }

  return output;
}
