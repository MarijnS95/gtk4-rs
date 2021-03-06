// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Buildable;
use crate::Filter;
use crate::MultiFilter;
use glib::translate::*;
use std::fmt;

glib::glib_wrapper! {
    pub struct EveryFilter(Object<ffi::GtkEveryFilter, ffi::GtkEveryFilterClass>) @extends MultiFilter, Filter, @implements gio::ListModel, Buildable;

    match fn {
        get_type => || ffi::gtk_every_filter_get_type(),
    }
}

impl EveryFilter {
    pub fn new() -> EveryFilter {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gtk_every_filter_new()) }
    }
}

impl Default for EveryFilter {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for EveryFilter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "EveryFilter")
    }
}
