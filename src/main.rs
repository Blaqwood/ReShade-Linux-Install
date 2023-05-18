mod runtime_gui;
mod runtime;

fn main() {
    let app = gtk::Application::builder().build();
    app.connect_activate(runtime_gui::build);
    app.run();
}
