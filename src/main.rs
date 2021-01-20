use gobject_sys::g_object_unref;
use gtk_sys::{
    gtk_application_new, gtk_application_window_new, gtk_container_add, gtk_text_view_get_buffer,
    gtk_text_view_new, gtk_widget_get_parent_window, gtk_widget_show_all,
    gtk_window_set_default_size, gtk_window_set_title, GtkApplication, GtkTextView, GtkWidget,
};
use std::ptr;
use std::time::Instant;
use std::{mem::MaybeUninit, os::raw::c_char};

macro_rules! cs {
    ($text:expr) => {
        concat!($text, "\0").as_ptr().cast::<c_char>()
    };
}

unsafe fn enter_keys(text_view: *mut GtkTextView, keys: &[(u16, i32, u32)]) {
    let window = gtk_widget_get_parent_window(text_view.cast());

    let ev = gdk_sys::gdk_event_new(gdk_sys::GDK_KEY_PRESS);
    (*ev).key.window = window;
    (*ev).key.send_event = 1;

    let start = Instant::now();

    for &(code, keyval, state) in keys.iter() {
        (*ev).key.hardware_keycode = code;
        (*ev).key.keyval = keyval as u32;
        (*ev).key.state = state;
        gtk_sys::gtk_text_view_im_context_filter_keypress(text_view.cast(), &mut (*ev).key);
    }

    gtk_sys::gtk_text_view_reset_im_context(text_view.cast());

    let elapsed = start.elapsed();

    gdk_sys::gdk_event_free(ev);

    println!("elapsed: {}ms", elapsed.as_millis());

    let buffer = gtk_text_view_get_buffer(text_view.cast());
    let mut start = MaybeUninit::uninit();
    let mut end = MaybeUninit::uninit();
    gtk_sys::gtk_text_buffer_get_start_iter(buffer, start.as_mut_ptr());
    gtk_sys::gtk_text_buffer_get_end_iter(buffer, end.as_mut_ptr());
    let c = gtk_sys::gtk_text_buffer_get_text(buffer, start.as_ptr(), end.as_ptr(), 0);
    let s = std::ffi::CString::from_raw(c);

    println!("text: {}", s.into_string().unwrap());

    std::process::exit(0);
}

unsafe extern "C" fn realize(window: *mut GtkWidget) {
    let text_view = gtk_text_view_new();

    gtk_container_add(window.cast(), text_view);

    let mut keys = Vec::new();

    // Enable hangul
    keys.push((65, gdk_sys::GDK_KEY_space, gdk_sys::GDK_SHIFT_MASK));

    for _ in 0..10 {
        // 테스트
        keys.push((53, gdk_sys::GDK_KEY_x, 0));
        keys.push((33, gdk_sys::GDK_KEY_p, 0));
        keys.push((28, gdk_sys::GDK_KEY_t, 0));
        keys.push((58, gdk_sys::GDK_KEY_m, 0));
        keys.push((53, gdk_sys::GDK_KEY_x, 0));
        keys.push((58, gdk_sys::GDK_KEY_m, 0));

        // 문자열
        keys.push((38, gdk_sys::GDK_KEY_a, 0));
        keys.push((57, gdk_sys::GDK_KEY_n, 0));
        keys.push((39, gdk_sys::GDK_KEY_s, 0));
        keys.push((25, gdk_sys::GDK_KEY_w, 0));
        keys.push((45, gdk_sys::GDK_KEY_k, 0));
        keys.push((40, gdk_sys::GDK_KEY_d, 0));
        keys.push((30, gdk_sys::GDK_KEY_u, 0));
        keys.push((41, gdk_sys::GDK_KEY_f, 0));
    }

    // END
    keys.push((9, gdk_sys::GDK_KEY_Escape, 0));

    enter_keys(text_view.cast(), &keys);
}

unsafe extern "C" fn activate(app: *mut GtkApplication) {
    let window = gtk_application_window_new(app);
    gtk_window_set_title(window.cast(), cs!("Benchmark"));
    gtk_window_set_default_size(window.cast(), 400, 300);

    gobject_sys::g_signal_connect_data(
        window.cast(),
        cs!("realize"),
        Some(std::mem::transmute(realize as *const fn(*mut GtkWidget))),
        ptr::null_mut(),
        None,
        0,
    );

    gtk_widget_show_all(window);
}

unsafe fn unsafe_main() {
    let app = gtk_application_new(cs!("github.riey.korean-ime-bench"), 0);

    gobject_sys::g_signal_connect_data(
        app.cast(),
        cs!("activate"),
        Some(std::mem::transmute(
            activate as *const fn(*mut GtkApplication),
        )),
        ptr::null_mut(),
        None,
        0,
    );

    gio_sys::g_application_run(app.cast(), 0, ptr::null_mut());

    g_object_unref(app.cast());
}

fn main() {
    unsafe { unsafe_main() }
}
