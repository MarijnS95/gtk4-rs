use glib::subclass::prelude::*;
use glib::translate::*;
use glib::Cast;

use super::widget::WidgetImpl;
use crate::{Popover, Widget};

pub trait PopoverImpl: PopoverImplExt + WidgetImpl {
    fn activate_default(&self, button: &Self::Type) {
        self.parent_activate_default(button)
    }

    fn closed(&self, button: &Self::Type) {
        self.parent_closed(button)
    }
}

pub trait PopoverImplExt: ObjectSubclass {
    fn parent_activate_default(&self, button: &Self::Type);
    fn parent_closed(&self, button: &Self::Type);
}

impl<T: PopoverImpl> PopoverImplExt for T {
    fn parent_activate_default(&self, button: &Self::Type) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GtkPopoverClass;
            if let Some(f) = (*parent_class).activate_default {
                f(button.unsafe_cast_ref::<Popover>().to_glib_none().0)
            }
        }
    }

    fn parent_closed(&self, button: &Self::Type) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GtkPopoverClass;
            if let Some(f) = (*parent_class).closed {
                f(button.unsafe_cast_ref::<Popover>().to_glib_none().0)
            }
        }
    }
}

unsafe impl<T: PopoverImpl> IsSubclassable<T> for Popover {
    fn override_vfuncs(class: &mut glib::Class<Self>) {
        <Widget as IsSubclassable<T>>::override_vfuncs(class);

        let klass = class.as_mut();
        klass.activate_default = Some(popover_activate_default::<T>);
        klass.closed = Some(popover_closed::<T>);
    }
}

unsafe extern "C" fn popover_activate_default<T: PopoverImpl>(ptr: *mut ffi::GtkPopover) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Popover> = from_glib_borrow(ptr);

    imp.activate_default(wrap.unsafe_cast_ref())
}

unsafe extern "C" fn popover_closed<T: PopoverImpl>(ptr: *mut ffi::GtkPopover) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Popover> = from_glib_borrow(ptr);

    imp.closed(wrap.unsafe_cast_ref())
}
