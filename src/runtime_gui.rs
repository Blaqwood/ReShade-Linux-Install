use gtk::prelude::*;
use gtk::CompositeTemplate;

use crate::runtime;
struct RepoWidget<'r> {
    checkbox: gtk::CheckButton,
    repo: &'r runtime::Repo,
}

pub fn build(app: &gtk::Application) {
    let builder = gtk::Builder::new();
    builder.add_from_file("gui.glade");

    let window = builder.object::<gtk::Window>("window").unwrap();
    let stack = builder.object::<gtk::Stack>("stack").unwrap();
    let repo_list = builder.object::<gtk::ListBox>("repo_list").unwrap();
    let next_btn = builder.object::<gtk::Button>("next").unwrap();
    let back_btn = builder.object::<gtk::Button>("back").unwrap();

    let stack_clones = [stack.clone(), stack.clone()];

    populate_repo_list(&repo_list);

    next_btn.connect_clicked(|b| {
        match stack.visible_child_name().as_ref() {
            Some("joe") => ()
            _ => ()
        }
    });
    


    window.show_all();
}


fn populate_repo_list(list_box: &gtk::ListBox) {
    let repos = runtime::Repos;
    for repo in repos {
        list_box.add(&create_repo_widget(&repo));
    }
    println!("Repo List: done");
}

fn create_repo_widget(repo: &runtime::Repo) -> gtk::Box {
    let main_box = gtk::Box::new(gtk::Orientation::Horizontal, 15);
    let check_box = gtk::CheckButton::new();
    let text_box = gtk::Box::new(gtk::Orientation::Horizontal, 5);
    let title = gtk::Label::new(Some(repo.name));
    let url = gtk::Label::new(Some(repo.url));

    text_box.add(&title);
    text_box.add(&url);
    
    main_box.add(&check_box);
    main_box.add(&text_box);

    return main_box;
}
