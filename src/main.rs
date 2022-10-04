mod gui;
mod install;

use gtk::prelude::*;

static APP_ID: &str = "com.blackwood.reshade";

fn main() {
    let app = gtk::Application::builder().application_id(APP_ID).build();

    app.connect_activate(gui::build);

    app.run();
}