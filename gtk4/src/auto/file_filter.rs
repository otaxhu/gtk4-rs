// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT
#![allow(deprecated)]

use crate::{ffi, Buildable, Filter};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GtkFileFilter")]
    pub struct FileFilter(Object<ffi::GtkFileFilter>) @extends Filter, @implements Buildable;

    match fn {
        type_ => || ffi::gtk_file_filter_get_type(),
    }
}

impl FileFilter {
    #[doc(alias = "gtk_file_filter_new")]
    pub fn new() -> FileFilter {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gtk_file_filter_new()) }
    }

    #[doc(alias = "gtk_file_filter_new_from_gvariant")]
    #[doc(alias = "new_from_gvariant")]
    pub fn from_gvariant(variant: &glib::Variant) -> FileFilter {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_file_filter_new_from_gvariant(
                variant.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_file_filter_add_mime_type")]
    pub fn add_mime_type(&self, mime_type: &str) {
        unsafe {
            ffi::gtk_file_filter_add_mime_type(self.to_glib_none().0, mime_type.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_file_filter_add_pattern")]
    pub fn add_pattern(&self, pattern: &str) {
        unsafe {
            ffi::gtk_file_filter_add_pattern(self.to_glib_none().0, pattern.to_glib_none().0);
        }
    }

    #[cfg_attr(feature = "v4_20", deprecated = "Since 4.20")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_file_filter_add_pixbuf_formats")]
    pub fn add_pixbuf_formats(&self) {
        unsafe {
            ffi::gtk_file_filter_add_pixbuf_formats(self.to_glib_none().0);
        }
    }

    #[cfg(feature = "v4_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_4")))]
    #[doc(alias = "gtk_file_filter_add_suffix")]
    pub fn add_suffix(&self, suffix: &str) {
        unsafe {
            ffi::gtk_file_filter_add_suffix(self.to_glib_none().0, suffix.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_file_filter_get_attributes")]
    #[doc(alias = "get_attributes")]
    pub fn attributes(&self) -> Vec<glib::GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::gtk_file_filter_get_attributes(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_file_filter_get_name")]
    #[doc(alias = "get_name")]
    pub fn name(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::gtk_file_filter_get_name(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_file_filter_set_name")]
    #[doc(alias = "name")]
    pub fn set_name(&self, name: Option<&str>) {
        unsafe {
            ffi::gtk_file_filter_set_name(self.to_glib_none().0, name.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_file_filter_to_gvariant")]
    pub fn to_gvariant(&self) -> glib::Variant {
        unsafe { from_glib_none(ffi::gtk_file_filter_to_gvariant(self.to_glib_none().0)) }
    }

    #[doc(alias = "name")]
    pub fn connect_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_name_trampoline<F: Fn(&FileFilter) + 'static>(
            this: *mut ffi::GtkFileFilter,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::name".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_name_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for FileFilter {
    fn default() -> Self {
        Self::new()
    }
}
