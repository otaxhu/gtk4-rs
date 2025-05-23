mod imp;

use gtk::glib;

glib::wrapper! {
    pub struct CustomOrientable(ObjectSubclass<imp::CustomOrientable>)
        @extends gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl Default for CustomOrientable {
    fn default() -> Self {
        glib::Object::new()
    }
}
