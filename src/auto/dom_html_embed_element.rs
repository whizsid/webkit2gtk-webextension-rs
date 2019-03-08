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
use glib::StaticType;
use glib::Value;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use libc;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct DOMHTMLEmbedElement(Object<ffi::WebKitDOMHTMLEmbedElement, ffi::WebKitDOMHTMLEmbedElementClass, DOMHTMLEmbedElementClass>) @extends DOMHTMLElement, DOMElement, DOMNode, DOMObject, @implements DOMEventTarget;

    match fn {
        get_type => || ffi::webkit_dom_html_embed_element_get_type(),
    }
}

pub const NONE_DOMHTML_EMBED_ELEMENT: Option<&DOMHTMLEmbedElement> = None;

pub trait DOMHTMLEmbedElementExt: 'static {
    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_align(&self) -> Option<GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_height(&self) -> libc::c_long;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_name(&self) -> Option<GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_src(&self) -> Option<GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_type_attr(&self) -> Option<GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_width(&self) -> libc::c_long;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_align(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_height(&self, value: libc::c_long);

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_name(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_src(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_type_attr(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_width(&self, value: libc::c_long);

    fn get_property_type(&self) -> Option<GString>;

    fn set_property_type<'a, P: Into<Option<&'a str>>>(&self, type_: P);

    fn connect_property_align_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_src_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMHTMLEmbedElement>> DOMHTMLEmbedElementExt for O {
    fn get_align(&self) -> Option<GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_embed_element_get_align(self.as_ref().to_glib_none().0))
        }
    }

    fn get_height(&self) -> libc::c_long {
        unsafe {
            ffi::webkit_dom_html_embed_element_get_height(self.as_ref().to_glib_none().0)
        }
    }

    fn get_name(&self) -> Option<GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_embed_element_get_name(self.as_ref().to_glib_none().0))
        }
    }

    fn get_src(&self) -> Option<GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_embed_element_get_src(self.as_ref().to_glib_none().0))
        }
    }

    fn get_type_attr(&self) -> Option<GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_embed_element_get_type_attr(self.as_ref().to_glib_none().0))
        }
    }

    fn get_width(&self) -> libc::c_long {
        unsafe {
            ffi::webkit_dom_html_embed_element_get_width(self.as_ref().to_glib_none().0)
        }
    }

    fn set_align(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_embed_element_set_align(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_height(&self, value: libc::c_long) {
        unsafe {
            ffi::webkit_dom_html_embed_element_set_height(self.as_ref().to_glib_none().0, value);
        }
    }

    fn set_name(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_embed_element_set_name(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_src(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_embed_element_set_src(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_type_attr(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_embed_element_set_type_attr(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_width(&self, value: libc::c_long) {
        unsafe {
            ffi::webkit_dom_html_embed_element_set_width(self.as_ref().to_glib_none().0, value);
        }
    }

    fn get_property_type(&self) -> Option<GString> {
        unsafe {
            let mut value = Value::from_type(<GString as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"type\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn set_property_type<'a, P: Into<Option<&'a str>>>(&self, type_: P) {
        let type_ = type_.into();
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"type\0".as_ptr() as *const _, Value::from(type_).to_glib_none().0);
        }
    }

    fn connect_property_align_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::align\0".as_ptr() as *const _,
                transmute(notify_align_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::height\0".as_ptr() as *const _,
                transmute(notify_height_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::name\0".as_ptr() as *const _,
                transmute(notify_name_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_src_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::src\0".as_ptr() as *const _,
                transmute(notify_src_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::type\0".as_ptr() as *const _,
                transmute(notify_type_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
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

unsafe extern "C" fn notify_align_trampoline<P>(this: *mut ffi::WebKitDOMHTMLEmbedElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLEmbedElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLEmbedElement::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_height_trampoline<P>(this: *mut ffi::WebKitDOMHTMLEmbedElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLEmbedElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLEmbedElement::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_name_trampoline<P>(this: *mut ffi::WebKitDOMHTMLEmbedElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLEmbedElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLEmbedElement::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_src_trampoline<P>(this: *mut ffi::WebKitDOMHTMLEmbedElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLEmbedElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLEmbedElement::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_type_trampoline<P>(this: *mut ffi::WebKitDOMHTMLEmbedElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLEmbedElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLEmbedElement::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_width_trampoline<P>(this: *mut ffi::WebKitDOMHTMLEmbedElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLEmbedElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLEmbedElement::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for DOMHTMLEmbedElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DOMHTMLEmbedElement")
    }
}
