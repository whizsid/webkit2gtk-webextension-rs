// This file was generated by gir (https://github.com/gtk-rs/gir @ f5fca82)
// from gir-files (https://github.com/gtk-rs/gir-files @ b8f5ef1)
// DO NOT EDIT

use ffi;
use glib;
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
    pub struct URIRequest(Object<ffi::WebKitURIRequest, ffi::WebKitURIRequestClass>);

    match fn {
        get_type => || ffi::webkit_uri_request_get_type(),
    }
}

impl URIRequest {
    pub fn new(uri: &str) -> URIRequest {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::webkit_uri_request_new(uri.to_glib_none().0))
        }
    }
}

pub trait URIRequestExt {
    //fn get_http_headers(&self) -> /*Ignored*/Option<soup::MessageHeaders>;

    #[cfg(any(feature = "v2_12", feature = "dox"))]
    fn get_http_method(&self) -> Option<String>;

    fn get_uri(&self) -> Option<String>;

    fn set_uri(&self, uri: &str);

    fn connect_property_uri_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<URIRequest> + IsA<glib::object::Object>> URIRequestExt for O {
    //fn get_http_headers(&self) -> /*Ignored*/Option<soup::MessageHeaders> {
    //    unsafe { TODO: call ffi::webkit_uri_request_get_http_headers() }
    //}

    #[cfg(any(feature = "v2_12", feature = "dox"))]
    fn get_http_method(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::webkit_uri_request_get_http_method(self.to_glib_none().0))
        }
    }

    fn get_uri(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::webkit_uri_request_get_uri(self.to_glib_none().0))
        }
    }

    fn set_uri(&self, uri: &str) {
        unsafe {
            ffi::webkit_uri_request_set_uri(self.to_glib_none().0, uri.to_glib_none().0);
        }
    }

    fn connect_property_uri_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::uri",
                transmute(notify_uri_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_uri_trampoline<P>(this: *mut ffi::WebKitURIRequest, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<URIRequest> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&URIRequest::from_glib_borrow(this).downcast_unchecked())
}
