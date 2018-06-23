// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use DOMElement;
use DOMEventTarget;
use DOMHTMLElement;
use DOMHTMLFormElement;
use DOMNode;
use DOMObject;
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
    pub struct DOMHTMLOptionElement(Object<ffi::WebKitDOMHTMLOptionElement, ffi::WebKitDOMHTMLOptionElementClass>): DOMHTMLElement, DOMElement, DOMNode, DOMObject, DOMEventTarget;

    match fn {
        get_type => || ffi::webkit_dom_html_option_element_get_type(),
    }
}

pub trait DOMHTMLOptionElementExt {
    fn get_default_selected(&self) -> bool;

    fn get_disabled(&self) -> bool;

    fn get_form(&self) -> Option<DOMHTMLFormElement>;

    fn get_index(&self) -> libc::c_long;

    fn get_label(&self) -> Option<String>;

    fn get_selected(&self) -> bool;

    fn get_text(&self) -> Option<String>;

    fn get_value(&self) -> Option<String>;

    fn set_default_selected(&self, value: bool);

    fn set_disabled(&self, value: bool);

    fn set_label(&self, value: &str);

    fn set_selected(&self, value: bool);

    fn set_value(&self, value: &str);

    fn connect_property_default_selected_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_disabled_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_form_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_index_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_label_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_selected_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMHTMLOptionElement> + IsA<glib::object::Object>> DOMHTMLOptionElementExt for O {
    fn get_default_selected(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_html_option_element_get_default_selected(self.to_glib_none().0))
        }
    }

    fn get_disabled(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_html_option_element_get_disabled(self.to_glib_none().0))
        }
    }

    fn get_form(&self) -> Option<DOMHTMLFormElement> {
        unsafe {
            from_glib_none(ffi::webkit_dom_html_option_element_get_form(self.to_glib_none().0))
        }
    }

    fn get_index(&self) -> libc::c_long {
        unsafe {
            ffi::webkit_dom_html_option_element_get_index(self.to_glib_none().0)
        }
    }

    fn get_label(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_option_element_get_label(self.to_glib_none().0))
        }
    }

    fn get_selected(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_html_option_element_get_selected(self.to_glib_none().0))
        }
    }

    fn get_text(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_option_element_get_text(self.to_glib_none().0))
        }
    }

    fn get_value(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_option_element_get_value(self.to_glib_none().0))
        }
    }

    fn set_default_selected(&self, value: bool) {
        unsafe {
            ffi::webkit_dom_html_option_element_set_default_selected(self.to_glib_none().0, value.to_glib());
        }
    }

    fn set_disabled(&self, value: bool) {
        unsafe {
            ffi::webkit_dom_html_option_element_set_disabled(self.to_glib_none().0, value.to_glib());
        }
    }

    fn set_label(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_option_element_set_label(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_selected(&self, value: bool) {
        unsafe {
            ffi::webkit_dom_html_option_element_set_selected(self.to_glib_none().0, value.to_glib());
        }
    }

    fn set_value(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_option_element_set_value(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn connect_property_default_selected_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::default-selected",
                transmute(notify_default_selected_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_disabled_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::disabled",
                transmute(notify_disabled_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_form_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::form",
                transmute(notify_form_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_index_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::index",
                transmute(notify_index_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_label_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::label",
                transmute(notify_label_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_selected_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::selected",
                transmute(notify_selected_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::text",
                transmute(notify_text_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::value",
                transmute(notify_value_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_default_selected_trampoline<P>(this: *mut ffi::WebKitDOMHTMLOptionElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLOptionElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLOptionElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_disabled_trampoline<P>(this: *mut ffi::WebKitDOMHTMLOptionElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLOptionElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLOptionElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_form_trampoline<P>(this: *mut ffi::WebKitDOMHTMLOptionElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLOptionElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLOptionElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_index_trampoline<P>(this: *mut ffi::WebKitDOMHTMLOptionElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLOptionElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLOptionElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_label_trampoline<P>(this: *mut ffi::WebKitDOMHTMLOptionElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLOptionElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLOptionElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_selected_trampoline<P>(this: *mut ffi::WebKitDOMHTMLOptionElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLOptionElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLOptionElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_text_trampoline<P>(this: *mut ffi::WebKitDOMHTMLOptionElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLOptionElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLOptionElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_value_trampoline<P>(this: *mut ffi::WebKitDOMHTMLOptionElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLOptionElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLOptionElement::from_glib_borrow(this).downcast_unchecked())
}
