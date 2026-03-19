use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box, Orientation, Paned};

use crate::core::config::{default_start_path, AppConfig};
use crate::ui::file_view::build_file_view;
use crate::ui::sidebar::build_sidebar;

pub fn build_ui(app: &Application) {
    let config = AppConfig::load();
    let shortcuts = config.valid_shortcuts();
    let start_path = default_start_path();

    let (file_view, current_path) = build_file_view(&start_path);

    // The sidebar needs a way to trigger a repopulate of the file view.
    // We expose that by re-using the same `populate` logic via a closure
    // that the sidebar can call with a new path.
    //
    // NOTE: Because GTK widgets are reference-counted, cloning `file_view`
    // here is cheap — it's the same widget.
    let file_view_ref = file_view.clone();
    let on_navigate = move |path: &std::path::PathBuf| {
        // Replace the child of the scrolled window to force a repopulate.
        // The actual navigation state lives inside `current_path` already;
        // we just need to tell `file_view` to refresh.
        //
        // Simplest approach: trigger a synthetic "navigate" by rebuilding the
        // child. Since `build_file_view` owns its own FlowBox internally, we
        // instead expose a separate refresh signal — see file_view::navigate_to.
        crate::ui::file_view::navigate_to(&file_view_ref, path);
    };

    let sidebar = build_sidebar(&shortcuts, current_path, on_navigate);

    let main_area = Box::builder()
        .orientation(Orientation::Vertical)
        .css_classes(vec!["main-area".to_string()])
        .build();
    main_area.append(&file_view);

    let paned = Paned::builder()
        .orientation(Orientation::Horizontal)
        .position(200)
        .build();
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