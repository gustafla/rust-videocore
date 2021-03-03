// +-----------------------------------------------------------------------------------------------+
// | Copyright 2015 Sean Kerr                                                                      |
// |                                                                                               |
// | Licensed under the Apache License, Version 2.0 (the "License");                               |
// | you may not use this file except in compliance with the License.                              |
// | You may obtain a copy of the License Author                                                   |
// |                                                                                               |
// |  http://www.apache.org/licenses/LICENSE-2.0                                                   |
// |                                                                                               |
// | Unless required by applicable law or agreed to in writing, software                           |
// | distributed under the License is distributed on an "AS IS" BASIS,                             |
// | WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.                      |
// | See the License for the specific language governing permissions and                           |
// | limitations under the License.                                                                |
// +-----------------------------------------------------------------------------------------------+
// | Author: Sean Kerr <sean@metatomic.io>, Lauri Gustafsson <me@gustafla.space>                   |
// +-----------------------------------------------------------------------------------------------+

// -------------------------------------------------------------------------------------------------
// DEPENDENCIES
// -------------------------------------------------------------------------------------------------

use std::ffi::c_void;
use std::os::raw::c_int;

// -------------------------------------------------------------------------------------------------
// TYPES
// -------------------------------------------------------------------------------------------------

pub type DisplayHandle = u32;
pub type ElementHandle = u32;
pub type ResourceHandle = u32;
pub type UpdateHandle = u32;
pub type Protection = u32;

// -------------------------------------------------------------------------------------------------
// ENUMS
// -------------------------------------------------------------------------------------------------

#[repr(C)]
pub enum FlagsAlpha {
    // bottom 2 bits sets the alpha mode
    FromSource = 0,
    FixedAllPixels = 1,
    FixedNonZero = 2,
    FixedExceed0x07 = 3,

    Premult = 1 << 16,
    Mix = 1 << 17,
}

#[repr(C)]
pub enum Status {
    Success = 0,
    Invalid = -1,
}

#[repr(C)]
pub enum Transform {
    NoRotate = 0,
    Rotate90 = 1,
    Rotate180 = 2,
    Rotate270 = 3,

    FlipHriz = 1 << 16,
    FlipVert = 1 << 17,

    // extra flags for controlling snapshot behaviour
    SnapshotNoYuv = 1 << 24,
    SnapshotNoRgb = 1 << 25,
    SnapshotFill = 1 << 26,
    SnapshotSwapRedBlue = 1 << 27,
    SnapshotPack = 1 << 28,
}

// -------------------------------------------------------------------------------------------------
// STRUCTS
// -------------------------------------------------------------------------------------------------

#[repr(C)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

#[repr(C)]
pub struct VCAlpha {
    pub flags: FlagsAlpha,
    pub opacity: u32,
    pub mask: ResourceHandle,
}

#[repr(C)]
pub struct Window {
    pub element: ElementHandle,
    pub width: c_int,
    pub height: c_int,
}

// -------------------------------------------------------------------------------------------------
// CONSTANTS
// -------------------------------------------------------------------------------------------------

const DISPMANX_PROTECTION_NONE: u32 = 0;

pub const DISPMANX_ID_MAIN_LCD: u32 = 0;
pub const DISPMANX_ID_AUX_LCD: u32 = 1;
pub const DISPMANX_ID_HDMI: u32 = 2;
pub const DISPMANX_ID_SDTV: u32 = 3;
pub const DISPMANX_ID_FORCE_LCD: u32 = 4;
pub const DISPMANX_ID_FORCE_TV: u32 = 5;
pub const DISPMANX_ID_FORCE_OTHER: u32 = 6; // non-default display

// -------------------------------------------------------------------------------------------------
// FUNCTIONS
// -------------------------------------------------------------------------------------------------

pub fn create_window(
    device: u32,
    dest_rect: &mut Rect,
    src_rect: &mut Rect,
    alpha: &mut VCAlpha,
    transform: Transform,
) -> Window {
    unsafe {
        let display = ffi::vc_dispmanx_display_open(device);
        let update = ffi::vc_dispmanx_update_start(0);
        let element = ffi::vc_dispmanx_element_add(
            update,
            display,
            0,
            dest_rect,
            0,
            src_rect,
            DISPMANX_PROTECTION_NONE,
            alpha,
            std::ptr::null_mut(),
            transform,
        );
        ffi::vc_dispmanx_update_submit_sync(update);
        Window {
            element,
            width: dest_rect.width,
            height: dest_rect.height,
        }
    }
}

// -------------------------------------------------------------------------------------------------
// FFI
// -------------------------------------------------------------------------------------------------
mod ffi {
    use super::*;

    #[repr(C)]
    pub enum _FlagsClamp {
        _None = 0,
        _LumaTransparent = 1,
        _Transparent = 2,
        _Replace = 3,
    }

    #[repr(C)]
    pub enum _FlagsKeymask {
        _Override = 1,
        _Smooth = 1 << 1,
        _CrInv = 1 << 2,
        _CbInv = 1 << 3,
        _YyInv = 1 << 4,
    }

    #[repr(C)]
    pub struct _Clamp {
        pub mode: _FlagsClamp,
        pub key_mask: _FlagsKeymask,
        pub key_value: *mut c_void,
        pub replace_value: u32,
    }

    extern "C" {
        pub fn vc_dispmanx_display_open(device: u32) -> DisplayHandle;

        pub fn vc_dispmanx_element_add(
            update: UpdateHandle,
            display: DisplayHandle,
            layer: i32,
            dest_rect: *mut Rect,
            src: ResourceHandle,
            src_rect: *mut Rect,
            protection: Protection,
            alpha: *mut VCAlpha,
            clamp: *mut _Clamp,
            transform: Transform,
        ) -> ElementHandle;

        pub fn vc_dispmanx_update_start(priority: i32) -> UpdateHandle;

        pub fn vc_dispmanx_update_submit_sync(update: UpdateHandle) -> i32;
    }
}
