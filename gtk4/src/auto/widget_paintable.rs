// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Widget;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::glib_wrapper! {
    pub struct WidgetPaintable(Object<ffi::GtkWidgetPaintable, ffi::GtkWidgetPaintableClass>) @implements gdk::Paintable;

    match fn {
        get_type => || ffi::gtk_widget_paintable_get_type(),
    }
}

impl WidgetPaintable {
    pub fn new<P: IsA<Widget>>(widget: Option<&P>) -> WidgetPaintable {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_widget_paintable_new(
                widget.map(|p| p.as_ref()).to_glib_none().0,
            ))
        }
    }
}

#[derive(Clone, Default)]
pub struct WidgetPaintableBuilder {
    widget: Option<Widget>,
}

impl WidgetPaintableBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> WidgetPaintable {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref widget) = self.widget {
            properties.push(("widget", widget));
        }
        let ret = glib::Object::new(WidgetPaintable::static_type(), &properties)
            .expect("object new")
            .downcast::<WidgetPaintable>()
            .expect("downcast");
        ret
    }

    pub fn widget<P: IsA<Widget>>(mut self, widget: &P) -> Self {
        self.widget = Some(widget.clone().upcast());
        self
    }
}

pub const NONE_WIDGET_PAINTABLE: Option<&WidgetPaintable> = None;

pub trait WidgetPaintableExt: 'static {
    fn get_widget(&self) -> Option<Widget>;

    fn set_widget<P: IsA<Widget>>(&self, widget: Option<&P>);

    fn connect_property_widget_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<WidgetPaintable>> WidgetPaintableExt for O {
    fn get_widget(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_widget_paintable_get_widget(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_widget<P: IsA<Widget>>(&self, widget: Option<&P>) {
        unsafe {
            ffi::gtk_widget_paintable_set_widget(
                self.as_ref().to_glib_none().0,
                widget.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    fn connect_property_widget_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_widget_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkWidgetPaintable,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<WidgetPaintable>,
        {
            let f: &F = &*(f as *const F);
            f(&WidgetPaintable::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::widget\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_widget_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for WidgetPaintable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "WidgetPaintable")
    }
}
