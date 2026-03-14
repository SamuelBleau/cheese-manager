pub mod core;

use gtk::gdk;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box, CssProvider, Label, Orientation, Paned};

const APP_ID: &str = "com.github.samuelbleau.cheese-manager";

fn main() {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_startup(|_| {
        load_css();
    });

    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    let paned = Paned::builder()
        .orientation(Orientation::Horizontal)
        .position(200)
        .build();

    let sidebar = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(5)
        .css_classes(vec!["sidebar".to_string()])
        .build();

    let shortcuts = vec!["🐀 Home", "Root", "Projects"];
    for name in shortcuts {
        let button = gtk::Button::with_label(name);
        button.child().unwrap().downcast::<Label>().unwrap().set_halign(gtk::Align::Start);
        sidebar.append(&button);
    }

    let main_area = Box::builder()
        .orientation(Orientation::Vertical)
        .css_classes(vec!["main-area".to_string()])
        .build();

    let flowbox = gtk::FlowBox::new();
    flowbox.set_valign(gtk::Align::Start);
    flowbox.set_max_children_per_line(10);
    flowbox.set_selection_mode(gtk::SelectionMode::None);

    let fake_folders = vec!["󰉋 src", "target", "Cargo.toml", "flake.nix", ".git"];
    for folder in fake_folders {
        let btn = gtk::Button::with_label(folder);
        btn.set_size_request(100, 100); 
        flowbox.append(&btn);
    }

    main_area.append(&flowbox);

    paned.set_start_child(Some(&sidebar));
    paned.set_end_child(Some(&main_area));

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Cheese Manager")
        .default_width(900)
        .default_height(600)
        .child(&paned)
        .build();

    window.present();
}

fn load_css() {
        let css_data = r#"
        window {
            background-color: #1a1a24;
            color: #EAEAEA;
            font-family: "Hack Nerd Font", sans-serif;
        }

        .sidebar {
            background-color: #252535;
            border-right: 1px solid #51516a;
            padding: 10px;
        }

        .main-area {
            background-color: #1a1a24;
            padding: 10px;
        }

        label {
            color: #EAEAEA;
        }
        
        button {
            background-image: none;
            background-color: #252535;
            color: #EAEAEA;
            border-radius: 6px;
            border: 1px solid #51516a;
        }
        button:hover {
            box-shadow: 0 0 10px rgba(155, 89, 182, 0.3);
            border-color: #9b59b6;
        }
    "#;

    let provider = CssProvider::new();
    provider.load_from_string(css_data);

    if let Some(display) = gdk::Display::default() {
        gtk::style_context_add_provider_for_display(
            &display,
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }
}