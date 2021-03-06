// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::translate::*;
use std::fmt;

glib::glib_wrapper! {
    pub struct X11AppLaunchContext(Object<ffi::GdkX11AppLaunchContext, ffi::GdkX11AppLaunchContextClass>) @extends gdk::AppLaunchContext, gio::AppLaunchContext;

    match fn {
        get_type => || ffi::gdk_x11_app_launch_context_get_type(),
    }
}

impl X11AppLaunchContext {}

impl fmt::Display for X11AppLaunchContext {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "X11AppLaunchContext")
    }
}
