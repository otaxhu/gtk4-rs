// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT
#![allow(deprecated)]

use crate::{ffi, CellRenderer, CellRendererMode, IconSize};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GtkCellRendererPixbuf")]
    pub struct CellRendererPixbuf(Object<ffi::GtkCellRendererPixbuf>) @extends CellRenderer;

    match fn {
        type_ => || ffi::gtk_cell_renderer_pixbuf_get_type(),
    }
}

impl CellRendererPixbuf {
    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_cell_renderer_pixbuf_new")]
    pub fn new() -> CellRendererPixbuf {
        assert_initialized_main_thread!();
        unsafe { CellRenderer::from_glib_none(ffi::gtk_cell_renderer_pixbuf_new()).unsafe_cast() }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`CellRendererPixbuf`] objects.
    ///
    /// This method returns an instance of [`CellRendererPixbufBuilder`](crate::builders::CellRendererPixbufBuilder) which can be used to create [`CellRendererPixbuf`] objects.
    pub fn builder() -> CellRendererPixbufBuilder {
        CellRendererPixbufBuilder::new()
    }

    pub fn gicon(&self) -> Option<gio::Icon> {
        ObjectExt::property(self, "gicon")
    }

    pub fn set_gicon<P: IsA<gio::Icon>>(&self, gicon: Option<&P>) {
        ObjectExt::set_property(self, "gicon", gicon)
    }

    #[doc(alias = "icon-name")]
    pub fn icon_name(&self) -> Option<glib::GString> {
        ObjectExt::property(self, "icon-name")
    }

    #[doc(alias = "icon-name")]
    pub fn set_icon_name(&self, icon_name: Option<&str>) {
        ObjectExt::set_property(self, "icon-name", icon_name)
    }

    #[doc(alias = "icon-size")]
    pub fn icon_size(&self) -> IconSize {
        ObjectExt::property(self, "icon-size")
    }

    #[doc(alias = "icon-size")]
    pub fn set_icon_size(&self, icon_size: IconSize) {
        ObjectExt::set_property(self, "icon-size", icon_size)
    }

    pub fn set_pixbuf(&self, pixbuf: Option<&gdk_pixbuf::Pixbuf>) {
        ObjectExt::set_property(self, "pixbuf", pixbuf)
    }

    #[doc(alias = "pixbuf-expander-closed")]
    pub fn pixbuf_expander_closed(&self) -> Option<gdk_pixbuf::Pixbuf> {
        ObjectExt::property(self, "pixbuf-expander-closed")
    }

    #[doc(alias = "pixbuf-expander-closed")]
    pub fn set_pixbuf_expander_closed(&self, pixbuf_expander_closed: Option<&gdk_pixbuf::Pixbuf>) {
        ObjectExt::set_property(self, "pixbuf-expander-closed", pixbuf_expander_closed)
    }

    #[doc(alias = "pixbuf-expander-open")]
    pub fn pixbuf_expander_open(&self) -> Option<gdk_pixbuf::Pixbuf> {
        ObjectExt::property(self, "pixbuf-expander-open")
    }

    #[doc(alias = "pixbuf-expander-open")]
    pub fn set_pixbuf_expander_open(&self, pixbuf_expander_open: Option<&gdk_pixbuf::Pixbuf>) {
        ObjectExt::set_property(self, "pixbuf-expander-open", pixbuf_expander_open)
    }

    pub fn texture(&self) -> Option<gdk::Texture> {
        ObjectExt::property(self, "texture")
    }

    pub fn set_texture<P: IsA<gdk::Texture>>(&self, texture: Option<&P>) {
        ObjectExt::set_property(self, "texture", texture)
    }

    #[doc(alias = "gicon")]
    pub fn connect_gicon_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_gicon_trampoline<F: Fn(&CellRendererPixbuf) + 'static>(
            this: *mut ffi::GtkCellRendererPixbuf,
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
                c"notify::gicon".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_gicon_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "icon-name")]
    pub fn connect_icon_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_icon_name_trampoline<F: Fn(&CellRendererPixbuf) + 'static>(
            this: *mut ffi::GtkCellRendererPixbuf,
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
                c"notify::icon-name".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_icon_name_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "icon-size")]
    pub fn connect_icon_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_icon_size_trampoline<F: Fn(&CellRendererPixbuf) + 'static>(
            this: *mut ffi::GtkCellRendererPixbuf,
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
                c"notify::icon-size".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_icon_size_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "pixbuf")]
    pub fn connect_pixbuf_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_pixbuf_trampoline<F: Fn(&CellRendererPixbuf) + 'static>(
            this: *mut ffi::GtkCellRendererPixbuf,
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
                c"notify::pixbuf".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_pixbuf_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "pixbuf-expander-closed")]
    pub fn connect_pixbuf_expander_closed_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_pixbuf_expander_closed_trampoline<
            F: Fn(&CellRendererPixbuf) + 'static,
        >(
            this: *mut ffi::GtkCellRendererPixbuf,
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
                c"notify::pixbuf-expander-closed".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_pixbuf_expander_closed_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "pixbuf-expander-open")]
    pub fn connect_pixbuf_expander_open_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_pixbuf_expander_open_trampoline<
            F: Fn(&CellRendererPixbuf) + 'static,
        >(
            this: *mut ffi::GtkCellRendererPixbuf,
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
                c"notify::pixbuf-expander-open".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_pixbuf_expander_open_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "texture")]
    pub fn connect_texture_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_texture_trampoline<F: Fn(&CellRendererPixbuf) + 'static>(
            this: *mut ffi::GtkCellRendererPixbuf,
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
                c"notify::texture".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_texture_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for CellRendererPixbuf {
    fn default() -> Self {
        Self::new()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`CellRendererPixbuf`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct CellRendererPixbufBuilder {
    builder: glib::object::ObjectBuilder<'static, CellRendererPixbuf>,
}

impl CellRendererPixbufBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn gicon(self, gicon: &impl IsA<gio::Icon>) -> Self {
        Self {
            builder: self.builder.property("gicon", gicon.clone().upcast()),
        }
    }

    pub fn icon_name(self, icon_name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("icon-name", icon_name.into()),
        }
    }

    pub fn icon_size(self, icon_size: IconSize) -> Self {
        Self {
            builder: self.builder.property("icon-size", icon_size),
        }
    }

    pub fn pixbuf(self, pixbuf: &gdk_pixbuf::Pixbuf) -> Self {
        Self {
            builder: self.builder.property("pixbuf", pixbuf.clone()),
        }
    }

    pub fn pixbuf_expander_closed(self, pixbuf_expander_closed: &gdk_pixbuf::Pixbuf) -> Self {
        Self {
            builder: self
                .builder
                .property("pixbuf-expander-closed", pixbuf_expander_closed.clone()),
        }
    }

    pub fn pixbuf_expander_open(self, pixbuf_expander_open: &gdk_pixbuf::Pixbuf) -> Self {
        Self {
            builder: self
                .builder
                .property("pixbuf-expander-open", pixbuf_expander_open.clone()),
        }
    }

    pub fn texture(self, texture: &impl IsA<gdk::Texture>) -> Self {
        Self {
            builder: self.builder.property("texture", texture.clone().upcast()),
        }
    }

    pub fn cell_background(self, cell_background: impl Into<glib::GString>) -> Self {
        Self {
            builder: self
                .builder
                .property("cell-background", cell_background.into()),
        }
    }

    pub fn cell_background_rgba(self, cell_background_rgba: &gdk::RGBA) -> Self {
        Self {
            builder: self
                .builder
                .property("cell-background-rgba", cell_background_rgba),
        }
    }

    pub fn cell_background_set(self, cell_background_set: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("cell-background-set", cell_background_set),
        }
    }

    pub fn height(self, height: i32) -> Self {
        Self {
            builder: self.builder.property("height", height),
        }
    }

    pub fn is_expanded(self, is_expanded: bool) -> Self {
        Self {
            builder: self.builder.property("is-expanded", is_expanded),
        }
    }

    pub fn is_expander(self, is_expander: bool) -> Self {
        Self {
            builder: self.builder.property("is-expander", is_expander),
        }
    }

    pub fn mode(self, mode: CellRendererMode) -> Self {
        Self {
            builder: self.builder.property("mode", mode),
        }
    }

    pub fn sensitive(self, sensitive: bool) -> Self {
        Self {
            builder: self.builder.property("sensitive", sensitive),
        }
    }

    pub fn visible(self, visible: bool) -> Self {
        Self {
            builder: self.builder.property("visible", visible),
        }
    }

    pub fn width(self, width: i32) -> Self {
        Self {
            builder: self.builder.property("width", width),
        }
    }

    pub fn xalign(self, xalign: f32) -> Self {
        Self {
            builder: self.builder.property("xalign", xalign),
        }
    }

    pub fn xpad(self, xpad: u32) -> Self {
        Self {
            builder: self.builder.property("xpad", xpad),
        }
    }

    pub fn yalign(self, yalign: f32) -> Self {
        Self {
            builder: self.builder.property("yalign", yalign),
        }
    }

    pub fn ypad(self, ypad: u32) -> Self {
        Self {
            builder: self.builder.property("ypad", ypad),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`CellRendererPixbuf`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> CellRendererPixbuf {
        assert_initialized_main_thread!();
        self.builder.build()
    }
}
