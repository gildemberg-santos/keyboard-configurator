use glib::object::ObjectType;
use std::mem;
use system76_keyboard_configurator::keyboard_backlight_widget;

#[no_mangle]
pub extern "C" fn pop_keyboard_backlight_widget() -> *mut gtk_sys::GtkWidget {
    unsafe {
        gtk::set_initialized();
    }

    let widget = keyboard_backlight_widget();
    let ptr = widget.as_ptr();
    mem::forget(widget);

    ptr
}
