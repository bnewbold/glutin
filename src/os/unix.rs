#![cfg(any(target_os = "linux", target_os = "dragonfly", target_os = "freebsd", target_os = "openbsd"))]

use std::os::raw::c_ulong;

use libc;
use Window;
use platform::Window as LinuxWindow;
use WindowBuilder;

/// Additional methods on `Window` that are specific to Unix.
pub trait WindowExt {
    /// Returns a pointer to the `Window` object of xlib that is used by this window.
    ///
    /// Returns `None` if the window doesn't use xlib (if it uses wayland for example).
    ///
    /// The pointer will become invalid when the glutin `Window` is destroyed.
    fn get_xlib_window(&self) -> Option<*mut libc::c_void>;

    /// Returns a pointer to the `Display` object of xlib that is used by this window.
    ///
    /// Returns `None` if the window doesn't use xlib (if it uses wayland for example).
    ///
    /// The pointer will become invalid when the glutin `Window` is destroyed.
    fn get_xlib_display(&self) -> Option<*mut libc::c_void>;
}

impl WindowExt for Window {
    #[inline]
    fn get_xlib_window(&self) -> Option<*mut libc::c_void> {
        match self.window {
            LinuxWindow::X(ref w) => Some(w.get_xlib_window()),
            _ => None
        }
    }

    #[inline]
    fn get_xlib_display(&self) -> Option<*mut libc::c_void> {
        match self.window {
            LinuxWindow::X(ref w) => Some(w.get_xlib_display()),
            _ => None
        }
    }
}

/// Additional methods on `WindowBuilder` that are specific to Unix.
pub trait WindowBuilderExt<'a> {
    fn from_existing_window(mut self, window_id: c_ulong)  -> WindowBuilder<'a>;
}

impl<'a> WindowBuilderExt<'a> for WindowBuilder<'a> {

    /// Tells this (UNIX/X11) WindowBuilder to use an existing X window (eg,
    /// one created by another application) instead of creating a new window.
    fn from_existing_window(mut self, window_id: c_ulong) -> WindowBuilder<'a> {
        self.platform_specific.existing_x11_window_id = Some(window_id);
        self
    }
}
