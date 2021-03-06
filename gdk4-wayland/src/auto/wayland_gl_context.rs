// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::translate::*;
use std::fmt;

glib::glib_wrapper! {
    pub struct WaylandGLContext(Object<ffi::GdkWaylandGLContext, ffi::GdkWaylandGLContextClass>) @extends gdk::GLContext, gdk::DrawContext;

    match fn {
        get_type => || ffi::gdk_wayland_gl_context_get_type(),
    }
}

impl WaylandGLContext {}

impl fmt::Display for WaylandGLContext {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "WaylandGLContext")
    }
}
