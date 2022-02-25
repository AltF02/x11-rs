# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed
- Updated pkg-config to 0.3.24
- Correct typos in xshm module
- Make members of xshm-structs public
- Update crate to rust 2021

## [2.19.1] - 2021-09-25
### Added
- This changelog
- _XkbControls struct
- _XkbMods struct
- XkbStateRec type
- PictStandardARGB32 const
- PictStandardRGB24 const
- PictStandardA8 const
- PictStandardA4 const
- PictStandardA1 const
- XShmQueryExtension function
- XShmGetEventBase function
- XshmQueryVersion function
- XShmPixmapFormat function
- XShmAttach function
- XShmDetach function
- XShmPutImage function
- XShmGetImage function
- XShmCreateImage function
- XShmCreatePixMap function
- ShmSeg type
- XShmCompletionEvent struct
- XshmSegmentInfo struct
- All feature

### Fixed
- Field order in [_XkbStateRec](https://github.com/freedesktop/xorg-xserver/blob/master/include/xkbstr.h#L47) struct

[Unreleased]: ../../compare/v2.19.1...HEAD
[2.19.1]: ../../compare/v2.19.0...v2.19.1
