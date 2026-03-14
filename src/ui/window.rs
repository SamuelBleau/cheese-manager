use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box, Label, Orientation, Paned, Button, FlowBox, SelectionMode, Align, ScrolledWindow, PolicyType};
use std::path::Path;

use crate::core::fs::list_directory;
use crate::core::node::NodeType;

pub fn build_ui(app: &Application) {
    let paned = Paned::builder()
        .orientation(Orientation::Horizontal)
        .position(200)
        .build();

    let sidebar = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(5)
        .css_classes(vec!["sidebar".to_string()])
        .build();

    let shortcuts = vec!["🐀 Home", "󰉋 Root", "󰈙 Projects"];
    for name in shortcuts {
        let button = Button::with_label(name);
        button.child().unwrap().downcast::<Label>().unwrap().set_halign(Align::Start);
        sidebar.append(&button);
    }

    let main_area = Box::builder()
        .orientation(Orientation::Vertical)
        .css_classes(vec!["main-area".to_string()])
        .build();

    let flowbox = FlowBox::new();
    flowbox.set_valign(Align::Start);
    flowbox.set_max_children_per_line(10);
    flowbox.set_selection_mode(SelectionMode::None);

    // Get the user's home directory (or fallback to root)
    let home_dir = std::env::var("HOME").unwrap_or_else(|_| "/".to_string());
    let path = Path::new(&home_dir);

    // Call the core module to read the directory
    if let Ok(nodes) = list_directory(&path) {
        for node in nodes {
            // Pick an icon based on NodeType
            let icon = match node.node_type {
                NodeType::Directory => "󰉋",
                NodeType::Archive => "󰗷",
                NodeType::Symlink => "󱅷",
                _ => "󰈙",
            };
            
            // Format the button label
            let label_text = format!("{} {}", icon, node.name);
            let btn = Button::with_label(&label_text);
            btn.set_size_request(100, 100);
            
            // Add custom class for styling later if nedded
            btn.add_css_class("file-item");

            flowbox.append(&btn);
        }
    } else {
        // If we can't read the directory, show an error!
        let error_label = Label::new(Some("Unable to read directory"));
        flowbox.append(&error_label);
    }

    let scrolled_window = ScrolledWindow::builder()
        .hscrollbar_policy(PolicyType::Never)
        .vscrollbar_policy(PolicyType::Automatic)
        .child(&flowbox)
        .vexpand(true)
        .build();

    main_area.append(&scrolled_window);

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
