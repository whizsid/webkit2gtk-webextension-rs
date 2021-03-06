// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(any(feature = "v2_18", feature = "dox"))]
use gio;
#[cfg(any(feature = "v2_18", feature = "dox"))]
use glib;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;
use webkit2_webextension_sys;
use ContextMenu;
use ContextMenuAction;

glib_wrapper! {
    pub struct ContextMenuItem(Object<webkit2_webextension_sys::WebKitContextMenuItem, webkit2_webextension_sys::WebKitContextMenuItemClass, ContextMenuItemClass>);

    match fn {
        get_type => || webkit2_webextension_sys::webkit_context_menu_item_get_type(),
    }
}

impl ContextMenuItem {
    //#[cfg_attr(feature = "v2_18", deprecated)]
    //pub fn new(action: /*Ignored*/&gtk::Action) -> ContextMenuItem {
    //    unsafe { TODO: call webkit2_webextension_sys:webkit_context_menu_item_new() }
    //}

    #[cfg(any(feature = "v2_18", feature = "dox"))]
    pub fn new_from_gaction<P: IsA<gio::Action>>(
        action: &P,
        label: &str,
        target: Option<&glib::Variant>,
    ) -> ContextMenuItem {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(
                webkit2_webextension_sys::webkit_context_menu_item_new_from_gaction(
                    action.as_ref().to_glib_none().0,
                    label.to_glib_none().0,
                    target.to_glib_none().0,
                ),
            )
        }
    }

    pub fn new_from_stock_action(action: ContextMenuAction) -> ContextMenuItem {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(
                webkit2_webextension_sys::webkit_context_menu_item_new_from_stock_action(
                    action.to_glib(),
                ),
            )
        }
    }

    pub fn new_from_stock_action_with_label(
        action: ContextMenuAction,
        label: &str,
    ) -> ContextMenuItem {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(
                webkit2_webextension_sys::webkit_context_menu_item_new_from_stock_action_with_label(
                    action.to_glib(),
                    label.to_glib_none().0,
                ),
            )
        }
    }

    pub fn new_separator() -> ContextMenuItem {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(webkit2_webextension_sys::webkit_context_menu_item_new_separator())
        }
    }

    pub fn new_with_submenu<P: IsA<ContextMenu>>(label: &str, submenu: &P) -> ContextMenuItem {
        skip_assert_initialized!();
        unsafe {
            from_glib_none(
                webkit2_webextension_sys::webkit_context_menu_item_new_with_submenu(
                    label.to_glib_none().0,
                    submenu.as_ref().to_glib_none().0,
                ),
            )
        }
    }
}

pub const NONE_CONTEXT_MENU_ITEM: Option<&ContextMenuItem> = None;

pub trait ContextMenuItemExt: 'static {
    //#[cfg_attr(feature = "v2_18", deprecated)]
    //fn get_action(&self) -> /*Ignored*/Option<gtk::Action>;

    #[cfg(any(feature = "v2_18", feature = "dox"))]
    fn get_gaction(&self) -> Option<gio::Action>;

    fn get_stock_action(&self) -> ContextMenuAction;

    fn get_submenu(&self) -> Option<ContextMenu>;

    fn is_separator(&self) -> bool;

    fn set_submenu<P: IsA<ContextMenu>>(&self, submenu: Option<&P>);
}

impl<O: IsA<ContextMenuItem>> ContextMenuItemExt for O {
    //fn get_action(&self) -> /*Ignored*/Option<gtk::Action> {
    //    unsafe { TODO: call webkit2_webextension_sys:webkit_context_menu_item_get_action() }
    //}

    #[cfg(any(feature = "v2_18", feature = "dox"))]
    fn get_gaction(&self) -> Option<gio::Action> {
        unsafe {
            from_glib_none(
                webkit2_webextension_sys::webkit_context_menu_item_get_gaction(
                    self.as_ref().to_glib_none().0,
                ),
            )
        }
    }

    fn get_stock_action(&self) -> ContextMenuAction {
        unsafe {
            from_glib(
                webkit2_webextension_sys::webkit_context_menu_item_get_stock_action(
                    self.as_ref().to_glib_none().0,
                ),
            )
        }
    }

    fn get_submenu(&self) -> Option<ContextMenu> {
        unsafe {
            from_glib_none(
                webkit2_webextension_sys::webkit_context_menu_item_get_submenu(
                    self.as_ref().to_glib_none().0,
                ),
            )
        }
    }

    fn is_separator(&self) -> bool {
        unsafe {
            from_glib(
                webkit2_webextension_sys::webkit_context_menu_item_is_separator(
                    self.as_ref().to_glib_none().0,
                ),
            )
        }
    }

    fn set_submenu<P: IsA<ContextMenu>>(&self, submenu: Option<&P>) {
        unsafe {
            webkit2_webextension_sys::webkit_context_menu_item_set_submenu(
                self.as_ref().to_glib_none().0,
                submenu.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }
}

impl fmt::Display for ContextMenuItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ContextMenuItem")
    }
}
