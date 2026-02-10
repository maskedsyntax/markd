use gtk::prelude::*;
use sourceview::prelude::*;
use crate::core::render;

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

    theme_switch.connect_state_set(|_, is_dark| {
        if let Some(settings) = gtk::Settings::get_default() {
            settings.set_property("gtk-application-prefer-dark-theme", &is_dark).unwrap();
        }
        Inhibit(false)
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

    let label_clone = preview_label.clone();
    buffer.connect_changed(move |buf: &sourceview::Buffer| {
        let (start, end) = buf.get_bounds();
        let text = buf.get_text(&start, &end, true).unwrap();
        
        let pango_markup = render::markdown_to_pango(&text);
        label_clone.set_markup(&pango_markup);
    });

    window.show_all();
}