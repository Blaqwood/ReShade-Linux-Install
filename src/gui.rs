use gtk::{prelude::*};

use crate::{var, install};

const EXE_FILTER: &str = "*.exe";

struct Sh_CheckBox {
    check_buttons: Vec<gtk::CheckButton>,
    shader: Vec<&'static str>
}

impl Sh_CheckBox {
    pub fn get_enabled(&self) -> Vec<&'static str> {
        self.check_buttons.iter()
            .zip(self.shader.clone())
            .filter(|(c, s)| c.is_active())
            .map(|(_, s)| s)
            .collect()
    }
}

pub fn build(app: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(app);
    let main_box = gtk::Box::new(gtk::Orientation::Vertical, 10);

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

    let lst_widget = gtk::ListBox::new();
    let scroll_widget = gtk::ScrolledWindow::builder().child(&lst_widget);
    
    let lst = var::names.iter().fold((vec![], vec![]), |mut acc, (n, m)| {
        let c = gtk::CheckButton::new();
        let t = gtk::Label::new(Some(n));

        let b = gtk::Box::new(gtk::Orientation::Horizontal, 20);
        b.add(&c);
        b.add(&t);

        lst_widget.add(&b);
        acc.0.push(c);
        acc.1.push(n);
        (acc.0, acc.1)
    });

    page3.add(&gtk::Box::builder().child(&lst_widget).build());
    //let list = install::SHADERS.iter().map(|t| {
//        (check_button, t.1)
    //}).collect::<Vec<_>>();

    //scroll_widget.add(&list_widget);
    /* End of Page 3 */
    
    let stack = create_stack(&[page1, page2, page3]);
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

    window.set_default_size(400, 500);

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

fn create_stack(p: &[gtk::Box]) -> gtk::Stack {
    let stack = gtk::Stack::builder().transition_type(gtk::StackTransitionType::SlideLeft).transition_duration(300).build();

    for b in p {
        stack.add(b);
    }
    stack
}