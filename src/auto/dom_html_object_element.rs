// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use DOMDocument;
use DOMElement;
use DOMEventTarget;
use DOMHTMLElement;
use DOMHTMLFormElement;
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
    pub struct DOMHTMLObjectElement(Object<ffi::WebKitDOMHTMLObjectElement, ffi::WebKitDOMHTMLObjectElementClass, DOMHTMLObjectElementClass>) @extends DOMHTMLElement, DOMElement, DOMNode, DOMObject, @implements DOMEventTarget;

    match fn {
        get_type => || ffi::webkit_dom_html_object_element_get_type(),
    }
}

pub const NONE_DOMHTML_OBJECT_ELEMENT: Option<&DOMHTMLObjectElement> = None;

pub trait DOMHTMLObjectElementExt: 'static {
    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_align(&self) -> Option<GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_archive(&self) -> Option<GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_border(&self) -> Option<GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_code(&self) -> Option<GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_code_base(&self) -> Option<GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_code_type(&self) -> Option<GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_content_document(&self) -> Option<DOMDocument>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_data(&self) -> Option<GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_declare(&self) -> bool;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_form(&self) -> Option<DOMHTMLFormElement>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_height(&self) -> Option<GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_hspace(&self) -> libc::c_long;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_name(&self) -> Option<GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_standby(&self) -> Option<GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_type_attr(&self) -> Option<GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_use_map(&self) -> Option<GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_vspace(&self) -> libc::c_long;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_width(&self) -> Option<GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_align(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_archive(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_border(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_code(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_code_base(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_code_type(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_data(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_declare(&self, value: bool);

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_height(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_hspace(&self, value: libc::c_long);

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_name(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_standby(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_type_attr(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_use_map(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_vspace(&self, value: libc::c_long);

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_width(&self, value: &str);

    fn get_property_type(&self) -> Option<GString>;

    fn set_property_type<'a, P: Into<Option<&'a str>>>(&self, type_: P);

    fn connect_property_align_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_archive_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_border_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_code_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_code_base_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_code_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_content_document_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_data_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_declare_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_form_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_hspace_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_standby_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_use_map_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_vspace_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMHTMLObjectElement>> DOMHTMLObjectElementExt for O {
    fn get_align(&self) -> Option<GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_object_element_get_align(self.as_ref().to_glib_none().0))
        }
    }

    fn get_archive(&self) -> Option<GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_object_element_get_archive(self.as_ref().to_glib_none().0))
        }
    }

    fn get_border(&self) -> Option<GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_object_element_get_border(self.as_ref().to_glib_none().0))
        }
    }

    fn get_code(&self) -> Option<GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_object_element_get_code(self.as_ref().to_glib_none().0))
        }
    }

    fn get_code_base(&self) -> Option<GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_object_element_get_code_base(self.as_ref().to_glib_none().0))
        }
    }

    fn get_code_type(&self) -> Option<GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_object_element_get_code_type(self.as_ref().to_glib_none().0))
        }
    }

    fn get_content_document(&self) -> Option<DOMDocument> {
        unsafe {
            from_glib_none(ffi::webkit_dom_html_object_element_get_content_document(self.as_ref().to_glib_none().0))
        }
    }

    fn get_data(&self) -> Option<GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_object_element_get_data(self.as_ref().to_glib_none().0))
        }
    }

    fn get_declare(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_html_object_element_get_declare(self.as_ref().to_glib_none().0))
        }
    }

    fn get_form(&self) -> Option<DOMHTMLFormElement> {
        unsafe {
            from_glib_none(ffi::webkit_dom_html_object_element_get_form(self.as_ref().to_glib_none().0))
        }
    }

    fn get_height(&self) -> Option<GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_object_element_get_height(self.as_ref().to_glib_none().0))
        }
    }

    fn get_hspace(&self) -> libc::c_long {
        unsafe {
            ffi::webkit_dom_html_object_element_get_hspace(self.as_ref().to_glib_none().0)
        }
    }

    fn get_name(&self) -> Option<GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_object_element_get_name(self.as_ref().to_glib_none().0))
        }
    }

    fn get_standby(&self) -> Option<GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_object_element_get_standby(self.as_ref().to_glib_none().0))
        }
    }

    fn get_type_attr(&self) -> Option<GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_object_element_get_type_attr(self.as_ref().to_glib_none().0))
        }
    }

    fn get_use_map(&self) -> Option<GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_object_element_get_use_map(self.as_ref().to_glib_none().0))
        }
    }

    fn get_vspace(&self) -> libc::c_long {
        unsafe {
            ffi::webkit_dom_html_object_element_get_vspace(self.as_ref().to_glib_none().0)
        }
    }

    fn get_width(&self) -> Option<GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_object_element_get_width(self.as_ref().to_glib_none().0))
        }
    }

    fn set_align(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_object_element_set_align(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_archive(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_object_element_set_archive(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_border(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_object_element_set_border(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_code(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_object_element_set_code(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_code_base(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_object_element_set_code_base(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_code_type(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_object_element_set_code_type(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_data(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_object_element_set_data(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_declare(&self, value: bool) {
        unsafe {
            ffi::webkit_dom_html_object_element_set_declare(self.as_ref().to_glib_none().0, value.to_glib());
        }
    }

    fn set_height(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_object_element_set_height(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_hspace(&self, value: libc::c_long) {
        unsafe {
            ffi::webkit_dom_html_object_element_set_hspace(self.as_ref().to_glib_none().0, value);
        }
    }

    fn set_name(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_object_element_set_name(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_standby(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_object_element_set_standby(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_type_attr(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_object_element_set_type_attr(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_use_map(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_object_element_set_use_map(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_vspace(&self, value: libc::c_long) {
        unsafe {
            ffi::webkit_dom_html_object_element_set_vspace(self.as_ref().to_glib_none().0, value);
        }
    }

    fn set_width(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_object_element_set_width(self.as_ref().to_glib_none().0, value.to_glib_none().0);
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

    fn connect_property_archive_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::archive\0".as_ptr() as *const _,
                transmute(notify_archive_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_border_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::border\0".as_ptr() as *const _,
                transmute(notify_border_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_code_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::code\0".as_ptr() as *const _,
                transmute(notify_code_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_code_base_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::code-base\0".as_ptr() as *const _,
                transmute(notify_code_base_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_code_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::code-type\0".as_ptr() as *const _,
                transmute(notify_code_type_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_content_document_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::content-document\0".as_ptr() as *const _,
                transmute(notify_content_document_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_data_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::data\0".as_ptr() as *const _,
                transmute(notify_data_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_declare_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::declare\0".as_ptr() as *const _,
                transmute(notify_declare_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_form_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::form\0".as_ptr() as *const _,
                transmute(notify_form_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::height\0".as_ptr() as *const _,
                transmute(notify_height_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_hspace_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::hspace\0".as_ptr() as *const _,
                transmute(notify_hspace_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::name\0".as_ptr() as *const _,
                transmute(notify_name_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_standby_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::standby\0".as_ptr() as *const _,
                transmute(notify_standby_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::type\0".as_ptr() as *const _,
                transmute(notify_type_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_use_map_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::use-map\0".as_ptr() as *const _,
                transmute(notify_use_map_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_vspace_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::vspace\0".as_ptr() as *const _,
                transmute(notify_vspace_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
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

unsafe extern "C" fn notify_align_trampoline<P>(this: *mut ffi::WebKitDOMHTMLObjectElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLObjectElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLObjectElement::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_archive_trampoline<P>(this: *mut ffi::WebKitDOMHTMLObjectElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLObjectElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLObjectElement::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_border_trampoline<P>(this: *mut ffi::WebKitDOMHTMLObjectElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLObjectElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLObjectElement::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_code_trampoline<P>(this: *mut ffi::WebKitDOMHTMLObjectElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLObjectElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLObjectElement::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_code_base_trampoline<P>(this: *mut ffi::WebKitDOMHTMLObjectElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLObjectElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLObjectElement::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_code_type_trampoline<P>(this: *mut ffi::WebKitDOMHTMLObjectElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLObjectElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLObjectElement::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_content_document_trampoline<P>(this: *mut ffi::WebKitDOMHTMLObjectElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLObjectElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLObjectElement::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_data_trampoline<P>(this: *mut ffi::WebKitDOMHTMLObjectElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLObjectElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLObjectElement::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_declare_trampoline<P>(this: *mut ffi::WebKitDOMHTMLObjectElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLObjectElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLObjectElement::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_form_trampoline<P>(this: *mut ffi::WebKitDOMHTMLObjectElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLObjectElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLObjectElement::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_height_trampoline<P>(this: *mut ffi::WebKitDOMHTMLObjectElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLObjectElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLObjectElement::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_hspace_trampoline<P>(this: *mut ffi::WebKitDOMHTMLObjectElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLObjectElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLObjectElement::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_name_trampoline<P>(this: *mut ffi::WebKitDOMHTMLObjectElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLObjectElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLObjectElement::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_standby_trampoline<P>(this: *mut ffi::WebKitDOMHTMLObjectElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLObjectElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLObjectElement::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_type_trampoline<P>(this: *mut ffi::WebKitDOMHTMLObjectElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLObjectElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLObjectElement::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_use_map_trampoline<P>(this: *mut ffi::WebKitDOMHTMLObjectElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLObjectElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLObjectElement::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_vspace_trampoline<P>(this: *mut ffi::WebKitDOMHTMLObjectElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLObjectElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLObjectElement::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_width_trampoline<P>(this: *mut ffi::WebKitDOMHTMLObjectElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLObjectElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLObjectElement::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for DOMHTMLObjectElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DOMHTMLObjectElement")
    }
}
