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
// | Author: Sean Kerr <sean@metatomic.io>                                                         |
// +-----------------------------------------------------------------------------------------------+

#![allow(dead_code)]

// -------------------------------------------------------------------------------------------------
// DEPENDENCIES
// -------------------------------------------------------------------------------------------------

// -------------------------------------------------------------------------------------------------
// TYPES
// -------------------------------------------------------------------------------------------------

pub type InputFormat = VCOSInputFormat;

// -------------------------------------------------------------------------------------------------
// ENUMS
// -------------------------------------------------------------------------------------------------

#[repr(C)]
pub enum _3dFormat {
    Unsupported = 0, // default
    Interleaved,     // for autosteroscopic displays
    SbsFullAuto,     // side-by-side, full width (also used by some autostereoscopic displays)
    SbsHalfHoriz,    // side-by-side, half width, horizontal subsampling (see HDMI spec)
    TbHalf,          // top-bottom 3D
    Max,
}

#[repr(C)]
pub enum Dither {
    None = 0, // default if not set
    Rgb666 = 1,
    Rgb565 = 2,
    Rgb555 = 3,
    Max,
}

#[repr(C)]
pub enum Interface {
    Min,
    Smi,
    Dpi,
    Dsi,
    Lvds,
    Max,
}

#[repr(C)]
pub enum VCOSInputFormat {
    Invalid = 0,
    Rgb888,
    Rgb565,
}

// -------------------------------------------------------------------------------------------------
// STRUCTS
// -------------------------------------------------------------------------------------------------

pub struct Info {
    pub type_: Interface,
    pub width: u32,
    pub height: u32,
    pub input_format: InputFormat,
    pub interlaced: u32,
    pub output_dither: Dither,
    pub pixel_freq: u32,
    pub line_rate: u32,
    pub format_3d: _3dFormat,
    pub use_pixelvalve_1: u32,
    pub dsi_video_mode: u32,
    pub hvs_channel: u32,
}
