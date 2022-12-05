use std::path::{Path, PathBuf};
use gtk::{prelude::*, gdk::EventMask};


fn main() {
    let _ = gtk::init();

    let main_win = gtk::Window::new(gtk::WindowType::Toplevel);
    let widgets_box = gtk::Box::new(gtk::Orientation::Horizontal, 30);

    let pages = gtk::Notebook::new();
    let btn = gtk::Button::with_label("Press me");
    pages.menu_label(&gtk::Label::new(Some("deez")));
    btn.connect_clicked(|_| println!("Pressed"));
    pages.add(&btn);
    widgets_box.add(&pages);
    main_win.add(&widgets_box);
    main_win.show_all();
    gtk::main();
}