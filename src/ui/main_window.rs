use gtk::prelude::*;
use sourceview::prelude::*;
use crate::core::render;
use std::rc::Rc;
use std::cell::RefCell;
use std::fs;
use std::path::PathBuf;

pub fn build(app: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(app);
    window.set_title("Markd");
    window.set_default_size(1000, 600);

    let header_bar = gtk::HeaderBar::new();
    header_bar.set_show_close_button(true);
    header_bar.set_title(Some("Markd"));
    window.set_titlebar(Some(&header_bar));

    let open_button = gtk::Button::with_label("Open");
    let save_button = gtk::Button::with_label("Save");
    
    header_bar.pack_start(&open_button);
    header_bar.pack_end(&save_button);

    // Theme toggle
    let theme_switch = gtk::Switch::new();
    let theme_label = gtk::Label::new(Some("Dark Mode"));
    let theme_box = gtk::Box::new(gtk::Orientation::Horizontal, 5);
    theme_box.add(&theme_label);
    theme_box.add(&theme_switch);
    header_bar.pack_end(&theme_box);

    if let Some(settings) = gtk::Settings::get_default() {
        let mut is_dark = false;
        if let Ok(val) = settings.get_property("gtk-application-prefer-dark-theme") {
            if let Ok(Some(d)) = val.get::<bool>() {
                is_dark = d;
            }
        }
        theme_switch.set_active(is_dark);
    }

    theme_switch.connect_property_active_notify(|switch| {
        let is_dark = switch.get_active();
        if let Some(settings) = gtk::Settings::get_default() {
            let _ = settings.set_property("gtk-application-prefer-dark-theme", &is_dark);
        }
    });

    let paned = gtk::Paned::new(gtk::Orientation::Horizontal);
    paned.set_position(500);

    let buffer = sourceview::Buffer::new(None::<&gtk::TextTagTable>);
    let view = sourceview::View::new_with_buffer(&buffer);
    view.set_monospace(true);
    view.set_show_line_numbers(true);
    
    let scroll_editor = gtk::ScrolledWindow::new(None::<&gtk::Adjustment>, None::<&gtk::Adjustment>);
    scroll_editor.add(&view);

    let preview_label = gtk::Label::new(None);
    preview_label.set_line_wrap(true);
    preview_label.set_xalign(0.0);
    preview_label.set_yalign(0.0);
    preview_label.set_selectable(true);
    preview_label.set_use_markup(true);
    
    let scroll_preview = gtk::ScrolledWindow::new(None::<&gtk::Adjustment>, None::<&gtk::Adjustment>);
    scroll_preview.add(&preview_label);

    paned.pack1(&scroll_editor, true, false);
    paned.pack2(&scroll_preview, true, false);

    window.add(&paned);

    // Application State
    let current_file = Rc::new(RefCell::new(None::<PathBuf>));

    // Connect signals
    let label_clone = preview_label.clone();
    buffer.connect_changed(move |buf: &sourceview::Buffer| {
        let (start, end) = buf.get_bounds();
        let text = buf.get_text(&start, &end, true).unwrap();
        
        let pango_markup = render::markdown_to_pango(&text);
        label_clone.set_markup(&pango_markup);
    });

    // Open logic
    let window_clone = window.clone();
    let buffer_clone = buffer.clone();
    let current_file_open = current_file.clone();
    open_button.connect_clicked(move |_| {
        let dialog = gtk::FileChooserDialog::with_buttons(
            Some("Open File"),
            Some(&window_clone),
            gtk::FileChooserAction::Open,
            &[("_Cancel", gtk::ResponseType::Cancel), ("_Open", gtk::ResponseType::Accept)],
        );

        if dialog.run() == gtk::ResponseType::Accept {
            if let Some(path) = dialog.get_filename() {
                if let Ok(content) = fs::read_to_string(&path) {
                    buffer_clone.set_text(&content);
                    *current_file_open.borrow_mut() = Some(path);
                }
            }
        }
        dialog.close();
    });

    // Save logic
    let window_save_clone = window.clone();
    let buffer_save_clone = buffer.clone();
    let current_file_save = current_file.clone();
    save_button.connect_clicked(move |_| {
        let mut path = current_file_save.borrow().clone();
        
        if path.is_none() {
            let dialog = gtk::FileChooserDialog::with_buttons(
                Some("Save File"),
                Some(&window_save_clone),
                gtk::FileChooserAction::Save,
                &[("_Cancel", gtk::ResponseType::Cancel), ("_Save", gtk::ResponseType::Accept)],
            );
            dialog.set_do_overwrite_confirmation(true);

            if dialog.run() == gtk::ResponseType::Accept {
                path = dialog.get_filename();
            }
            dialog.close();
        }

        if let Some(p) = path {
            let (start, end) = buffer_save_clone.get_bounds();
            let text = buffer_save_clone.get_text(&start, &end, true).unwrap();
            if fs::write(&p, text.as_str()).is_ok() {
                *current_file_save.borrow_mut() = Some(p);
            }
        }
    });

    window.show_all();
}