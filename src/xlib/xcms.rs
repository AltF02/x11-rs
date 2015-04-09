// x11-rs: Rust bindings for X11 libraries
// The X11 libraries are available under the MIT license.
// These bindings are public domain.

use libc::{
  c_char,
  c_double,
  c_int,
  c_uint,
  c_ulong,
};

use ::xlib::{
  Display,
  Visual,
};


//
// functions
//


#[link(name="X11")]
extern "C" {
  pub fn XcmsAddColorSpace (_1: *mut XcmsColorSpace) -> c_int;
  pub fn XcmsAddFunctionSet (_1: *mut XcmsFunctionSet) -> c_int;
  pub fn XcmsAllocColor (_4: *mut Display, _3: c_ulong, _2: *mut XcmsColor, _1: c_ulong) -> c_int;
  pub fn XcmsAllocNamedColor (_6: *mut Display, _5: c_ulong, _4: *const c_char, _3: *mut XcmsColor, _2: *mut XcmsColor, _1: c_ulong) -> c_int;
  pub fn XcmsCCCOfColormap (_2: *mut Display, _1: c_ulong) -> XcmsCCC;
  pub fn XcmsCIELabClipab (_5: XcmsCCC, _4: *mut XcmsColor, _3: c_uint, _2: c_uint, _1: *mut c_int) -> c_int;
  pub fn XcmsCIELabClipL (_5: XcmsCCC, _4: *mut XcmsColor, _3: c_uint, _2: c_uint, _1: *mut c_int) -> c_int;
  pub fn XcmsCIELabClipLab (_5: XcmsCCC, _4: *mut XcmsColor, _3: c_uint, _2: c_uint, _1: *mut c_int) -> c_int;
  pub fn XcmsCIELabQueryMaxC (_4: XcmsCCC, _3: c_double, _2: c_double, _1: *mut XcmsColor) -> c_int;
  pub fn XcmsCIELabQueryMaxL (_4: XcmsCCC, _3: c_double, _2: c_double, _1: *mut XcmsColor) -> c_int;
  pub fn XcmsCIELabQueryMaxLC (_3: XcmsCCC, _2: c_double, _1: *mut XcmsColor) -> c_int;
  pub fn XcmsCIELabQueryMinL (_4: XcmsCCC, _3: c_double, _2: c_double, _1: *mut XcmsColor) -> c_int;
  pub fn XcmsCIELabToCIEXYZ (_4: XcmsCCC, _3: *mut XcmsColor, _2: *mut XcmsColor, _1: c_uint) -> c_int;
  pub fn XcmsCIELabWhiteShiftColors (_7: XcmsCCC, _6: *mut XcmsColor, _5: *mut XcmsColor, _4: c_ulong, _3: *mut XcmsColor, _2: c_uint, _1: *mut c_int) -> c_int;
  pub fn XcmsCIELuvClipL (_5: XcmsCCC, _4: *mut XcmsColor, _3: c_uint, _2: c_uint, _1: *mut c_int) -> c_int;
  pub fn XcmsCIELuvClipLuv (_5: XcmsCCC, _4: *mut XcmsColor, _3: c_uint, _2: c_uint, _1: *mut c_int) -> c_int;
  pub fn XcmsCIELuvClipuv (_5: XcmsCCC, _4: *mut XcmsColor, _3: c_uint, _2: c_uint, _1: *mut c_int) -> c_int;
  pub fn XcmsCIELuvQueryMaxC (_4: XcmsCCC, _3: c_double, _2: c_double, _1: *mut XcmsColor) -> c_int;
  pub fn XcmsCIELuvQueryMaxL (_4: XcmsCCC, _3: c_double, _2: c_double, _1: *mut XcmsColor) -> c_int;
  pub fn XcmsCIELuvQueryMaxLC (_3: XcmsCCC, _2: c_double, _1: *mut XcmsColor) -> c_int;
  pub fn XcmsCIELuvQueryMinL (_4: XcmsCCC, _3: c_double, _2: c_double, _1: *mut XcmsColor) -> c_int;
  pub fn XcmsCIELuvToCIEuvY (_4: XcmsCCC, _3: *mut XcmsColor, _2: *mut XcmsColor, _1: c_uint) -> c_int;
  pub fn XcmsCIELuvWhiteShiftColors (_7: XcmsCCC, _6: *mut XcmsColor, _5: *mut XcmsColor, _4: c_ulong, _3: *mut XcmsColor, _2: c_uint, _1: *mut c_int) -> c_int;
  pub fn XcmsCIEuvYToCIELuv (_4: XcmsCCC, _3: *mut XcmsColor, _2: *mut XcmsColor, _1: c_uint) -> c_int;
  pub fn XcmsCIEuvYToCIEXYZ (_4: XcmsCCC, _3: *mut XcmsColor, _2: *mut XcmsColor, _1: c_uint) -> c_int;
  pub fn XcmsCIEuvYToTekHVC (_4: XcmsCCC, _3: *mut XcmsColor, _2: *mut XcmsColor, _1: c_uint) -> c_int;
  pub fn XcmsCIExyYToCIEXYZ (_4: XcmsCCC, _3: *mut XcmsColor, _2: *mut XcmsColor, _1: c_uint) -> c_int;
  pub fn XcmsCIEXYZToCIELab (_4: XcmsCCC, _3: *mut XcmsColor, _2: *mut XcmsColor, _1: c_uint) -> c_int;
  pub fn XcmsCIEXYZToCIEuvY (_4: XcmsCCC, _3: *mut XcmsColor, _2: *mut XcmsColor, _1: c_uint) -> c_int;
  pub fn XcmsCIEXYZToCIExyY (_4: XcmsCCC, _3: *mut XcmsColor, _2: *mut XcmsColor, _1: c_uint) -> c_int;
  pub fn XcmsCIEXYZToRGBi (_4: XcmsCCC, _3: *mut XcmsColor, _2: c_uint, _1: *mut c_int) -> c_int;
  pub fn XcmsClientWhitePointOfCCC (_1: XcmsCCC) -> *mut XcmsColor;
  pub fn XcmsConvertColors (_5: XcmsCCC, _4: *mut XcmsColor, _3: c_uint, _2: c_ulong, _1: *mut c_int) -> c_int;
  pub fn XcmsCreateCCC (_8: *mut Display, _7: c_int, _6: *mut Visual, _5: *mut XcmsColor, _4: Option<unsafe extern "C" fn (XcmsCCC, *mut XcmsColor, c_uint, c_uint, *mut c_int) -> c_int>, _3: *mut c_char, _2: Option<unsafe extern "C" fn (XcmsCCC, *mut XcmsColor, *mut XcmsColor, c_ulong, *mut XcmsColor, c_uint, *mut c_int) -> c_int>, _1: *mut c_char) -> XcmsCCC;
  pub fn XcmsDefaultCCC (_2: *mut Display, _1: c_int) -> XcmsCCC;
  pub fn XcmsDisplayOfCCC (_1: XcmsCCC) -> *mut Display;
  pub fn XcmsFormatOfPrefix (_1: *mut c_char) -> c_ulong;
  pub fn XcmsFreeCCC (_1: XcmsCCC) -> ();
  pub fn XcmsLookupColor (_6: *mut Display, _5: c_ulong, _4: *const c_char, _3: *mut XcmsColor, _2: *mut XcmsColor, _1: c_ulong) -> c_int;
  pub fn XcmsPrefixOfFormat (_1: c_ulong) -> *mut c_char;
  pub fn XcmsQueryBlack (_3: XcmsCCC, _2: c_ulong, _1: *mut XcmsColor) -> c_int;
  pub fn XcmsQueryBlue (_3: XcmsCCC, _2: c_ulong, _1: *mut XcmsColor) -> c_int;
  pub fn XcmsQueryColor (_4: *mut Display, _3: c_ulong, _2: *mut XcmsColor, _1: c_ulong) -> c_int;
  pub fn XcmsQueryColors (_5: *mut Display, _4: c_ulong, _3: *mut XcmsColor, _2: c_uint, _1: c_ulong) -> c_int;
  pub fn XcmsQueryGreen (_3: XcmsCCC, _2: c_ulong, _1: *mut XcmsColor) -> c_int;
  pub fn XcmsQueryRed (_3: XcmsCCC, _2: c_ulong, _1: *mut XcmsColor) -> c_int;
  pub fn XcmsQueryWhite (_3: XcmsCCC, _2: c_ulong, _1: *mut XcmsColor) -> c_int;
  pub fn XcmsRGBiToCIEXYZ (_4: XcmsCCC, _3: *mut XcmsColor, _2: c_uint, _1: *mut c_int) -> c_int;
  pub fn XcmsRGBiToRGB (_4: XcmsCCC, _3: *mut XcmsColor, _2: c_uint, _1: *mut c_int) -> c_int;
  pub fn XcmsRGBToRGBi (_4: XcmsCCC, _3: *mut XcmsColor, _2: c_uint, _1: *mut c_int) -> c_int;
  pub fn XcmsScreenNumberOfCCC (_1: XcmsCCC) -> c_int;
  pub fn XcmsScreenWhitePointOfCCC (_1: XcmsCCC) -> *mut XcmsColor;
  pub fn XcmsSetCCCOfColormap (_3: *mut Display, _2: c_ulong, _1: XcmsCCC) -> XcmsCCC;
  pub fn XcmsSetCompressionProc (_3: XcmsCCC, _2: Option<unsafe extern "C" fn (XcmsCCC, *mut XcmsColor, c_uint, c_uint, *mut c_int) -> c_int>, _1: *mut c_char) -> Option<unsafe extern "C" fn (XcmsCCC, *mut XcmsColor, c_uint, c_uint, *mut c_int) -> c_int>;
  pub fn XcmsSetWhiteAdjustProc (_3: XcmsCCC, _2: Option<unsafe extern "C" fn (XcmsCCC, *mut XcmsColor, *mut XcmsColor, c_ulong, *mut XcmsColor, c_uint, *mut c_int) -> c_int>, _1: *mut c_char) -> Option<unsafe extern "C" fn (XcmsCCC, *mut XcmsColor, *mut XcmsColor, c_ulong, *mut XcmsColor, c_uint, *mut c_int) -> c_int>;
  pub fn XcmsSetWhitePoint (_2: XcmsCCC, _1: *mut XcmsColor) -> c_int;
  pub fn XcmsStoreColor (_3: *mut Display, _2: c_ulong, _1: *mut XcmsColor) -> c_int;
  pub fn XcmsStoreColors (_5: *mut Display, _4: c_ulong, _3: *mut XcmsColor, _2: c_uint, _1: *mut c_int) -> c_int;
  pub fn XcmsTekHVCClipC (_5: XcmsCCC, _4: *mut XcmsColor, _3: c_uint, _2: c_uint, _1: *mut c_int) -> c_int;
  pub fn XcmsTekHVCClipV (_5: XcmsCCC, _4: *mut XcmsColor, _3: c_uint, _2: c_uint, _1: *mut c_int) -> c_int;
  pub fn XcmsTekHVCClipVC (_5: XcmsCCC, _4: *mut XcmsColor, _3: c_uint, _2: c_uint, _1: *mut c_int) -> c_int;
  pub fn XcmsTekHVCQueryMaxC (_4: XcmsCCC, _3: c_double, _2: c_double, _1: *mut XcmsColor) -> c_int;
  pub fn XcmsTekHVCQueryMaxV (_4: XcmsCCC, _3: c_double, _2: c_double, _1: *mut XcmsColor) -> c_int;
  pub fn XcmsTekHVCQueryMaxVC (_3: XcmsCCC, _2: c_double, _1: *mut XcmsColor) -> c_int;
  pub fn XcmsTekHVCQueryMaxVSamples (_4: XcmsCCC, _3: c_double, _2: *mut XcmsColor, _1: c_uint) -> c_int;
  pub fn XcmsTekHVCQueryMinV (_4: XcmsCCC, _3: c_double, _2: c_double, _1: *mut XcmsColor) -> c_int;
  pub fn XcmsTekHVCToCIEuvY (_4: XcmsCCC, _3: *mut XcmsColor, _2: *mut XcmsColor, _1: c_uint) -> c_int;
  pub fn XcmsTekHVCWhiteShiftColors (_7: XcmsCCC, _6: *mut XcmsColor, _5: *mut XcmsColor, _4: c_ulong, _3: *mut XcmsColor, _2: c_uint, _1: *mut c_int) -> c_int;
  pub fn XcmsVisualOfCCC (_1: XcmsCCC) -> *mut Visual;
}


//
// types
//


// TODO structs
#[repr(C)] pub struct _XcmsCCC;
#[repr(C)] pub struct XcmsColor;
#[repr(C)] pub struct _XcmsColorSpace;
#[repr(C)] pub struct _XcmsFunctionSet;

// typedefs
pub type XcmsCCC = *mut _XcmsCCC;
pub type XcmsColorSpace = _XcmsColorSpace;
pub type XcmsFunctionSet = _XcmsFunctionSet;
