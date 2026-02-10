use gtk::prelude::*;
use gio::prelude::*;
use std::env;

mod ui;
mod core;
mod io;

fn main() {
    tracing_subscriber::fmt::init();

    let app = gtk::Application::new(
        Some("com.maskedsyntax.markd"),
        gio::ApplicationFlags::FLAGS_NONE,
    ).expect("failed to initialize GTK application");

    app.connect_activate(build_ui);

    let args: Vec<String> = env::args().collect();
    app.run(&args);
}

fn build_ui(app: &gtk::Application) {
    ui::main_window::build(app);
}