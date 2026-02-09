use adw::prelude::*;
use gtk::prelude::*;
use gtk::{gio, glib};
use sourceview5::prelude::*;
use crate::core::render;

pub fn build(app: &adw::Application) {
    let window = adw::ApplicationWindow::builder()
        .application(app)
        .title("Markd")
        .default_width(1000)
        .default_height(600)
        .build();

    // Create a HeaderBar
    let header_bar = gtk::HeaderBar::builder()
        .title_widget(&adw::WindowTitle::new("Markd", ""))
        .build();

    // Create buttons for the header bar
    let open_button = gtk::Button::builder()
        .label("Open")
        .build();
    let save_button = gtk::Button::builder()
        .label("Save")
        .build();
    
    // Pack buttons into the header bar
    header_bar.pack_start(&open_button);
    header_bar.pack_end(&save_button);

    // Main content area: Split pane
    let paned = gtk::Paned::builder()
        .orientation(gtk::Orientation::Horizontal)
        .position(500)
        .shrink_start_child(false)
        .shrink_end_child(false)
        .hexpand(true)
        .vexpand(true)
        .build();

    // Left Pane: SourceView (Editor)
    let buffer = sourceview5::Buffer::new(None);
    let view = sourceview5::View::builder()
        .buffer(&buffer)
        .monospace(true)
        .show_line_numbers(true)
        .vexpand(true)
        .hexpand(true)
        .build();
    let scroll_editor = gtk::ScrolledWindow::builder()
        .child(&view)
        .hscrollbar_policy(gtk::PolicyType::Automatic)
        .vscrollbar_policy(gtk::PolicyType::Automatic)
        .vexpand(true)
        .hexpand(true)
        .build();

    // Right Pane: Label (Preview)
    let preview_label = gtk::Label::builder()
        .label("Preview will appear here")
        .wrap(true)
        .xalign(0.0)
        .yalign(0.0)
        .selectable(true)
        .vexpand(true)
        .hexpand(true)
        .use_markup(true)
        .build();
        
    let scroll_preview = gtk::ScrolledWindow::builder()
        .child(&preview_label)
        .hscrollbar_policy(gtk::PolicyType::Automatic)
        .vscrollbar_policy(gtk::PolicyType::Automatic)
        .vexpand(true)
        .hexpand(true)
        .build();

    paned.set_start_child(Some(&scroll_editor));
    paned.set_end_child(Some(&scroll_preview));

    // Layout
    let content = gtk::Box::new(gtk::Orientation::Vertical, 0);
    content.append(&header_bar);
    content.append(&paned);

    // Set window content
    window.set_content(Some(&content));

    // Connect signals
    let buffer_clone = buffer.clone();
    let label_clone = preview_label.clone();
    
    buffer.connect_changed(move |buf| {
        let start = buf.start_iter();
        let end = buf.end_iter();
        let text = buf.text(&start, &end, true);
        
        let pango_markup = render::markdown_to_pango(&text);
        label_clone.set_markup(&pango_markup);
    });

    window.present();
}
