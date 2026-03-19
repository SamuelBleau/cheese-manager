use gtk::prelude::*;
use gtk::{Align, Button, FlowBox, Label, PolicyType, ScrolledWindow, SelectionMode};
use std::cell::RefCell;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use crate::core::fs::list_directory;
use crate::core::node::NodeType;

/// Shared navigation state passed around between callbacks.
pub type CurrentPath = Rc<RefCell<PathBuf>>;

// We stash the FlowBox and CurrentPath inside the ScrolledWindow's widget data
// so that `navigate_to` can reach them without extra bookkeeping in the caller.
const FLOWBOX_KEY: &str = "cheese_flowbox";

/// Creates a [`ScrolledWindow`] containing a [`FlowBox`] that displays the
/// contents of `start_path` and handles click-to-navigate for directories.
pub fn build_file_view(start_path: &Path) -> (ScrolledWindow, CurrentPath) {
    let flowbox = FlowBox::new();
    flowbox.set_valign(Align::Start);
    flowbox.set_max_children_per_line(10);
    flowbox.set_selection_mode(SelectionMode::None);

    // Activate the inner widget (the Button) when a FlowBoxChild is activated.
    flowbox.connect_child_activated(|_, child| {
        if let Some(widget) = child.child() {
            widget.activate();
        }
    });

    let current_path: CurrentPath = Rc::new(RefCell::new(start_path.to_path_buf()));
    populate(&flowbox, current_path.clone(), start_path);

    let scrolled = ScrolledWindow::builder()
        .hscrollbar_policy(PolicyType::Never)
        .vscrollbar_policy(PolicyType::Automatic)
        .child(&flowbox)
        .vexpand(true)
        .build();

    // Stash the flowbox so `navigate_to` can reach it later.
    unsafe {
        scrolled.set_data(FLOWBOX_KEY, flowbox.clone());
    }

    (scrolled, current_path)
}

/// Called by the sidebar to navigate the file view to a new `path`.
pub fn navigate_to(scrolled: &ScrolledWindow, path: &PathBuf) {
    unsafe {
        if let Some(flowbox) = scrolled.data::<FlowBox>(FLOWBOX_KEY) {
            let flowbox = flowbox.as_ref();
            // Build a temporary CurrentPath just for the populate call;
            // the canonical one lives in build_file_view's caller.
            let cp: CurrentPath = Rc::new(RefCell::new(path.clone()));
            populate(flowbox, cp, path);
        }
    }
}

/// Clears the flowbox and repopulates it with the contents of `path`.
fn populate(flowbox: &FlowBox, current_path: CurrentPath, path: &Path) {
    while let Some(child) = flowbox.first_child() {
        flowbox.remove(&child);
    }

    // "Go up" button — only shown when a parent directory exists.
    if let Some(parent) = path.parent() {
        let parent_path = parent.to_path_buf();
        let btn = nav_button("󰁮 ..");
        let fb = flowbox.clone();
        let cp = current_path.clone();
        btn.connect_clicked(move |_| navigate(&fb, cp.clone(), &parent_path));
        flowbox.append(&btn);
    }

    match list_directory(path) {
        Ok(nodes) => {
            for node in nodes {
                let icon = node_icon(&node.node_type);
                let btn = nav_button(&format!("{icon} {}", node.name));

                if node.node_type == NodeType::Directory {
                    let node_path = node.path.clone();
                    let fb = flowbox.clone();
                    let cp = current_path.clone();
                    btn.connect_clicked(move |_| navigate(&fb, cp.clone(), &node_path));
                }

                flowbox.append(&btn);
            }
        }
        Err(_) => {
            flowbox.append(&Label::new(Some("Unable to read directory")));
        }
    }
}

/// Updates `current_path` and repopulates the flowbox.
fn navigate(flowbox: &FlowBox, current_path: CurrentPath, path: &Path) {
    current_path.replace(path.to_path_buf());
    populate(flowbox, current_path, path);
}

fn nav_button(label: &str) -> Button {
    let btn = Button::with_label(label);
    btn.set_size_request(100, 100);
    btn.add_css_class("file-item");
    btn
}

fn node_icon(node_type: &NodeType) -> &'static str {
    match node_type {
        NodeType::Directory => "󰉋",
        NodeType::Archive => "󰗷",
        NodeType::Symlink => "󱅷",
        _ => "󰈙",
    }
}