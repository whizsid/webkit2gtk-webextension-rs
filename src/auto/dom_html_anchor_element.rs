// This file was generated by gir (https://github.com/gtk-rs/gir @ f5fca82)
// from gir-files (https://github.com/gtk-rs/gir-files @ b8f5ef1)
// DO NOT EDIT

use DOMElement;
use DOMEventTarget;
use DOMHTMLElement;
use DOMNode;
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
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct DOMHTMLAnchorElement(Object<ffi::WebKitDOMHTMLAnchorElement, ffi::WebKitDOMHTMLAnchorElementClass>): DOMHTMLElement, DOMElement, DOMNode, DOMObject, DOMEventTarget;

    match fn {
        get_type => || ffi::webkit_dom_html_anchor_element_get_type(),
    }
}

pub trait DOMHTMLAnchorElementExt {
    fn get_charset(&self) -> Option<String>;

    fn get_coords(&self) -> Option<String>;

    fn get_hash(&self) -> Option<String>;

    fn get_host(&self) -> Option<String>;

    fn get_hostname(&self) -> Option<String>;

    fn get_href(&self) -> Option<String>;

    fn get_hreflang(&self) -> Option<String>;

    fn get_name(&self) -> Option<String>;

    fn get_pathname(&self) -> Option<String>;

    fn get_port(&self) -> Option<String>;

    fn get_protocol(&self) -> Option<String>;

    fn get_rel(&self) -> Option<String>;

    fn get_rev(&self) -> Option<String>;

    fn get_search(&self) -> Option<String>;

    fn get_shape(&self) -> Option<String>;

    fn get_target(&self) -> Option<String>;

    fn get_text(&self) -> Option<String>;

    fn get_type_attr(&self) -> Option<String>;

    fn set_charset(&self, value: &str);

    fn set_coords(&self, value: &str);

    fn set_hash(&self, value: &str);

    fn set_host(&self, value: &str);

    fn set_hostname(&self, value: &str);

    fn set_href(&self, value: &str);

    fn set_hreflang(&self, value: &str);

    fn set_name(&self, value: &str);

    fn set_pathname(&self, value: &str);

    fn set_port(&self, value: &str);

    fn set_protocol(&self, value: &str);

    fn set_rel(&self, value: &str);

    fn set_rev(&self, value: &str);

    fn set_search(&self, value: &str);

    fn set_shape(&self, value: &str);

    fn set_target(&self, value: &str);

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn set_text(&self, value: &str);

    fn set_type_attr(&self, value: &str);

    fn set_property_text(&self, text: Option<&str>);

    fn get_property_type(&self) -> Option<String>;

    fn set_property_type(&self, type_: Option<&str>);

    fn connect_property_charset_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_coords_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_hash_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_host_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_hostname_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_href_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_hreflang_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_pathname_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_port_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_protocol_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_rel_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_rev_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_search_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_shape_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_target_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMHTMLAnchorElement> + IsA<glib::object::Object>> DOMHTMLAnchorElementExt for O {
    fn get_charset(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_anchor_element_get_charset(self.to_glib_none().0))
        }
    }

    fn get_coords(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_anchor_element_get_coords(self.to_glib_none().0))
        }
    }

    fn get_hash(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_anchor_element_get_hash(self.to_glib_none().0))
        }
    }

    fn get_host(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_anchor_element_get_host(self.to_glib_none().0))
        }
    }

    fn get_hostname(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_anchor_element_get_hostname(self.to_glib_none().0))
        }
    }

    fn get_href(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_anchor_element_get_href(self.to_glib_none().0))
        }
    }

    fn get_hreflang(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_anchor_element_get_hreflang(self.to_glib_none().0))
        }
    }

    fn get_name(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_anchor_element_get_name(self.to_glib_none().0))
        }
    }

    fn get_pathname(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_anchor_element_get_pathname(self.to_glib_none().0))
        }
    }

    fn get_port(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_anchor_element_get_port(self.to_glib_none().0))
        }
    }

    fn get_protocol(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_anchor_element_get_protocol(self.to_glib_none().0))
        }
    }

    fn get_rel(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_anchor_element_get_rel(self.to_glib_none().0))
        }
    }

    fn get_rev(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_anchor_element_get_rev(self.to_glib_none().0))
        }
    }

    fn get_search(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_anchor_element_get_search(self.to_glib_none().0))
        }
    }

    fn get_shape(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_anchor_element_get_shape(self.to_glib_none().0))
        }
    }

    fn get_target(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_anchor_element_get_target(self.to_glib_none().0))
        }
    }

    fn get_text(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_anchor_element_get_text(self.to_glib_none().0))
        }
    }

    fn get_type_attr(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_anchor_element_get_type_attr(self.to_glib_none().0))
        }
    }

    fn set_charset(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_anchor_element_set_charset(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_coords(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_anchor_element_set_coords(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_hash(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_anchor_element_set_hash(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_host(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_anchor_element_set_host(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_hostname(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_anchor_element_set_hostname(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_href(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_anchor_element_set_href(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_hreflang(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_anchor_element_set_hreflang(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_name(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_anchor_element_set_name(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_pathname(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_anchor_element_set_pathname(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_port(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_anchor_element_set_port(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_protocol(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_anchor_element_set_protocol(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_rel(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_anchor_element_set_rel(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_rev(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_anchor_element_set_rev(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_search(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_anchor_element_set_search(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_shape(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_anchor_element_set_shape(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_target(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_anchor_element_set_target(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn set_text(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_anchor_element_set_text(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_type_attr(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_anchor_element_set_type_attr(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_property_text(&self, text: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "text".to_glib_none().0, Value::from(text).to_glib_none().0);
        }
    }

    fn get_property_type(&self) -> Option<String> {
        unsafe {
            let mut value = Value::from_type(<String as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "type".to_glib_none().0, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn set_property_type(&self, type_: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "type".to_glib_none().0, Value::from(type_).to_glib_none().0);
        }
    }

    fn connect_property_charset_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::charset",
                transmute(notify_charset_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_coords_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::coords",
                transmute(notify_coords_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_hash_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::hash",
                transmute(notify_hash_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_host_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::host",
                transmute(notify_host_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_hostname_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::hostname",
                transmute(notify_hostname_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_href_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::href",
                transmute(notify_href_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_hreflang_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::hreflang",
                transmute(notify_hreflang_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::name",
                transmute(notify_name_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_pathname_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::pathname",
                transmute(notify_pathname_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_port_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::port",
                transmute(notify_port_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_protocol_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::protocol",
                transmute(notify_protocol_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_rel_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::rel",
                transmute(notify_rel_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_rev_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::rev",
                transmute(notify_rev_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_search_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::search",
                transmute(notify_search_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_shape_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::shape",
                transmute(notify_shape_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_target_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::target",
                transmute(notify_target_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::text",
                transmute(notify_text_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
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

unsafe extern "C" fn notify_charset_trampoline<P>(this: *mut ffi::WebKitDOMHTMLAnchorElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLAnchorElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLAnchorElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_coords_trampoline<P>(this: *mut ffi::WebKitDOMHTMLAnchorElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLAnchorElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLAnchorElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_hash_trampoline<P>(this: *mut ffi::WebKitDOMHTMLAnchorElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLAnchorElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLAnchorElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_host_trampoline<P>(this: *mut ffi::WebKitDOMHTMLAnchorElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLAnchorElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLAnchorElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_hostname_trampoline<P>(this: *mut ffi::WebKitDOMHTMLAnchorElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLAnchorElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLAnchorElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_href_trampoline<P>(this: *mut ffi::WebKitDOMHTMLAnchorElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLAnchorElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLAnchorElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_hreflang_trampoline<P>(this: *mut ffi::WebKitDOMHTMLAnchorElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLAnchorElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLAnchorElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_name_trampoline<P>(this: *mut ffi::WebKitDOMHTMLAnchorElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLAnchorElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLAnchorElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_pathname_trampoline<P>(this: *mut ffi::WebKitDOMHTMLAnchorElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLAnchorElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLAnchorElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_port_trampoline<P>(this: *mut ffi::WebKitDOMHTMLAnchorElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLAnchorElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLAnchorElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_protocol_trampoline<P>(this: *mut ffi::WebKitDOMHTMLAnchorElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLAnchorElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLAnchorElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_rel_trampoline<P>(this: *mut ffi::WebKitDOMHTMLAnchorElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLAnchorElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLAnchorElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_rev_trampoline<P>(this: *mut ffi::WebKitDOMHTMLAnchorElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLAnchorElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLAnchorElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_search_trampoline<P>(this: *mut ffi::WebKitDOMHTMLAnchorElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLAnchorElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLAnchorElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_shape_trampoline<P>(this: *mut ffi::WebKitDOMHTMLAnchorElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLAnchorElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLAnchorElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_target_trampoline<P>(this: *mut ffi::WebKitDOMHTMLAnchorElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLAnchorElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLAnchorElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_text_trampoline<P>(this: *mut ffi::WebKitDOMHTMLAnchorElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLAnchorElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLAnchorElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_type_trampoline<P>(this: *mut ffi::WebKitDOMHTMLAnchorElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLAnchorElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLAnchorElement::from_glib_borrow(this).downcast_unchecked())
}
