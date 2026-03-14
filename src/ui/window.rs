use gtk::prelude::*;
use gtk::{
    Align, Application, ApplicationWindow, Box, Button, FlowBox, Label, Orientation, Paned,
    PolicyType, ScrolledWindow, SelectionMode,
};
use std::path::{Path, PathBuf};

use crate::core::fs::list_directory;
use crate::core::node::NodeType;
use std::rc::Rc;
use std::cell::RefCell;

fn load_directory_into_flowbox(flowbox: &FlowBox, current_path: Rc<RefCell<PathBuf>>, path: &Path) {
    // clear existing children
    while let Some(child) = flowbox.first_child() {
        flowbox.remove(&child);
    }

    match list_directory(path) {
        Ok(nodes) => {
            for node in nodes {
                let icon = match node.node_type {
                    NodeType::Directory => "󰉋",
                    NodeType::Archive => "󰗷",
                    NodeType::Symlink => "󱅷",
                    _ => "󰈙",
                };

                let label_text = format!("{} {}", icon, node.name);
                let btn = Button::with_label(&label_text);
                btn.set_size_request(100, 100);
                btn.add_css_class("file-item");

                // If it's a directory, make click navigate into it
                if node.node_type == NodeType::Directory {
                    let node_path = node.path.clone();
                    let current_path_clone = current_path.clone();
                    let flowbox_clone = flowbox.clone();
                    btn.connect_clicked(move |_| {
                        *current_path_clone.borrow_mut() = node_path.clone();
                        load_directory_into_flowbox(&flowbox_clone, current_path_clone.clone(), &node_path);
                    });
                }

                flowbox.append(&btn);
            }
        }
        Err(_) => {
            let error_label = Label::new(Some("Unable to read directory"));
            flowbox.append(&error_label);
        }
    }
}

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

    // Shared state: current path
    let current_path = Rc::new(RefCell::new(PathBuf::from(
        std::env::var("HOME").unwrap_or_else(|_| "/".to_string()),
    )));

    // initial populate
    load_directory_into_flowbox(&flowbox, current_path.clone(), &current_path.borrow());

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
