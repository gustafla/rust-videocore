use rpi_window::{bcm_host, dispmanx};

#[link(name = "brcmEGL")]
#[link(name = "brcmGLESv2")]
extern "C" {
    fn glClearColor(r: f32, g: f32, b: f32, a: f32);
    fn glClear(mask: std::os::raw::c_uint);
}

const GL_COLOR_BUFFER_BIT: std::os::raw::c_uint = 0x00004000;

fn main() {
    bcm_host::init();
    let size = bcm_host::graphics_get_display_size(0).unwrap();

    // Set up dispmanx structs
    let mut src = dispmanx::Rect {
        x: 0,
        y: 0,
        width: (size.width as i32) << 16,
        height: (size.height as i32) << 16,
    };
    let mut dest = dispmanx::Rect {
        x: 0,
        y: 0,
        width: size.width as i32,
        height: size.height as i32,
    };
    let mut alpha = dispmanx::VCAlpha {
        flags: dispmanx::FlagsAlpha::FixedAllPixels,
        opacity: 255,
        mask: 0,
    };

    // Create dispmanx window
    let mut window = dispmanx::create_window(
        0,
        &mut dest,
        &mut src,
        &mut alpha,
        dispmanx::Transform::NoRotate,
    );

    // EGL for GLES2
    let egl = khronos_egl::Instance::new(khronos_egl::Static);
    let egl_display = egl.get_display(khronos_egl::DEFAULT_DISPLAY).unwrap();
    egl.initialize(egl_display).unwrap();
    let egl_attribs = [
        khronos_egl::RED_SIZE,
        5,
        khronos_egl::GREEN_SIZE,
        6,
        khronos_egl::BLUE_SIZE,
        5,
        khronos_egl::ALPHA_SIZE,
        0,
        khronos_egl::DEPTH_SIZE,
        0,
        khronos_egl::STENCIL_SIZE,
        0,
        khronos_egl::SAMPLE_BUFFERS,
        0,
        khronos_egl::NONE,
    ];
    let egl_config = egl
        .choose_first_config(egl_display, &egl_attribs)
        .unwrap()
        .unwrap();
    let egl_buffer = unsafe {
        egl.create_window_surface(
            egl_display,
            egl_config,
            &mut window as *mut dispmanx::Window as khronos_egl::NativeWindowType,
            None,
        )
    }
    .unwrap();
    let egl_context_attribs = [khronos_egl::CONTEXT_CLIENT_VERSION, 2, khronos_egl::NONE];
    let egl_context = egl
        .create_context(egl_display, egl_config, None, &egl_context_attribs)
        .unwrap();
    egl.make_current(
        egl_display,
        Some(egl_buffer),
        Some(egl_buffer),
        Some(egl_context),
    )
    .unwrap();

    // Fill the window with red using OpenGL
    unsafe {
        glClearColor(1., 0., 0., 1.);
        glClear(GL_COLOR_BUFFER_BIT);
    }

    // Show for a few seconds
    egl.swap_buffers(egl_display, egl_buffer).unwrap();
    std::thread::sleep(std::time::Duration::from_secs(5));

    bcm_host::deinit();
}
