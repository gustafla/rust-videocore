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

// -------------------------------------------------------------------------------------------------
// STRUCTS
// -------------------------------------------------------------------------------------------------

pub struct GraphicsDisplaySize {
    pub height: u32,
    pub width: u32,
}

// -------------------------------------------------------------------------------------------------
// FUNCTIONS
// -------------------------------------------------------------------------------------------------

pub fn deinit() {
    unsafe {
        ffi::bcm_host_deinit();
    }
}

pub fn graphics_get_display_size(display_number: u16) -> Option<GraphicsDisplaySize> {
    unsafe {
        let mut width: u32 = 0;
        let mut height: u32 = 0;

        if ffi::graphics_get_display_size(display_number, &mut width, &mut height) == 0 {
            Some(GraphicsDisplaySize { height, width })
        } else {
            None
        }
    }
}

pub fn init() {
    unsafe {
        ffi::bcm_host_init();
    }
}

// -------------------------------------------------------------------------------------------------
// FFI
// -------------------------------------------------------------------------------------------------

mod ffi {
    extern "C" {
        pub fn bcm_host_deinit();

        pub fn bcm_host_init();

        pub fn graphics_get_display_size(
            display_number: u16,
            width: *mut u32,
            height: *mut u32,
        ) -> i32;
    }
}
