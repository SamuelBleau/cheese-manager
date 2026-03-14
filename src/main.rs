pub mod core;
pub mod ui;

use gtk::gdk;
use gtk::prelude::*;
use gtk::{Application, CssProvider};

const APP_ID: &str = "com.github.samuelbleau.cheese-manager";

fn main() {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_startup(|_| {
        load_css();
    });

    app.connect_activate(crate::ui::window::build_ui);
    app.run();
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