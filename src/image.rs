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
// ENUMS
// -------------------------------------------------------------------------------------------------

#[repr(C)]
pub enum ImageBayerFormat {
    //defined to be identical to register bits
    Raw6 = 0,
    Raw7 = 1,
    Raw8 = 2,
    Raw10 = 3,
    Raw12 = 4,
    Raw14 = 5,
    Raw16 = 6,
    Raw10_8 = 7,
    Raw12_8 = 8,
    Raw14_8 = 9,
    Raw10L = 11,
    Raw12L = 12,
    Raw14L = 13,
    Raw16BigEndian = 14,
    Raw4 = 15,
}

#[repr(C)]
pub enum ImageBayerOrder {
    //defined to be identical to register bits
    RGGB = 0,
    GBRG = 1,
    BGGR = 2,
    GRBG = 3,
}

#[repr(C)]
pub enum ImageTransform {
    Rot0 = 0,
    MirrorRot0 = (1 << 0),
    MirrorRot180 = (1 << 1),
    Rot180 = (1 << 0) | (1 << 1),
    MirrorRot90 = (1 << 2),
    Rot270 = (1 << 2) | (1 << 0),
    Rot90 = (1 << 2) | (1 << 1),
    MirrorRot270 = (1 << 2) | (1 << 0) | (1 << 1),
}

#[allow()]
#[repr(C)]
pub enum ImageType {
    Min = 0, //bounds for error checking
    Rgb565 = 1,
    _1Bpp,
    Yuv420,
    _48Bpp,
    Rgb888,
    _8Bpp,
    _4Bpp,        // 4bpp palettised image
    _3D32,        // A separated format of 16 colour/light shorts followed by 16 z values
    _3D32B,       // 16 colours followed by 16 z values
    _3D32Mat,     // A separated format of 16 material/colour/light shorts followed by 16 z values
    Rgb2x9,       // 32 bit format containing 18 bits of 6.6.6 RGB, 9 bits per short
    Rgb666,       // 32-bit format holding 18 bits of 6.6.6 RGB
    Pal4Obsolete, // 4bpp palettised image with embedded palette
    Pal8Obsolete, // 8bpp palettised image with embedded palette
    Rgba32,       // RGB888 with an alpha byte after each pixel  // xxx: isn't it BEFORE each pixel?
    Yuv422, // a line of Y (32-byte padded), a line of U (16-byte padded), and a line of V (16-byte padded)
    Rgba565, // RGB565 with a transparent patch
    Rgba16, // Compressed (4444) version of RGBA32
    YuvUv,  // VCIII codec format
    TfRgba32, // VCIII T-format RGBA8888
    TfRgbx32, // VCIII T-format RGBx8888
    TfFloat, // VCIII T-format float
    TfRgba16, // VCIII T-format RGBA4444
    TfRgba5551, // VCIII T-format RGB5551
    TfRgb565, // VCIII T-format RGB565
    TfYa88, // VCIII T-format 8-bit luma and 8-bit alpha
    TfByte, // VCIII T-format 8 bit generic sample
    TfPal8, // VCIII T-format 8-bit palette
    TfPal4, // VCIII T-format 4-bit palette
    TfEtc1, // VCIII T-format Ericsson Texture Compressed
    Bgr888, // RGB888 with R & B swapped
    Bgr888Np, // RGB888 with R & B swapped, but with no pitch, i.e. no padding after each row of pixels
    Bayer,    // Bayer image, extra defines which variant is being used
    Codec,    // General wrapper for codec images e.g. JPEG from camera
    YuvUv32,  // VCIII codec format
    TfY8,     // VCIII T-format 8-bit luma
    TfA8,     // VCIII T-format 8-bit alpha
    TfShort,  // VCIII T-format 16-bit generic sample
    Tf1Bpp,   // VCIII T-format 1bpp black/white
    OpenGl,
    Yuv444I,      // VCIII-B0 HVS YUV 4:4:4 interleaved samples
    Yuv422Planar, // Y, U, & V planes separately (YUV422 has them interleaved on a per line basis)
    Argb8888,     // 32bpp with 8bit alpha at MS byte, with R, G, B (LS byte)
    Xrgb8888,     // 32bpp with 8bit unused at MS byte, with R, G, B (LS byte)
    Yuv422YUYV,   // interleaved 8 bit samples of Y, U, Y, V
    Yuv422YVYU,   // interleaved 8 bit samples of Y, V, Y, U
    Yuv422UYVY,   // interleaved 8 bit samples of U, Y, V, Y
    Yuv422VYUY,   // interleaved 8 bit samples of V, Y, U, Y
    Rgbx32,       // 32bpp like RGBA32 but with unused alpha
    Rgbx8888,     // 32bpp, corresponding to RGBA with unused alpha
    Bgrx8888,     // 32bpp, corresponding to BGRA with unused alpha
    Yuv420Sp, // Y as a plane, then UV byte interleaved in plane with with same pitch, half height
    Yuv444Planar, // Y, U, & V planes separately 4:4:4
    TfU8,     // T-format 8-bit U - same as TF_Y8 buf from U plane
    TfV8,     // T-format 8-bit U - same as TF_Y8 buf from V plane
    Max,      // bounds for error checking
    ForceEnum16Bit = 0xffff,
}

// -------------------------------------------------------------------------------------------------
// STRUCTS
// -------------------------------------------------------------------------------------------------

#[repr(C)]
pub struct Image;

#[repr(C)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}
