// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::GString;
use glib_sys;
use libc;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use webkit2_webextension_sys;
use DOMDocument;
use DOMEventTarget;
use DOMHTMLCollection;
use DOMNode;
use DOMObject;

glib_wrapper! {
    pub struct DOMHTMLDocument(Object<webkit2_webextension_sys::WebKitDOMHTMLDocument, webkit2_webextension_sys::WebKitDOMHTMLDocumentClass, DOMHTMLDocumentClass>) @extends DOMDocument, DOMNode, DOMObject, @implements DOMEventTarget;

    match fn {
        get_type => || webkit2_webextension_sys::webkit_dom_html_document_get_type(),
    }
}

pub const NONE_DOMHTML_DOCUMENT: Option<&DOMHTMLDocument> = None;

pub trait DOMHTMLDocumentExt: 'static {
    #[cfg_attr(feature = "v2_22", deprecated)]
    fn capture_events(&self);

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn clear(&self);

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn close(&self);

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_alink_color(&self) -> Option<GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_bg_color(&self) -> Option<GString>;

    #[cfg_attr(feature = "v2_14", deprecated)]
    #[cfg(any(not(feature = "v2_14"), feature = "dox"))]
    fn get_compat_mode(&self) -> Option<GString>;

    #[cfg_attr(feature = "v2_14", deprecated)]
    #[cfg(any(not(feature = "v2_14"), feature = "dox"))]
    fn get_design_mode(&self) -> Option<GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    #[cfg(any(not(feature = "v2_16"), feature = "dox"))]
    fn get_dir(&self) -> Option<GString>;

    #[cfg_attr(feature = "v2_14", deprecated)]
    #[cfg(any(not(feature = "v2_14"), feature = "dox"))]
    fn get_embeds(&self) -> Option<DOMHTMLCollection>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_fg_color(&self) -> Option<GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_height(&self) -> libc::c_long;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_link_color(&self) -> Option<GString>;

    #[cfg_attr(feature = "v2_14", deprecated)]
    #[cfg(any(not(feature = "v2_14"), feature = "dox"))]
    fn get_plugins(&self) -> Option<DOMHTMLCollection>;

    #[cfg_attr(feature = "v2_14", deprecated)]
    #[cfg(any(not(feature = "v2_14"), feature = "dox"))]
    fn get_scripts(&self) -> Option<DOMHTMLCollection>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_vlink_color(&self) -> Option<GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_width(&self) -> libc::c_long;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn release_events(&self);

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_alink_color(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_bg_color(&self, value: &str);

    #[cfg_attr(feature = "v2_14", deprecated)]
    #[cfg(any(not(feature = "v2_14"), feature = "dox"))]
    fn set_design_mode(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated)]
    #[cfg(any(not(feature = "v2_16"), feature = "dox"))]
    fn set_dir(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_fg_color(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_link_color(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_vlink_color(&self, value: &str);

    fn connect_property_alink_color_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_bg_color_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_dir_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_fg_color_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_link_color_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_vlink_color_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMHTMLDocument>> DOMHTMLDocumentExt for O {
    fn capture_events(&self) {
        unsafe {
            webkit2_webextension_sys::webkit_dom_html_document_capture_events(self.as_ref().to_glib_none().0);
        }
    }

    fn clear(&self) {
        unsafe {
            webkit2_webextension_sys::webkit_dom_html_document_clear(self.as_ref().to_glib_none().0);
        }
    }

    fn close(&self) {
        unsafe {
            webkit2_webextension_sys::webkit_dom_html_document_close(self.as_ref().to_glib_none().0);
        }
    }

    fn get_alink_color(&self) -> Option<GString> {
        unsafe {
            from_glib_full(webkit2_webextension_sys::webkit_dom_html_document_get_alink_color(self.as_ref().to_glib_none().0))
        }
    }

    fn get_bg_color(&self) -> Option<GString> {
        unsafe {
            from_glib_full(webkit2_webextension_sys::webkit_dom_html_document_get_bg_color(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(not(feature = "v2_14"), feature = "dox"))]
    fn get_compat_mode(&self) -> Option<GString> {
        unsafe {
            from_glib_full(webkit2_webextension_sys::webkit_dom_html_document_get_compat_mode(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(not(feature = "v2_14"), feature = "dox"))]
    fn get_design_mode(&self) -> Option<GString> {
        unsafe {
            from_glib_full(webkit2_webextension_sys::webkit_dom_html_document_get_design_mode(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(not(feature = "v2_16"), feature = "dox"))]
    fn get_dir(&self) -> Option<GString> {
        unsafe {
            from_glib_full(webkit2_webextension_sys::webkit_dom_html_document_get_dir(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(not(feature = "v2_14"), feature = "dox"))]
    fn get_embeds(&self) -> Option<DOMHTMLCollection> {
        unsafe {
            from_glib_full(webkit2_webextension_sys::webkit_dom_html_document_get_embeds(self.as_ref().to_glib_none().0))
        }
    }

    fn get_fg_color(&self) -> Option<GString> {
        unsafe {
            from_glib_full(webkit2_webextension_sys::webkit_dom_html_document_get_fg_color(self.as_ref().to_glib_none().0))
        }
    }

    fn get_height(&self) -> libc::c_long {
        unsafe {
            webkit2_webextension_sys::webkit_dom_html_document_get_height(self.as_ref().to_glib_none().0)
        }
    }

    fn get_link_color(&self) -> Option<GString> {
        unsafe {
            from_glib_full(webkit2_webextension_sys::webkit_dom_html_document_get_link_color(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(not(feature = "v2_14"), feature = "dox"))]
    fn get_plugins(&self) -> Option<DOMHTMLCollection> {
        unsafe {
            from_glib_full(webkit2_webextension_sys::webkit_dom_html_document_get_plugins(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(not(feature = "v2_14"), feature = "dox"))]
    fn get_scripts(&self) -> Option<DOMHTMLCollection> {
        unsafe {
            from_glib_full(webkit2_webextension_sys::webkit_dom_html_document_get_scripts(self.as_ref().to_glib_none().0))
        }
    }

    fn get_vlink_color(&self) -> Option<GString> {
        unsafe {
            from_glib_full(webkit2_webextension_sys::webkit_dom_html_document_get_vlink_color(self.as_ref().to_glib_none().0))
        }
    }

    fn get_width(&self) -> libc::c_long {
        unsafe {
            webkit2_webextension_sys::webkit_dom_html_document_get_width(self.as_ref().to_glib_none().0)
        }
    }

    fn release_events(&self) {
        unsafe {
            webkit2_webextension_sys::webkit_dom_html_document_release_events(self.as_ref().to_glib_none().0);
        }
    }

    fn set_alink_color(&self, value: &str) {
        unsafe {
            webkit2_webextension_sys::webkit_dom_html_document_set_alink_color(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_bg_color(&self, value: &str) {
        unsafe {
            webkit2_webextension_sys::webkit_dom_html_document_set_bg_color(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    #[cfg(any(not(feature = "v2_14"), feature = "dox"))]
    fn set_design_mode(&self, value: &str) {
        unsafe {
            webkit2_webextension_sys::webkit_dom_html_document_set_design_mode(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    #[cfg(any(not(feature = "v2_16"), feature = "dox"))]
    fn set_dir(&self, value: &str) {
        unsafe {
            webkit2_webextension_sys::webkit_dom_html_document_set_dir(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_fg_color(&self, value: &str) {
        unsafe {
            webkit2_webextension_sys::webkit_dom_html_document_set_fg_color(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_link_color(&self, value: &str) {
        unsafe {
            webkit2_webextension_sys::webkit_dom_html_document_set_link_color(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_vlink_color(&self, value: &str) {
        unsafe {
            webkit2_webextension_sys::webkit_dom_html_document_set_vlink_color(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn connect_property_alink_color_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_alink_color_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMHTMLDocument, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<DOMHTMLDocument>
        {
            let f: &F = &*(f as *const F);
            f(&DOMHTMLDocument::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::alink-color\0".as_ptr() as *const _,
                Some(transmute(notify_alink_color_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_bg_color_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_bg_color_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMHTMLDocument, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<DOMHTMLDocument>
        {
            let f: &F = &*(f as *const F);
            f(&DOMHTMLDocument::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::bg-color\0".as_ptr() as *const _,
                Some(transmute(notify_bg_color_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_dir_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_dir_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMHTMLDocument, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<DOMHTMLDocument>
        {
            let f: &F = &*(f as *const F);
            f(&DOMHTMLDocument::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::dir\0".as_ptr() as *const _,
                Some(transmute(notify_dir_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_fg_color_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_fg_color_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMHTMLDocument, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<DOMHTMLDocument>
        {
            let f: &F = &*(f as *const F);
            f(&DOMHTMLDocument::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::fg-color\0".as_ptr() as *const _,
                Some(transmute(notify_fg_color_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_height_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMHTMLDocument, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<DOMHTMLDocument>
        {
            let f: &F = &*(f as *const F);
            f(&DOMHTMLDocument::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::height\0".as_ptr() as *const _,
                Some(transmute(notify_height_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_link_color_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_link_color_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMHTMLDocument, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<DOMHTMLDocument>
        {
            let f: &F = &*(f as *const F);
            f(&DOMHTMLDocument::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::link-color\0".as_ptr() as *const _,
                Some(transmute(notify_link_color_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_vlink_color_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_vlink_color_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMHTMLDocument, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<DOMHTMLDocument>
        {
            let f: &F = &*(f as *const F);
            f(&DOMHTMLDocument::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::vlink-color\0".as_ptr() as *const _,
                Some(transmute(notify_vlink_color_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_width_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMHTMLDocument, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<DOMHTMLDocument>
        {
            let f: &F = &*(f as *const F);
            f(&DOMHTMLDocument::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::width\0".as_ptr() as *const _,
                Some(transmute(notify_width_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for DOMHTMLDocument {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DOMHTMLDocument")
    }
}
