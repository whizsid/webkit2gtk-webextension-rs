// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use DOMFile;
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
    pub struct DOMFileList(Object<ffi::WebKitDOMFileList, ffi::WebKitDOMFileListClass>): DOMObject;

    match fn {
        get_type => || ffi::webkit_dom_file_list_get_type(),
    }
}

pub trait DOMFileListExt {
    fn get_length(&self) -> libc::c_ulong;

    fn item(&self, index: libc::c_ulong) -> Option<DOMFile>;

    fn connect_property_length_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMFileList> + IsA<glib::object::Object>> DOMFileListExt for O {
    fn get_length(&self) -> libc::c_ulong {
        unsafe {
            ffi::webkit_dom_file_list_get_length(self.to_glib_none().0)
        }
    }

    fn item(&self, index: libc::c_ulong) -> Option<DOMFile> {
        unsafe {
            from_glib_full(ffi::webkit_dom_file_list_item(self.to_glib_none().0, index))
        }
    }

    fn connect_property_length_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::length",
                transmute(notify_length_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_length_trampoline<P>(this: *mut ffi::WebKitDOMFileList, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMFileList> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMFileList::from_glib_borrow(this).downcast_unchecked())
}
