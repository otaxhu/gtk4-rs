// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT
#![allow(deprecated)]

use crate::{
    ffi, CairoContext, Cursor, Device, Display, Event, FrameClock, GLContext, ModifierType,
    Monitor, VulkanContext,
};
use glib::{
    object::ObjectType as _,
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GdkSurface")]
    pub struct Surface(Object<ffi::GdkSurface, ffi::GdkSurfaceClass>);

    match fn {
        type_ => || ffi::gdk_surface_get_type(),
    }
}

impl Surface {
    pub const NONE: Option<&'static Surface> = None;

    #[doc(alias = "gdk_surface_new_popup")]
    pub fn new_popup(parent: &impl IsA<Surface>, autohide: bool) -> Surface {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::gdk_surface_new_popup(
                parent.as_ref().to_glib_none().0,
                autohide.into_glib(),
            ))
        }
    }

    #[doc(alias = "gdk_surface_new_toplevel")]
    pub fn new_toplevel(display: &impl IsA<Display>) -> Surface {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::gdk_surface_new_toplevel(
                display.as_ref().to_glib_none().0,
            ))
        }
    }
}

pub trait SurfaceExt: IsA<Surface> + 'static {
    #[doc(alias = "gdk_surface_beep")]
    fn beep(&self) {
        unsafe {
            ffi::gdk_surface_beep(self.as_ref().to_glib_none().0);
        }
    }

    #[cfg_attr(feature = "v4_18", deprecated = "Since 4.18")]
    #[allow(deprecated)]
    #[doc(alias = "gdk_surface_create_cairo_context")]
    fn create_cairo_context(&self) -> CairoContext {
        unsafe {
            from_glib_full(ffi::gdk_surface_create_cairo_context(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_surface_create_gl_context")]
    fn create_gl_context(&self) -> Result<GLContext, glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let ret =
                ffi::gdk_surface_create_gl_context(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[cfg_attr(feature = "v4_14", deprecated = "Since 4.14")]
    #[allow(deprecated)]
    #[doc(alias = "gdk_surface_create_vulkan_context")]
    fn create_vulkan_context(&self) -> Result<VulkanContext, glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let ret =
                ffi::gdk_surface_create_vulkan_context(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "gdk_surface_destroy")]
    fn destroy(&self) {
        unsafe {
            ffi::gdk_surface_destroy(self.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "gdk_surface_get_cursor")]
    #[doc(alias = "get_cursor")]
    fn cursor(&self) -> Option<Cursor> {
        unsafe { from_glib_none(ffi::gdk_surface_get_cursor(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "gdk_surface_get_device_cursor")]
    #[doc(alias = "get_device_cursor")]
    fn device_cursor(&self, device: &impl IsA<Device>) -> Option<Cursor> {
        unsafe {
            from_glib_none(ffi::gdk_surface_get_device_cursor(
                self.as_ref().to_glib_none().0,
                device.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_surface_get_device_position")]
    #[doc(alias = "get_device_position")]
    fn device_position(&self, device: &impl IsA<Device>) -> Option<(f64, f64, ModifierType)> {
        unsafe {
            let mut x = std::mem::MaybeUninit::uninit();
            let mut y = std::mem::MaybeUninit::uninit();
            let mut mask = std::mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gdk_surface_get_device_position(
                self.as_ref().to_glib_none().0,
                device.as_ref().to_glib_none().0,
                x.as_mut_ptr(),
                y.as_mut_ptr(),
                mask.as_mut_ptr(),
            ));
            if ret {
                Some((
                    x.assume_init(),
                    y.assume_init(),
                    from_glib(mask.assume_init()),
                ))
            } else {
                None
            }
        }
    }

    #[doc(alias = "gdk_surface_get_display")]
    #[doc(alias = "get_display")]
    fn display(&self) -> Display {
        unsafe { from_glib_none(ffi::gdk_surface_get_display(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "gdk_surface_get_frame_clock")]
    #[doc(alias = "get_frame_clock")]
    #[doc(alias = "frame-clock")]
    fn frame_clock(&self) -> FrameClock {
        unsafe {
            from_glib_none(ffi::gdk_surface_get_frame_clock(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_surface_get_height")]
    #[doc(alias = "get_height")]
    fn height(&self) -> i32 {
        unsafe { ffi::gdk_surface_get_height(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "gdk_surface_get_mapped")]
    #[doc(alias = "get_mapped")]
    #[doc(alias = "mapped")]
    fn is_mapped(&self) -> bool {
        unsafe { from_glib(ffi::gdk_surface_get_mapped(self.as_ref().to_glib_none().0)) }
    }

    #[cfg(feature = "v4_12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_12")))]
    #[doc(alias = "gdk_surface_get_scale")]
    #[doc(alias = "get_scale")]
    fn scale(&self) -> f64 {
        unsafe { ffi::gdk_surface_get_scale(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "gdk_surface_get_scale_factor")]
    #[doc(alias = "get_scale_factor")]
    #[doc(alias = "scale-factor")]
    fn scale_factor(&self) -> i32 {
        unsafe { ffi::gdk_surface_get_scale_factor(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "gdk_surface_get_width")]
    #[doc(alias = "get_width")]
    fn width(&self) -> i32 {
        unsafe { ffi::gdk_surface_get_width(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "gdk_surface_hide")]
    fn hide(&self) {
        unsafe {
            ffi::gdk_surface_hide(self.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "gdk_surface_is_destroyed")]
    fn is_destroyed(&self) -> bool {
        unsafe {
            from_glib(ffi::gdk_surface_is_destroyed(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_surface_queue_render")]
    fn queue_render(&self) {
        unsafe {
            ffi::gdk_surface_queue_render(self.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "gdk_surface_request_layout")]
    fn request_layout(&self) {
        unsafe {
            ffi::gdk_surface_request_layout(self.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "gdk_surface_set_cursor")]
    #[doc(alias = "cursor")]
    fn set_cursor(&self, cursor: Option<&Cursor>) {
        unsafe {
            ffi::gdk_surface_set_cursor(self.as_ref().to_glib_none().0, cursor.to_glib_none().0);
        }
    }

    #[doc(alias = "gdk_surface_set_device_cursor")]
    fn set_device_cursor(&self, device: &impl IsA<Device>, cursor: &Cursor) {
        unsafe {
            ffi::gdk_surface_set_device_cursor(
                self.as_ref().to_glib_none().0,
                device.as_ref().to_glib_none().0,
                cursor.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gdk_surface_set_input_region")]
    fn set_input_region(&self, region: &cairo::Region) {
        unsafe {
            ffi::gdk_surface_set_input_region(
                self.as_ref().to_glib_none().0,
                mut_override(region.to_glib_none().0),
            );
        }
    }

    #[cfg_attr(feature = "v4_16", deprecated = "Since 4.16")]
    #[allow(deprecated)]
    #[doc(alias = "gdk_surface_set_opaque_region")]
    fn set_opaque_region(&self, region: Option<&cairo::Region>) {
        unsafe {
            ffi::gdk_surface_set_opaque_region(
                self.as_ref().to_glib_none().0,
                mut_override(region.to_glib_none().0),
            );
        }
    }

    #[doc(alias = "enter-monitor")]
    fn connect_enter_monitor<F: Fn(&Self, &Monitor) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn enter_monitor_trampoline<
            P: IsA<Surface>,
            F: Fn(&P, &Monitor) + 'static,
        >(
            this: *mut ffi::GdkSurface,
            monitor: *mut ffi::GdkMonitor,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                Surface::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(monitor),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"enter-monitor".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    enter_monitor_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "event")]
    fn connect_event<F: Fn(&Self, &Event) -> bool + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn event_trampoline<
            P: IsA<Surface>,
            F: Fn(&P, &Event) -> bool + 'static,
        >(
            this: *mut ffi::GdkSurface,
            event: *mut ffi::GdkEvent,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(
                Surface::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(event),
            )
            .into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"event".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    event_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "layout")]
    fn connect_layout<F: Fn(&Self, i32, i32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn layout_trampoline<P: IsA<Surface>, F: Fn(&P, i32, i32) + 'static>(
            this: *mut ffi::GdkSurface,
            width: std::ffi::c_int,
            height: std::ffi::c_int,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                Surface::from_glib_borrow(this).unsafe_cast_ref(),
                width,
                height,
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"layout".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    layout_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "leave-monitor")]
    fn connect_leave_monitor<F: Fn(&Self, &Monitor) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn leave_monitor_trampoline<
            P: IsA<Surface>,
            F: Fn(&P, &Monitor) + 'static,
        >(
            this: *mut ffi::GdkSurface,
            monitor: *mut ffi::GdkMonitor,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                Surface::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(monitor),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"leave-monitor".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    leave_monitor_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "render")]
    fn connect_render<F: Fn(&Self, &cairo::Region) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn render_trampoline<
            P: IsA<Surface>,
            F: Fn(&P, &cairo::Region) -> bool + 'static,
        >(
            this: *mut ffi::GdkSurface,
            region: *mut cairo::ffi::cairo_region_t,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(
                Surface::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(region),
            )
            .into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"render".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    render_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "cursor")]
    fn connect_cursor_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_cursor_trampoline<P: IsA<Surface>, F: Fn(&P) + 'static>(
            this: *mut ffi::GdkSurface,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Surface::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::cursor".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_cursor_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "height")]
    fn connect_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_height_trampoline<P: IsA<Surface>, F: Fn(&P) + 'static>(
            this: *mut ffi::GdkSurface,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Surface::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::height".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_height_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "mapped")]
    fn connect_mapped_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_mapped_trampoline<P: IsA<Surface>, F: Fn(&P) + 'static>(
            this: *mut ffi::GdkSurface,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Surface::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::mapped".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_mapped_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v4_12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_12")))]
    #[doc(alias = "scale")]
    fn connect_scale_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_scale_trampoline<P: IsA<Surface>, F: Fn(&P) + 'static>(
            this: *mut ffi::GdkSurface,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Surface::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::scale".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_scale_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "scale-factor")]
    fn connect_scale_factor_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_scale_factor_trampoline<
            P: IsA<Surface>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GdkSurface,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Surface::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::scale-factor".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_scale_factor_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "width")]
    fn connect_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_width_trampoline<P: IsA<Surface>, F: Fn(&P) + 'static>(
            this: *mut ffi::GdkSurface,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Surface::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::width".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_width_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<Surface>> SurfaceExt for O {}
