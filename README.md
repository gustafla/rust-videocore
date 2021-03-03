# Dispmanx window for EGL/GL on the Raspberry Pi

This is an extremely trimmed fork of [videocore](https://crates.io/crates/videocore)
for opening a window to render with EGL/OpenGL ES.
If you need to use more of the VC API than this crate exposes,
use the original videcore crate instead.

# Usage

Call `bcm_host::init`, create a `dispmanx::Rect` for src and dest,
create a `dispmanx::VCAlpha`, and call `dispmanx::create_window`
which returns a window struct which can be passed to eglCreateWindowSurface.

See [examples](https://github.com/gustafla/rpi_window/tree/master/examples/gles2.rs) for details.

When building, make sure pkg-config can find the driver userspace libraries.
Build with the envvar `PKG_CONFIG_LIBDIR` set to `/opt/vc/lib/pkgconfig`, and
`PKG_CONFIG_ALLOW_CROSS` set to `1` if necessary.
See [pkg-config-rs](https://github.com/rust-lang/pkg-config-rs) for details.

For example: to build and run the example on a Raspberry Pi, run

```
PKG_CONFIG_LIBDIR=/opt/vc/lib/pkgconfig cargo run --example gles2
```
