use std::io::prelude::*;
use std::path::{PathBuf, Path};

use gtk::{Orientation};
use gtk::prelude::*;

#[derive(Debug)]
enum API {
    Dxgi,
    D3d9,
    Opengl,
}

pub fn main() {
    const APP_ID: &str = "com.blackwood.reshade";
    let app = gtk::Application::builder()
        .application_id(APP_ID)
        .build();
    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &gtk::Application) {
    let main_win = gtk::ApplicationWindow::builder()
        .application(app)
        .title("To do list")
        .default_width(400)
        .default_height(500)
        .build();
        
    let widgets = gtk::Box::builder()
        .margin(10)
        .orientation(gtk::Orientation::Vertical)
        .spacing(5)
    .build();
    
    let titlebar = gtk::HeaderBar::new();
    let about = gtk::Button::with_label("About");
    main_win.add(&widgets);
    

    let install_start = gtk::Button::with_label("Start ReShade installation");
    widgets.add(&install_start);
    
    main_win.show_all();

    install_start.connect_clicked(move |_| {
        main_win.set_can_focus(false);
        get_reshade();
    });
    
    /* install_start.connect_clicked(|file_diag| {
        let diag = gtk::MessageDialog::new(
            None::<&gtk::Window>,
            gtk::DialogFlags::empty(),
            gtk::MessageType::Info,
            gtk::ButtonsType::Ok,
            format!("File {} selected", file_diag.filename().unwrap().display()).as_str()
        );
        diag.run();
    }); */
    

}

fn install(game_path: PathBuf, api: API) {
    
}

fn get_reshade() {
    let window = gtk::Window::builder()
        .title("ReShade")
    .build();
    let widgets = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(20)
        .margin(18)
    .build();
    
    window.add(&widgets);

    let lab = gtk::Label::new(Some("Select ReShade Executable"));
    let open_file = gtk::FileChooserButton::new("Open File", gtk::FileChooserAction::Open);
    let next = gtk::Button::with_label("Next");
    
    widgets.add(&lab);
    widgets.add(&open_file);
    widgets.add(&next);

    window.show_all();

    next.connect_clicked(move |_| {
        match open_file.filename() {
            Some(_) => {
                let filename = open_file.filename().unwrap();
                let file = std::fs::File::open(filename).unwrap();
                let archive = zip_extract::extract(file, &Path::new("/tmp"), true);
                match archive {
                    Ok(()) => println!("Extracted"),
                    Err(e) => println!("{e}"),
                }
                window.close();
                get_game();
            }
            None => {
                window.close();
                error_diag();
            }
        }
    });
}

fn get_api() {
    let window = gtk::Window::builder()
        .title("ReShade")
    .build();
    let widgets = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(20)
        .margin(18)
    .build();

    window.add(&widgets);

    let radio_buttons = gtk::Box::new(Orientation::Vertical, 25);
    let radio1 = gtk::RadioButton::with_label("Direct3D 9");
    let radio2 = gtk::RadioButton::with_label_from_widget(&radio1, "Direct3D 10/11/12");
    let radio3 = gtk::RadioButton::with_label_from_widget(&radio1, "OpenGL");

    radio_buttons.add(&radio1);
    radio_buttons.add(&radio2);
    radio_buttons.add(&radio3);

    let lab = gtk::Label::new(Some("Which rendering API does the game use?"));
    let choice = gtk::RadioButton::new();
    let next = gtk::Button::with_label("Next");

    widgets.add(&lab);
    widgets.add(&radio_buttons);
    widgets.add(&next);

    window.show_all();

    next.connect_clicked(move |_| {
        window.close();
        let mut api = API::D3d9;
        if radio1.is_active() {
            api = API::D3d9;
        }
        else if radio2.is_active() {
            api = API::Dxgi;
        }
        else if radio3.is_active() {
            api = API::Opengl;
        }
        //install_dll(api);
    });

}

fn get_game() {
    let window = gtk::Window::builder()
        .title("ReShade")
    .build();
    let widgets = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(20)
        .margin(18)
    .build();

    window.add(&widgets);

    let lab = gtk::Label::new(Some("Select Game Executable"));
    lab.set_margin(30);
    let open_file = gtk::FileChooserButton::new("Open File",gtk::FileChooserAction::Open);
    let next = gtk::Button::with_label("Next");
    widgets.add(&lab);
    widgets.add(&open_file);
    widgets.add(&next);

    window.show_all();

    next.connect_clicked(move |_| {
        let mut path = open_file.filename().unwrap();
        create_config(&mut path);
        window.close();
        get_api();
    });
}

fn create_config(path: &mut PathBuf) {
    let content = format!("{}\n{}\n{}\n{}\n\n{}\n{}",
        r"[GENERAL]",
        r"EffectsSearchPath=.\reshade-shaders\Shaders",
        "PreprocessorDefinitions=RESHADE_DEPTH_LINEARIZATION_FAR_PLANE=1000,RESHADE_DEPTH_INPUT_IS_UPSIDE_DOWN=0,RESHADE_DEPTH_INPUT_IS_REVERSED=1,RESHADE_DEPTH_INPUT_IS_LOGARITHMIC=0",
        r"TextureSearchPath=.\reshade-shaders\Textures",
        r"[INPUT]",
        r"KeyOverlay=36,0,0,0"
    );

    path.pop();
    path.push("ReShade.ini");
    let mut file = std::fs::File::create(path).unwrap();
    file.write_all(content.as_bytes());
    let exe = gtk::FileFilter::new();
}

fn error_diag() {
    let diag = gtk::MessageDialog::new(
        None::<&gtk::Window>,
        gtk::DialogFlags::empty(),
        gtk::MessageType::Error,
        gtk::ButtonsType::Close,
        format!("An Error Occured").as_str()
    );
    diag.connect_close(|_| {
        
    });
    diag.run();
}

fn download_shader() {
    todo!();
}