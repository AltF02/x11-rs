// x11-rs: Rust bindings for X11 libraries
// The X11 libraries are available under the MIT license.
// These bindings are public domain.

use libc::{
  c_int,
  c_uint,
  c_ulong,
  c_uchar,
  c_ushort,
};

// Reserved resource and constant definitions

pub const None: c_int = 0;
pub const ParentRelative: c_int = 1;
pub const CopyFromParent: c_int = 0;
pub const PointerWindow: c_int = 0;
pub const InputFocus: c_int = 1;
pub const PointerRoot: c_int = 1;
pub const AnyPropertyType: c_int = 0;
pub const AnyKey: c_int = 0;
pub const AnyButton: c_int = 0;
pub const AllTemporary: c_int = 0;
pub const CurrentTime: c_int = 0;
pub const NoSymbol: c_int = 0;

// Reply codes

pub const X_Reply: c_uchar = 1;
pub const X_Error: c_uchar = 0;

// Request codes

pub const X_CreateWindow: c_uchar = 1;
pub const X_ChangeWindowAttributes: c_uchar = 2;
pub const X_GetWindowAttributes: c_uchar = 3;
pub const X_DestroyWindow: c_uchar = 4;
pub const X_DestroySubwindows: c_uchar = 5;
pub const X_ChangeSaveSet: c_uchar = 6;
pub const X_ReparentWindow: c_uchar = 7;
pub const X_MapWindow: c_uchar = 8;
pub const X_MapSubwindows: c_uchar = 9;
pub const X_UnmapWindow: c_uchar = 10;
pub const X_UnmapSubwindows: c_uchar = 11;
pub const X_ConfigureWindow: c_uchar = 12;
pub const X_CirculateWindow: c_uchar = 13;
pub const X_GetGeometry: c_uchar = 14;
pub const X_QueryTree: c_uchar = 15;
pub const X_InternAtom: c_uchar = 16;
pub const X_GetAtomName: c_uchar = 17;
pub const X_ChangeProperty: c_uchar = 18;
pub const X_DeleteProperty: c_uchar = 19;
pub const X_GetProperty: c_uchar = 20;
pub const X_ListProperties: c_uchar = 21;
pub const X_SetSelectionOwner: c_uchar = 22;
pub const X_GetSelectionOwner: c_uchar = 23;
pub const X_ConvertSelection: c_uchar = 24;
pub const X_SendEvent: c_uchar = 25;
pub const X_GrabPointer: c_uchar = 26;
pub const X_UngrabPointer: c_uchar = 27;
pub const X_GrabButton: c_uchar = 28;
pub const X_UngrabButton: c_uchar = 29;
pub const X_ChangeActivePointerGrab: c_uchar = 30;
pub const X_GrabKeyboard: c_uchar = 31;
pub const X_UngrabKeyboard: c_uchar = 32;
pub const X_GrabKey: c_uchar = 33;
pub const X_UngrabKey: c_uchar = 34;
pub const X_AllowEvents: c_uchar = 35;
pub const X_GrabServer: c_uchar = 36;
pub const X_UngrabServer: c_uchar = 37;
pub const X_QueryPointer: c_uchar = 38;
pub const X_GetMotionEvents: c_uchar = 39;
pub const X_TranslateCoords: c_uchar = 40;
pub const X_WarpPointer: c_uchar = 41;
pub const X_SetInputFocus: c_uchar = 42;
pub const X_GetInputFocus: c_uchar = 43;
pub const X_QueryKeymap: c_uchar = 44;
pub const X_OpenFont: c_uchar = 45;
pub const X_CloseFont: c_uchar = 46;
pub const X_QueryFont: c_uchar = 47;
pub const X_QueryTextExtents: c_uchar = 48;
pub const X_ListFonts: c_uchar = 49;
pub const X_ListFontsWithInfo: c_uchar = 50;
pub const X_SetFontPath: c_uchar = 51;
pub const X_GetFontPath: c_uchar = 52;
pub const X_CreatePixmap: c_uchar = 53;
pub const X_FreePixmap: c_uchar = 54;
pub const X_CreateGC: c_uchar = 55;
pub const X_ChangeGC: c_uchar = 56;
pub const X_CopyGC: c_uchar = 57;
pub const X_SetDashes: c_uchar = 58;
pub const X_SetClipRectangles: c_uchar = 59;
pub const X_FreeGC: c_uchar = 60;
pub const X_ClearArea: c_uchar = 61;
pub const X_CopyArea: c_uchar = 62;
pub const X_CopyPlane: c_uchar = 63;
pub const X_PolyPoint: c_uchar = 64;
pub const X_PolyLine: c_uchar = 65;
pub const X_PolySegment: c_uchar = 66;
pub const X_PolyRectangle: c_uchar = 67;
pub const X_PolyArc: c_uchar = 68;
pub const X_FillPoly: c_uchar = 69;
pub const X_PolyFillRectangle: c_uchar = 70;
pub const X_PolyFillArc: c_uchar = 71;
pub const X_PutImage: c_uchar = 72;
pub const X_GetImage: c_uchar = 73;
pub const X_PolyText8: c_uchar = 74;
pub const X_PolyText16: c_uchar = 75;
pub const X_ImageText8: c_uchar = 76;
pub const X_ImageText16: c_uchar = 77;
pub const X_CreateColormap: c_uchar = 78;
pub const X_FreeColormap: c_uchar = 79;
pub const X_CopyColormapAndFree: c_uchar = 80;
pub const X_InstallColormap: c_uchar = 81;
pub const X_UninstallColormap: c_uchar = 82;
pub const X_ListInstalledColormaps: c_uchar = 83;
pub const X_AllocColor: c_uchar = 84;
pub const X_AllocNamedColor: c_uchar = 85;
pub const X_AllocColorCells: c_uchar = 86;
pub const X_AllocColorPlanes: c_uchar = 87;
pub const X_FreeColors: c_uchar = 88;
pub const X_StoreColors: c_uchar = 89;
pub const X_StoreNamedColor: c_uchar = 90;
pub const X_QueryColors: c_uchar = 91;
pub const X_LookupColor: c_uchar = 92;
pub const X_CreateCursor: c_uchar = 93;
pub const X_CreateGlyphCursor: c_uchar = 94;
pub const X_FreeCursor: c_uchar = 95;
pub const X_RecolorCursor: c_uchar = 96;
pub const X_QueryBestSize: c_uchar = 97;
pub const X_QueryExtension: c_uchar = 98;
pub const X_ListExtensions: c_uchar = 99;
pub const X_ChangeKeyboardMapping: c_uchar = 100;
pub const X_GetKeyboardMapping: c_uchar = 101;
pub const X_ChangeKeyboardControl: c_uchar = 102;
pub const X_GetKeyboardControl: c_uchar = 103;
pub const X_Bell: c_uchar = 104;
pub const X_ChangePointerControl: c_uchar = 105;
pub const X_GetPointerControl: c_uchar = 106;
pub const X_SetScreenSaver: c_uchar = 107;
pub const X_GetScreenSaver: c_uchar = 108;
pub const X_ChangeHosts: c_uchar = 109;
pub const X_ListHosts: c_uchar = 110;
pub const X_SetAccessControl: c_uchar = 111;
pub const X_SetCloseDownMode: c_uchar = 112;
pub const X_KillClient: c_uchar = 113;
pub const X_RotateProperties: c_uchar = 114;
pub const X_ForceScreenSaver: c_uchar = 115;
pub const X_SetPointerMapping: c_uchar = 116;
pub const X_GetPointerMapping: c_uchar = 117;
pub const X_SetModifierMapping: c_uchar = 118;
pub const X_GetModifierMapping: c_uchar = 119;
pub const X_NoOperation: c_uchar = 127;


/* Definitions for the X window system likely to be used by applications */

pub const X_PROTOCOL: c_int = 11;
pub const X_PROTOCOL_REVISION: c_int = 0;
