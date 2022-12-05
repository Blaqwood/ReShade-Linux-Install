use gtk::{
    prelude::*,
    glib::Type,
};

pub const SHADERS: [(&str, &str); 3] = [
    ("Standard Effects", "https://github.com/"),
    ("SweetFX by Ceejay.dk", "https://github.com/CeeJayDK/SweetFX"),
    ("AstrayFX by BlueSkyDefender", "https://blueskydefender.github.io/AstrayFX/"),
];

fn main() {
    let _ = gtk::init();
    let win = gtk::Window::new(gtk::WindowType::Toplevel);
    let widget_box = gtk::Box::new(gtk::Orientation::Vertical, 20);

    let tree = gtk::TreeView::new();
    let model = gtk::ListStore::new(&[3, Type::STRING, Type::STRING]);
    /*let data = gtk::ListStore::
    for (i, (s, u)) in SHADERS.iter().enumerate() {
        data.append();
    }*/

    widget_box.add(&tree);
    widget_box.add(&gtk::Label::new(Some("Select")));
    
    win.add(&widget_box);
    win.show_all();
    gtk::main();
}

fn diag(selected: Vec<String>) {
    let message: String = format!("You selected {}",
        selected.join(",")
    );
    let d = gtk::MessageDialog::new(
        None::<&gtk::Window>,
        gtk::DialogFlags::MODAL,
        gtk::MessageType::Info,
        gtk::ButtonsType::Close,
        message.as_str()
    );
    d.run();
}