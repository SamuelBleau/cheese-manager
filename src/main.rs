pub mod core;
pub mod ui;

use gtk::gdk;
use gtk::prelude::*;
use gtk::{Application, CssProvider};

use crate::core::config::AppConfig;

const APP_ID: &str = "com.github.samuelbleau.cheese-manager";

fn main() {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_startup(|_| {
        let config = AppConfig::load();
        load_css(&config.css());
    });

    app.connect_activate(crate::ui::window::build_ui);
    app.run();
}

fn load_css(css_data: &str) {
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