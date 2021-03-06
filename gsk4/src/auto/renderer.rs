// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::RenderNode;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::Value;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use std::ptr;

glib::glib_wrapper! {
    pub struct Renderer(Object<ffi::GskRenderer, ffi::GskRendererClass>);

    match fn {
        get_type => || ffi::gsk_renderer_get_type(),
    }
}

impl Renderer {
    pub fn new_for_surface(surface: &gdk::Surface) -> Option<Renderer> {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gsk_renderer_new_for_surface(surface.to_glib_none().0)) }
    }
}

pub const NONE_RENDERER: Option<&Renderer> = None;

pub trait RendererExt: 'static {
    fn get_surface(&self) -> Option<gdk::Surface>;

    fn is_realized(&self) -> bool;

    fn realize(&self, surface: &gdk::Surface) -> Result<(), glib::Error>;

    fn render<P: IsA<RenderNode>>(&self, root: &P, region: Option<&cairo::Region>);

    fn render_texture<P: IsA<RenderNode>>(
        &self,
        root: &P,
        viewport: Option<&graphene::Rect>,
    ) -> Option<gdk::Texture>;

    fn unrealize(&self);

    fn get_property_realized(&self) -> bool;

    fn connect_property_realized_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_surface_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Renderer>> RendererExt for O {
    fn get_surface(&self) -> Option<gdk::Surface> {
        unsafe {
            from_glib_none(ffi::gsk_renderer_get_surface(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_realized(&self) -> bool {
        unsafe {
            from_glib(ffi::gsk_renderer_is_realized(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn realize(&self, surface: &gdk::Surface) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::gsk_renderer_realize(
                self.as_ref().to_glib_none().0,
                surface.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn render<P: IsA<RenderNode>>(&self, root: &P, region: Option<&cairo::Region>) {
        unsafe {
            ffi::gsk_renderer_render(
                self.as_ref().to_glib_none().0,
                root.as_ref().to_glib_none().0,
                region.to_glib_none().0,
            );
        }
    }

    fn render_texture<P: IsA<RenderNode>>(
        &self,
        root: &P,
        viewport: Option<&graphene::Rect>,
    ) -> Option<gdk::Texture> {
        unsafe {
            from_glib_full(ffi::gsk_renderer_render_texture(
                self.as_ref().to_glib_none().0,
                root.as_ref().to_glib_none().0,
                viewport.to_glib_none().0,
            ))
        }
    }

    fn unrealize(&self) {
        unsafe {
            ffi::gsk_renderer_unrealize(self.as_ref().to_glib_none().0);
        }
    }

    fn get_property_realized(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"realized\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `realized` getter")
                .unwrap()
        }
    }

    fn connect_property_realized_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_realized_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GskRenderer,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Renderer>,
        {
            let f: &F = &*(f as *const F);
            f(&Renderer::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::realized\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_realized_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_surface_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_surface_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GskRenderer,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Renderer>,
        {
            let f: &F = &*(f as *const F);
            f(&Renderer::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::surface\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_surface_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Renderer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Renderer")
    }
}
