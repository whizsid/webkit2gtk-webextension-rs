// This file was generated by gir (https://github.com/gtk-rs/gir @ f5fca82)
// from gir-files (https://github.com/gtk-rs/gir-files @ b8f5ef1)
// DO NOT EDIT

use DOMDOMWindow;
use DOMEvent;
use DOMObject;
use DOMUIEvent;
use ffi;
use glib;
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
    pub struct DOMKeyboardEvent(Object<ffi::WebKitDOMKeyboardEvent, ffi::WebKitDOMKeyboardEventClass>): DOMUIEvent, DOMEvent, DOMObject;

    match fn {
        get_type => || ffi::webkit_dom_keyboard_event_get_type(),
    }
}

pub trait DOMKeyboardEventExt {
    fn get_alt_graph_key(&self) -> bool;

    fn get_alt_key(&self) -> bool;

    fn get_ctrl_key(&self) -> bool;

    fn get_key_identifier(&self) -> Option<String>;

    fn get_key_location(&self) -> libc::c_ulong;

    fn get_meta_key(&self) -> bool;

    fn get_modifier_state(&self, keyIdentifierArg: &str) -> bool;

    fn get_shift_key(&self) -> bool;

    fn init_keyboard_event(&self, type_: &str, canBubble: bool, cancelable: bool, view: &DOMDOMWindow, keyIdentifier: &str, location: libc::c_ulong, ctrlKey: bool, altKey: bool, shiftKey: bool, metaKey: bool, altGraphKey: bool);

    fn connect_property_alt_graph_key_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_alt_key_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_ctrl_key_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_key_identifier_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_key_location_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_meta_key_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_shift_key_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMKeyboardEvent> + IsA<glib::object::Object>> DOMKeyboardEventExt for O {
    fn get_alt_graph_key(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_keyboard_event_get_alt_graph_key(self.to_glib_none().0))
        }
    }

    fn get_alt_key(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_keyboard_event_get_alt_key(self.to_glib_none().0))
        }
    }

    fn get_ctrl_key(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_keyboard_event_get_ctrl_key(self.to_glib_none().0))
        }
    }

    fn get_key_identifier(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_keyboard_event_get_key_identifier(self.to_glib_none().0))
        }
    }

    fn get_key_location(&self) -> libc::c_ulong {
        unsafe {
            ffi::webkit_dom_keyboard_event_get_key_location(self.to_glib_none().0)
        }
    }

    fn get_meta_key(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_keyboard_event_get_meta_key(self.to_glib_none().0))
        }
    }

    fn get_modifier_state(&self, keyIdentifierArg: &str) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_keyboard_event_get_modifier_state(self.to_glib_none().0, keyIdentifierArg.to_glib_none().0))
        }
    }

    fn get_shift_key(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_keyboard_event_get_shift_key(self.to_glib_none().0))
        }
    }

    fn init_keyboard_event(&self, type_: &str, canBubble: bool, cancelable: bool, view: &DOMDOMWindow, keyIdentifier: &str, location: libc::c_ulong, ctrlKey: bool, altKey: bool, shiftKey: bool, metaKey: bool, altGraphKey: bool) {
        unsafe {
            ffi::webkit_dom_keyboard_event_init_keyboard_event(self.to_glib_none().0, type_.to_glib_none().0, canBubble.to_glib(), cancelable.to_glib(), view.to_glib_none().0, keyIdentifier.to_glib_none().0, location, ctrlKey.to_glib(), altKey.to_glib(), shiftKey.to_glib(), metaKey.to_glib(), altGraphKey.to_glib());
        }
    }

    fn connect_property_alt_graph_key_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::alt-graph-key",
                transmute(notify_alt_graph_key_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_alt_key_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::alt-key",
                transmute(notify_alt_key_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_ctrl_key_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::ctrl-key",
                transmute(notify_ctrl_key_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_key_identifier_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::key-identifier",
                transmute(notify_key_identifier_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_key_location_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::key-location",
                transmute(notify_key_location_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_meta_key_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::meta-key",
                transmute(notify_meta_key_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_shift_key_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::shift-key",
                transmute(notify_shift_key_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_alt_graph_key_trampoline<P>(this: *mut ffi::WebKitDOMKeyboardEvent, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMKeyboardEvent> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMKeyboardEvent::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_alt_key_trampoline<P>(this: *mut ffi::WebKitDOMKeyboardEvent, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMKeyboardEvent> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMKeyboardEvent::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_ctrl_key_trampoline<P>(this: *mut ffi::WebKitDOMKeyboardEvent, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMKeyboardEvent> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMKeyboardEvent::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_key_identifier_trampoline<P>(this: *mut ffi::WebKitDOMKeyboardEvent, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMKeyboardEvent> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMKeyboardEvent::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_key_location_trampoline<P>(this: *mut ffi::WebKitDOMKeyboardEvent, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMKeyboardEvent> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMKeyboardEvent::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_meta_key_trampoline<P>(this: *mut ffi::WebKitDOMKeyboardEvent, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMKeyboardEvent> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMKeyboardEvent::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_shift_key_trampoline<P>(this: *mut ffi::WebKitDOMKeyboardEvent, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMKeyboardEvent> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMKeyboardEvent::from_glib_borrow(this).downcast_unchecked())
}
