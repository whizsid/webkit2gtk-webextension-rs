// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use DOMDOMWindow;
use DOMEvent;
use DOMMouseEvent;
use DOMObject;
use DOMUIEvent;
use ffi;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_ffi;
use libc;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct DOMWheelEvent(Object<ffi::WebKitDOMWheelEvent, ffi::WebKitDOMWheelEventClass, DOMWheelEventClass>) @extends DOMMouseEvent, DOMUIEvent, DOMEvent, DOMObject;

    match fn {
        get_type => || ffi::webkit_dom_wheel_event_get_type(),
    }
}

pub const NONE_DOM_WHEEL_EVENT: Option<&DOMWheelEvent> = None;

pub trait DOMWheelEventExt: 'static {
    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_wheel_delta(&self) -> libc::c_long;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_wheel_delta_x(&self) -> libc::c_long;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_wheel_delta_y(&self) -> libc::c_long;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn init_wheel_event<P: IsA<DOMDOMWindow>>(&self, wheelDeltaX: libc::c_long, wheelDeltaY: libc::c_long, view: &P, screenX: libc::c_long, screenY: libc::c_long, clientX: libc::c_long, clientY: libc::c_long, ctrlKey: bool, altKey: bool, shiftKey: bool, metaKey: bool);

    fn connect_property_wheel_delta_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_wheel_delta_x_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_wheel_delta_y_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMWheelEvent>> DOMWheelEventExt for O {
    fn get_wheel_delta(&self) -> libc::c_long {
        unsafe {
            ffi::webkit_dom_wheel_event_get_wheel_delta(self.as_ref().to_glib_none().0)
        }
    }

    fn get_wheel_delta_x(&self) -> libc::c_long {
        unsafe {
            ffi::webkit_dom_wheel_event_get_wheel_delta_x(self.as_ref().to_glib_none().0)
        }
    }

    fn get_wheel_delta_y(&self) -> libc::c_long {
        unsafe {
            ffi::webkit_dom_wheel_event_get_wheel_delta_y(self.as_ref().to_glib_none().0)
        }
    }

    fn init_wheel_event<P: IsA<DOMDOMWindow>>(&self, wheelDeltaX: libc::c_long, wheelDeltaY: libc::c_long, view: &P, screenX: libc::c_long, screenY: libc::c_long, clientX: libc::c_long, clientY: libc::c_long, ctrlKey: bool, altKey: bool, shiftKey: bool, metaKey: bool) {
        unsafe {
            ffi::webkit_dom_wheel_event_init_wheel_event(self.as_ref().to_glib_none().0, wheelDeltaX, wheelDeltaY, view.as_ref().to_glib_none().0, screenX, screenY, clientX, clientY, ctrlKey.to_glib(), altKey.to_glib(), shiftKey.to_glib(), metaKey.to_glib());
        }
    }

    fn connect_property_wheel_delta_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::wheel-delta\0".as_ptr() as *const _,
                transmute(notify_wheel_delta_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_wheel_delta_x_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::wheel-delta-x\0".as_ptr() as *const _,
                transmute(notify_wheel_delta_x_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_wheel_delta_y_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::wheel-delta-y\0".as_ptr() as *const _,
                transmute(notify_wheel_delta_y_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_wheel_delta_trampoline<P>(this: *mut ffi::WebKitDOMWheelEvent, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMWheelEvent> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMWheelEvent::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_wheel_delta_x_trampoline<P>(this: *mut ffi::WebKitDOMWheelEvent, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMWheelEvent> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMWheelEvent::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_wheel_delta_y_trampoline<P>(this: *mut ffi::WebKitDOMWheelEvent, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMWheelEvent> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMWheelEvent::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for DOMWheelEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DOMWheelEvent")
    }
}
