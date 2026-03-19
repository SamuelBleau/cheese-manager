use gtk::prelude::*;
use gtk::{Align, Box, Button, Label, Orientation};

use crate::core::config::{resolve_path, ShortcutConfig};
use crate::ui::file_view::CurrentPath;

use std::path::PathBuf;

/// Builds the sidebar [`Box`] from a list of validated shortcuts.
///
/// Each shortcut gets a button; clicking it navigates the file view to
/// that path via the shared `current_path` + `on_navigate` callback.
pub fn build_sidebar(
    shortcuts: &[ShortcutConfig],
    current_path: CurrentPath,
    on_navigate: impl Fn(&PathBuf) + Clone + 'static,
) -> Box {
    let sidebar = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(5)
        .css_classes(vec!["sidebar".to_string()])
        .build();

    for shortcut in shortcuts {
        let target: PathBuf = resolve_path(&shortcut.path);
        let btn = shortcut_button(&shortcut.label);

        let cp = current_path.clone();
        let nav = on_navigate.clone();
        btn.connect_clicked(move |_| {
            cp.replace(target.clone());
            nav(&target);
        });

        sidebar.append(&btn);
    }

    sidebar
}

fn shortcut_button(label: &str) -> Button {
    let btn = Button::with_label(label);
    if let Some(child) = btn.child() {
        if let Ok(lbl) = child.downcast::<Label>() {
            lbl.set_halign(Align::Start);
        }
    }
    btn
}