// This file was generated by gir (https://github.com/gtk-rs/gir @ f5fca82)
// from gir-files (https://github.com/gtk-rs/gir-files @ b8f5ef1)
// DO NOT EDIT

use DOMElement;
use DOMEventTarget;
use DOMHTMLElement;
use DOMNode;
use DOMObject;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct DOMHTMLMarqueeElement(Object<ffi::WebKitDOMHTMLMarqueeElement, ffi::WebKitDOMHTMLMarqueeElementClass>): DOMHTMLElement, DOMElement, DOMNode, DOMObject, DOMEventTarget;

    match fn {
        get_type => || ffi::webkit_dom_html_marquee_element_get_type(),
    }
}

pub trait DOMHTMLMarqueeElementExt {
    fn start(&self);

    fn stop(&self);
}

impl<O: IsA<DOMHTMLMarqueeElement>> DOMHTMLMarqueeElementExt for O {
    fn start(&self) {
        unsafe {
            ffi::webkit_dom_html_marquee_element_start(self.to_glib_none().0);
        }
    }

    fn stop(&self) {
        unsafe {
            ffi::webkit_dom_html_marquee_element_stop(self.to_glib_none().0);
        }
    }
}
