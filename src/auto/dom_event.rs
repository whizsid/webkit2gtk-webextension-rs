// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use DOMEventTarget;
use DOMObject;
use ffi;
use glib;
use glib::StaticType;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use libc;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct DOMEvent(Object<ffi::WebKitDOMEvent, ffi::WebKitDOMEventClass>): DOMObject;

    match fn {
        get_type => || ffi::webkit_dom_event_get_type(),
    }
}

pub trait DOMEventExt {
    fn get_bubbles(&self) -> bool;

    fn get_cancel_bubble(&self) -> bool;

    fn get_cancelable(&self) -> bool;

    fn get_current_target(&self) -> Option<DOMEventTarget>;

    fn get_event_phase(&self) -> libc::c_ushort;

    fn get_event_type(&self) -> Option<String>;

    fn get_return_value(&self) -> bool;

    fn get_src_element(&self) -> Option<DOMEventTarget>;

    fn get_target(&self) -> Option<DOMEventTarget>;

    fn get_time_stamp(&self) -> u32;

    fn init_event(&self, eventTypeArg: &str, canBubbleArg: bool, cancelableArg: bool);

    fn prevent_default(&self);

    fn set_cancel_bubble(&self, value: bool);

    fn set_return_value(&self, value: bool);

    fn stop_propagation(&self);

    fn get_property_type(&self) -> Option<String>;

    fn connect_property_bubbles_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_cancel_bubble_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_cancelable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_current_target_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_event_phase_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_return_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_src_element_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_target_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_time_stamp_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMEvent> + IsA<glib::object::Object>> DOMEventExt for O {
    fn get_bubbles(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_event_get_bubbles(self.to_glib_none().0))
        }
    }

    fn get_cancel_bubble(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_event_get_cancel_bubble(self.to_glib_none().0))
        }
    }

    fn get_cancelable(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_event_get_cancelable(self.to_glib_none().0))
        }
    }

    fn get_current_target(&self) -> Option<DOMEventTarget> {
        unsafe {
            from_glib_full(ffi::webkit_dom_event_get_current_target(self.to_glib_none().0))
        }
    }

    fn get_event_phase(&self) -> libc::c_ushort {
        unsafe {
            ffi::webkit_dom_event_get_event_phase(self.to_glib_none().0)
        }
    }

    fn get_event_type(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_event_get_event_type(self.to_glib_none().0))
        }
    }

    fn get_return_value(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_event_get_return_value(self.to_glib_none().0))
        }
    }

    fn get_src_element(&self) -> Option<DOMEventTarget> {
        unsafe {
            from_glib_full(ffi::webkit_dom_event_get_src_element(self.to_glib_none().0))
        }
    }

    fn get_target(&self) -> Option<DOMEventTarget> {
        unsafe {
            from_glib_full(ffi::webkit_dom_event_get_target(self.to_glib_none().0))
        }
    }

    fn get_time_stamp(&self) -> u32 {
        unsafe {
            ffi::webkit_dom_event_get_time_stamp(self.to_glib_none().0)
        }
    }

    fn init_event(&self, eventTypeArg: &str, canBubbleArg: bool, cancelableArg: bool) {
        unsafe {
            ffi::webkit_dom_event_init_event(self.to_glib_none().0, eventTypeArg.to_glib_none().0, canBubbleArg.to_glib(), cancelableArg.to_glib());
        }
    }

    fn prevent_default(&self) {
        unsafe {
            ffi::webkit_dom_event_prevent_default(self.to_glib_none().0);
        }
    }

    fn set_cancel_bubble(&self, value: bool) {
        unsafe {
            ffi::webkit_dom_event_set_cancel_bubble(self.to_glib_none().0, value.to_glib());
        }
    }

    fn set_return_value(&self, value: bool) {
        unsafe {
            ffi::webkit_dom_event_set_return_value(self.to_glib_none().0, value.to_glib());
        }
    }

    fn stop_propagation(&self) {
        unsafe {
            ffi::webkit_dom_event_stop_propagation(self.to_glib_none().0);
        }
    }

    fn get_property_type(&self) -> Option<String> {
        unsafe {
            let mut value = Value::from_type(<String as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "type".to_glib_none().0, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn connect_property_bubbles_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::bubbles",
                transmute(notify_bubbles_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_cancel_bubble_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::cancel-bubble",
                transmute(notify_cancel_bubble_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_cancelable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::cancelable",
                transmute(notify_cancelable_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_current_target_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::current-target",
                transmute(notify_current_target_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_event_phase_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::event-phase",
                transmute(notify_event_phase_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_return_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::return-value",
                transmute(notify_return_value_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_src_element_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::src-element",
                transmute(notify_src_element_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_target_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::target",
                transmute(notify_target_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_time_stamp_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::time-stamp",
                transmute(notify_time_stamp_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::type",
                transmute(notify_type_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_bubbles_trampoline<P>(this: *mut ffi::WebKitDOMEvent, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMEvent> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMEvent::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_cancel_bubble_trampoline<P>(this: *mut ffi::WebKitDOMEvent, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMEvent> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMEvent::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_cancelable_trampoline<P>(this: *mut ffi::WebKitDOMEvent, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMEvent> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMEvent::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_current_target_trampoline<P>(this: *mut ffi::WebKitDOMEvent, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMEvent> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMEvent::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_event_phase_trampoline<P>(this: *mut ffi::WebKitDOMEvent, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMEvent> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMEvent::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_return_value_trampoline<P>(this: *mut ffi::WebKitDOMEvent, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMEvent> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMEvent::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_src_element_trampoline<P>(this: *mut ffi::WebKitDOMEvent, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMEvent> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMEvent::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_target_trampoline<P>(this: *mut ffi::WebKitDOMEvent, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMEvent> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMEvent::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_time_stamp_trampoline<P>(this: *mut ffi::WebKitDOMEvent, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMEvent> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMEvent::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_type_trampoline<P>(this: *mut ffi::WebKitDOMEvent, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMEvent> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMEvent::from_glib_borrow(this).downcast_unchecked())
}
