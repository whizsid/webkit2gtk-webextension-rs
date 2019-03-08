// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use DOMElement;
use DOMEventTarget;
use DOMHTMLElement;
use DOMNode;
use DOMObject;
use ffi;
use glib::GString;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_ffi;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct DOMHTMLHRElement(Object<ffi::WebKitDOMHTMLHRElement, ffi::WebKitDOMHTMLHRElementClass, DOMHTMLHRElementClass>) @extends DOMHTMLElement, DOMElement, DOMNode, DOMObject, @implements DOMEventTarget;

    match fn {
        get_type => || ffi::webkit_dom_html_hr_element_get_type(),
    }
}

pub const NONE_DOMHTMLHR_ELEMENT: Option<&DOMHTMLHRElement> = None;

pub trait DOMHTMLHRElementExt: 'static {
    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_align(&self) -> Option<GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_no_shade(&self) -> bool;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_size(&self) -> Option<GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_width(&self) -> Option<GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_align(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_no_shade(&self, value: bool);

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_size(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_width(&self, value: &str);

    fn connect_property_align_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_no_shade_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMHTMLHRElement>> DOMHTMLHRElementExt for O {
    fn get_align(&self) -> Option<GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_hr_element_get_align(self.as_ref().to_glib_none().0))
        }
    }

    fn get_no_shade(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_html_hr_element_get_no_shade(self.as_ref().to_glib_none().0))
        }
    }

    fn get_size(&self) -> Option<GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_hr_element_get_size(self.as_ref().to_glib_none().0))
        }
    }

    fn get_width(&self) -> Option<GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_hr_element_get_width(self.as_ref().to_glib_none().0))
        }
    }

    fn set_align(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_hr_element_set_align(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_no_shade(&self, value: bool) {
        unsafe {
            ffi::webkit_dom_html_hr_element_set_no_shade(self.as_ref().to_glib_none().0, value.to_glib());
        }
    }

    fn set_size(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_hr_element_set_size(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_width(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_hr_element_set_width(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn connect_property_align_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::align\0".as_ptr() as *const _,
                transmute(notify_align_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_no_shade_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::no-shade\0".as_ptr() as *const _,
                transmute(notify_no_shade_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::size\0".as_ptr() as *const _,
                transmute(notify_size_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::width\0".as_ptr() as *const _,
                transmute(notify_width_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_align_trampoline<P>(this: *mut ffi::WebKitDOMHTMLHRElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLHRElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLHRElement::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_no_shade_trampoline<P>(this: *mut ffi::WebKitDOMHTMLHRElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLHRElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLHRElement::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_size_trampoline<P>(this: *mut ffi::WebKitDOMHTMLHRElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLHRElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLHRElement::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_width_trampoline<P>(this: *mut ffi::WebKitDOMHTMLHRElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLHRElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLHRElement::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for DOMHTMLHRElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DOMHTMLHRElement")
    }
}
