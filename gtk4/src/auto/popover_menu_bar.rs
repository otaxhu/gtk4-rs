// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{
    ffi, Accessible, AccessibleRole, Align, Buildable, ConstraintTarget, LayoutManager, Overflow,
    Widget,
};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GtkPopoverMenuBar")]
    pub struct PopoverMenuBar(Object<ffi::GtkPopoverMenuBar>) @extends Widget, @implements Accessible, Buildable, ConstraintTarget;

    match fn {
        type_ => || ffi::gtk_popover_menu_bar_get_type(),
    }
}

impl PopoverMenuBar {
    #[doc(alias = "gtk_popover_menu_bar_new_from_model")]
    #[doc(alias = "new_from_model")]
    pub fn from_model(model: Option<&impl IsA<gio::MenuModel>>) -> PopoverMenuBar {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_popover_menu_bar_new_from_model(
                model.map(|p| p.as_ref()).to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`PopoverMenuBar`] objects.
    ///
    /// This method returns an instance of [`PopoverMenuBarBuilder`](crate::builders::PopoverMenuBarBuilder) which can be used to create [`PopoverMenuBar`] objects.
    pub fn builder() -> PopoverMenuBarBuilder {
        PopoverMenuBarBuilder::new()
    }

    #[doc(alias = "gtk_popover_menu_bar_add_child")]
    pub fn add_child(&self, child: &impl IsA<Widget>, id: &str) -> bool {
        unsafe {
            from_glib(ffi::gtk_popover_menu_bar_add_child(
                self.to_glib_none().0,
                child.as_ref().to_glib_none().0,
                id.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_popover_menu_bar_get_menu_model")]
    #[doc(alias = "get_menu_model")]
    #[doc(alias = "menu-model")]
    pub fn menu_model(&self) -> Option<gio::MenuModel> {
        unsafe {
            from_glib_none(ffi::gtk_popover_menu_bar_get_menu_model(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_popover_menu_bar_remove_child")]
    pub fn remove_child(&self, child: &impl IsA<Widget>) -> bool {
        unsafe {
            from_glib(ffi::gtk_popover_menu_bar_remove_child(
                self.to_glib_none().0,
                child.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_popover_menu_bar_set_menu_model")]
    #[doc(alias = "menu-model")]
    pub fn set_menu_model(&self, model: Option<&impl IsA<gio::MenuModel>>) {
        unsafe {
            ffi::gtk_popover_menu_bar_set_menu_model(
                self.to_glib_none().0,
                model.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "menu-model")]
    pub fn connect_menu_model_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_menu_model_trampoline<F: Fn(&PopoverMenuBar) + 'static>(
            this: *mut ffi::GtkPopoverMenuBar,
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
                c"notify::menu-model".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_menu_model_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`PopoverMenuBar`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct PopoverMenuBarBuilder {
    builder: glib::object::ObjectBuilder<'static, PopoverMenuBar>,
}

impl PopoverMenuBarBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn menu_model(self, menu_model: &impl IsA<gio::MenuModel>) -> Self {
        Self {
            builder: self
                .builder
                .property("menu-model", menu_model.clone().upcast()),
        }
    }

    pub fn can_focus(self, can_focus: bool) -> Self {
        Self {
            builder: self.builder.property("can-focus", can_focus),
        }
    }

    pub fn can_target(self, can_target: bool) -> Self {
        Self {
            builder: self.builder.property("can-target", can_target),
        }
    }

    pub fn css_classes(self, css_classes: impl Into<glib::StrV>) -> Self {
        Self {
            builder: self.builder.property("css-classes", css_classes.into()),
        }
    }

    pub fn css_name(self, css_name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("css-name", css_name.into()),
        }
    }

    pub fn cursor(self, cursor: &gdk::Cursor) -> Self {
        Self {
            builder: self.builder.property("cursor", cursor.clone()),
        }
    }

    pub fn focus_on_click(self, focus_on_click: bool) -> Self {
        Self {
            builder: self.builder.property("focus-on-click", focus_on_click),
        }
    }

    pub fn focusable(self, focusable: bool) -> Self {
        Self {
            builder: self.builder.property("focusable", focusable),
        }
    }

    pub fn halign(self, halign: Align) -> Self {
        Self {
            builder: self.builder.property("halign", halign),
        }
    }

    pub fn has_tooltip(self, has_tooltip: bool) -> Self {
        Self {
            builder: self.builder.property("has-tooltip", has_tooltip),
        }
    }

    pub fn height_request(self, height_request: i32) -> Self {
        Self {
            builder: self.builder.property("height-request", height_request),
        }
    }

    pub fn hexpand(self, hexpand: bool) -> Self {
        Self {
            builder: self.builder.property("hexpand", hexpand),
        }
    }

    pub fn hexpand_set(self, hexpand_set: bool) -> Self {
        Self {
            builder: self.builder.property("hexpand-set", hexpand_set),
        }
    }

    pub fn layout_manager(self, layout_manager: &impl IsA<LayoutManager>) -> Self {
        Self {
            builder: self
                .builder
                .property("layout-manager", layout_manager.clone().upcast()),
        }
    }

    #[cfg(feature = "v4_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_18")))]
    pub fn limit_events(self, limit_events: bool) -> Self {
        Self {
            builder: self.builder.property("limit-events", limit_events),
        }
    }

    pub fn margin_bottom(self, margin_bottom: i32) -> Self {
        Self {
            builder: self.builder.property("margin-bottom", margin_bottom),
        }
    }

    pub fn margin_end(self, margin_end: i32) -> Self {
        Self {
            builder: self.builder.property("margin-end", margin_end),
        }
    }

    pub fn margin_start(self, margin_start: i32) -> Self {
        Self {
            builder: self.builder.property("margin-start", margin_start),
        }
    }

    pub fn margin_top(self, margin_top: i32) -> Self {
        Self {
            builder: self.builder.property("margin-top", margin_top),
        }
    }

    pub fn name(self, name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("name", name.into()),
        }
    }

    pub fn opacity(self, opacity: f64) -> Self {
        Self {
            builder: self.builder.property("opacity", opacity),
        }
    }

    pub fn overflow(self, overflow: Overflow) -> Self {
        Self {
            builder: self.builder.property("overflow", overflow),
        }
    }

    pub fn receives_default(self, receives_default: bool) -> Self {
        Self {
            builder: self.builder.property("receives-default", receives_default),
        }
    }

    pub fn sensitive(self, sensitive: bool) -> Self {
        Self {
            builder: self.builder.property("sensitive", sensitive),
        }
    }

    pub fn tooltip_markup(self, tooltip_markup: impl Into<glib::GString>) -> Self {
        Self {
            builder: self
                .builder
                .property("tooltip-markup", tooltip_markup.into()),
        }
    }

    pub fn tooltip_text(self, tooltip_text: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("tooltip-text", tooltip_text.into()),
        }
    }

    pub fn valign(self, valign: Align) -> Self {
        Self {
            builder: self.builder.property("valign", valign),
        }
    }

    pub fn vexpand(self, vexpand: bool) -> Self {
        Self {
            builder: self.builder.property("vexpand", vexpand),
        }
    }

    pub fn vexpand_set(self, vexpand_set: bool) -> Self {
        Self {
            builder: self.builder.property("vexpand-set", vexpand_set),
        }
    }

    pub fn visible(self, visible: bool) -> Self {
        Self {
            builder: self.builder.property("visible", visible),
        }
    }

    pub fn width_request(self, width_request: i32) -> Self {
        Self {
            builder: self.builder.property("width-request", width_request),
        }
    }

    pub fn accessible_role(self, accessible_role: AccessibleRole) -> Self {
        Self {
            builder: self.builder.property("accessible-role", accessible_role),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`PopoverMenuBar`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> PopoverMenuBar {
        assert_initialized_main_thread!();
        self.builder.build()
    }
}
