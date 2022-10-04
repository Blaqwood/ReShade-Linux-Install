use gtk::{prelude::*};

use crate::install;

const EXE_FILTER: &str = "*.exe";

struct CheckBox {
    main_widget: gtk::Box,
    check_widget: gtk::CheckButton,
    data:(&'static str, &'static str),
}

impl CheckBox {
    pub fn new(name: &str, link: &str) -> Self {
        let check_button = gtk::CheckButton::new();
        let link_to = gtk::LinkButton::with_label(link, name);
        let main = gtk::Box::builder().orientation(gtk::Orientation::Horizontal).child(&check_button).child(&link_to).build();
        Self {
            main_widget: main,
            check_widget: check_button,
            data: (name, link),
        }
    }

    pub fn add_widget(&self) -> &gtk::Box {
        &self.main_widget
    }

    pub fn get_shader(&self) -> Option<(&'a str, &'a str)> { 
        if self.check.is_active() {
            Some(self.data)
        }
        else {
            None
        }
    }
}

pub fn build(app: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(app);
    let main_box = gtk::Box::new(gtk::Orientation::Vertical, 10);

    let stack = gtk::Stack::builder().transition_type(gtk::StackTransitionType::SlideLeft).transition_duration(300).build();

    let bar = gtk::ActionBar::new();
    let next = gtk::Button::with_label("Next");

    bar.pack_end(&next);
    
    /* Start of page 1 */
    let page1 = gtk::Box::new(gtk::Orientation::Vertical, 20);
    let reshade_file = gtk::FileChooserButton::new("Open File", gtk::FileChooserAction::Open);
    page1.add(&gtk::Label::new(Some("Select path to ReShade executable")));
    page1.add(&reshade_file);
    /* End of page 1 */

    /* Start of page 2 */
    let page2 = gtk::Box::new(gtk::Orientation::Vertical, 20);
    let game_file = {
        let main = gtk::FileChooserButton::new("Open File", gtk::FileChooserAction::Open);
        let exe = gtk::FileFilter::new();
        exe.add_pattern(EXE_FILTER);
        main.add_filter(&exe);
        main
    };
    page2.add(&gtk::Label::new(Some("Select path to executable")));
    page2.add(&game_file);
    /* End of Page 2 */


    /* Start of Page 3 */
    let page3 = gtk::Box::new(gtk::Orientation::Vertical, 20);

    let scroll_widget = gtk::ScrolledWindow::builder();
    let list_widget = gtk::ListBox::new();


    let list = install::SHADERS.iter().map(|t| {
        (check_button, t.1)
    }).collect::<Vec<_>>();

    //scroll_widget.add(&list_widget);
    /* End of Page 3 */
    
    stack.add_named(&page1, "Page1");
    stack.add_named(&page2,"Page2");
    stack.add_named(&page3, "Page3");
    
    main_box.add(&bar);
    main_box.add(&stack);

    next.connect_clicked(move |btn| {
        match stack.visible_child_name().as_deref() {
            Some("Page1") => stack.set_visible_child_name("Page2"),
            Some("Page2") => stack.set_visible_child_name("Page3"),
            Some("Page3") => println!("jtr"),
            _ => println!("323")
        }
    });

    window.add(&main_box);

    window.show_all();
}

fn create_list(list: Vec<&str>) -> gtk::ListStore {
    todo!()
}

fn success_diag() {
    let diag = gtk::MessageDialog::new(
        None::<&gtk::Window>,
        gtk::DialogFlags::empty(),
        gtk::MessageType::Error,
        gtk::ButtonsType::Close,
        "Install Success"
    );
//    install::cleaup();
    diag.connect_response(|_, res| gtk::main_quit());
}

fn err_diag(message: &str) {
    let diag = gtk::MessageDialog::new(
        None::<&gtk::Window>,
        gtk::DialogFlags::empty(),
        gtk::MessageType::Error,
        gtk::ButtonsType::Close,
        message
    );
    //install::cleaup();
    diag.connect_response(|_, res| {
        if res == gtk::ResponseType::Close {
            gtk::main_quit()
        }
    });
}