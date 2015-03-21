// x11-rs: Rust bindings for X11 libraries
// The X11 libraries are available under the MIT license.
// These bindings are public domain.

use libc::{
  c_char,
  c_int,
};

use ::xlib::{
  Bool,
  Display,
  Drawable,
  XVisualInfo,
};


//
// functions
//


#[link(name="GL")]
extern "C" {
  pub fn glXChooseFBConfig (display: *mut Display, screen: c_int, attr: *const c_int, nitems: *mut c_int)
      -> *mut GLXFBConfig;
  // glXChooseVisual
  // glXCopyContext
  pub fn glXCreateContext (display: *mut Display, visual_info: *const XVisualInfo, share_context: GLXContext,
      direct: Bool) -> GLXContext;
  // glXCreateGLXPixmap
  // glXCreateNewContext
  // glXCreatePbuffer
  // glXCreatePixmap
  // glXCreateWindow
  pub fn glXDestroyContext (display: *mut Display, context: GLXContext);
  // glXDestroyGLXPixmap
  // glXDestroyPbuffer
  // glXDestroyPixmap
  // glXDestroyWindow
  // glXGet
  // glXGetClientString
  // glXGetConfig
  pub fn glXGetCurrentContext () -> GLXContext;
  pub fn glXGetCurrentDisplay () -> *mut Display;
  pub fn glXGetCurrentDrawable () -> GLXDrawable;
  // glXGetCurrentReadDrawable
  // glXGetFBConfigAttrib
  // glXGetFBConfigs
  pub fn glXGetProcAddress (name: *const c_char) -> Option<unsafe extern "C" fn ()>;
  // glXGetSelectedEvent
  pub fn glXGetVisualFromFBConfig (display: *mut Display, fbconfig: GLXFBConfig) -> *mut XVisualInfo;
  // glXIsDirect
  // glXMakeContextCurrent
  pub fn glXMakeCurrent (display: *mut Display, drawable: GLXDrawable, context: GLXContext) -> Bool;
  // glXNewCommandSGI
  // glXQuery
  // glXQueryContext
  // glXQueryDrawable
  // glXQueryExtension
  // glXQueryExtensionsString
  // glXQueryServerString
  // glXQueryVersion
  // glXSelectEvent
  pub fn glXSwapBuffers (display: *mut Display, drawable: GLXDrawable);
  // glXUseXFont
  // glXWaitGL
  // glXWaitX
}


//
// types
//


// ID types
pub type GLXDrawable = Drawable;

// opaque pointers
#[allow(missing_copy_implementations)]
#[repr(C)]
pub struct GLXContext_Rec;
pub type GLXContext = *mut GLXContext_Rec;

#[allow(missing_copy_implementations)]
#[repr(C)]
pub struct GLXFBConfig_Rec;
pub type GLXFBConfig = *mut GLXFBConfig_Rec;


//
// constants
//


// config caveats
pub const GLX_SLOW_CONFIG: c_int = 0x8001;
pub const GLX_NON_CONFORMANT_CONFIG: c_int = 0x800d;

// drawable type mask
pub const GLX_WINDOW_BIT: c_int = 0x0001;
pub const GLX_PIXMAP_BIT: c_int = 0x0002;
pub const GLX_PBUFFER_BIT: c_int = 0x0004;

// framebuffer attributes
pub const GLX_BUFFER_SIZE: c_int = 0x0002;
pub const GLX_LEVEL: c_int = 0x0003;
pub const GLX_DOUBLEBUFFER: c_int = 0x0005;
pub const GLX_STEREO: c_int = 0x0006;
pub const GLX_AUX_BUFFERS: c_int = 0x0007;
pub const GLX_RED_SIZE: c_int = 0x0008;
pub const GLX_GREEN_SIZE: c_int = 0x0009;
pub const GLX_BLUE_SIZE: c_int = 0x000a;
pub const GLX_ALPHA_SIZE: c_int = 0x000b;
pub const GLX_DEPTH_SIZE: c_int = 0x000c;
pub const GLX_STENCIL_SIZE: c_int = 0x000d;
pub const GLX_ACCUM_RED_SIZE: c_int = 0x000e;
pub const GLX_ACCUM_GREEN_SIZE: c_int = 0x000f;
pub const GLX_ACCUM_BLUE_SIZE: c_int = 0x0010;
pub const GLX_ACCUM_ALPHA_SIZE: c_int = 0x0011;
pub const GLX_CONFIG_CAVEAT: c_int = 0x0020;
pub const GLX_X_VISUAL_TYPE: c_int = 0x0022;
pub const GLX_TRANSPARENT_TYPE: c_int = 0x0023;
pub const GLX_TRANSPARENT_INDEX_VALUE: c_int = 0x0024;
pub const GLX_TRANSPARENT_RED_VALUE: c_int = 0x0025;
pub const GLX_TRANSPARENT_GREEN_VALUE: c_int = 0x0026;
pub const GLX_TRANSPARENT_BLUE_VALUE: c_int = 0x0027;
pub const GLX_TRANSPARENT_ALPHA_VALUE: c_int = 0x0028;
pub const GLX_DRAWABLE_TYPE: c_int = 0x8010;
pub const GLX_RENDER_TYPE: c_int = 0x8011;
pub const GLX_X_RENDERABLE: c_int = 0x8012;
pub const GLX_FBCONFIG_ID: c_int = 0x8013;

// misc
pub const GLX_DONT_CARE: c_int = -1;
pub const GLX_NONE: c_int = 0x8000;

// render type mask
pub const GLX_RGBA_BIT: c_int = 0x0001;
pub const GLX_COLOR_INDEX_BIT: c_int = 0x0002;

// transparent types
pub const GLX_TRANSPARENT_RGB: c_int = 0x8008;
pub const GLX_TRANSPARENT_INDEX: c_int = 0x8009;

// visual types
pub const GLX_TRUE_COLOR: c_int = 0x8002;
pub const GLX_DIRECT_COLOR: c_int = 0x8003;
pub const GLX_PSEUDO_COLOR: c_int = 0x8004;
pub const GLX_STATIC_COLOR: c_int = 0x8005;
pub const GLX_GRAY_SCALE: c_int = 0x8006;
pub const GLX_STATIC_GRAY: c_int = 0x8007;


//
// ARB extensions
//


pub mod arb {
  use libc::c_int;

  // context attributes
  pub const GLX_CONTEXT_MAJOR_VERSION_ARB: c_int = 0x2091;
  pub const GLX_CONTEXT_MINOR_VERSION_ARB: c_int = 0x2092;
  pub const GLX_CONTEXT_FLAGS_ARB: c_int = 0x2094;
  pub const GLX_CONTEXT_PROFILE_MASK_ARB: c_int = 0x9126;

  // context flags
  pub const GLX_CONTEXT_DEBUG_BIT_ARB: c_int = 0x0001;
  pub const GLX_CONTEXT_FORWARD_COMPATIBLE_BIT_ARB: c_int = 0x0002;

  // context profile mask
  pub const GLX_CONTEXT_CORE_PROFILE_BIT_ARB: c_int = 0x0001;
  pub const GLX_CONTEXT_COMPATIBILITY_PROFILE_BIT_ARB: c_int = 0x0002;
}
