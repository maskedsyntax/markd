use adw::prelude::*;
use gtk::prelude::*;
use gtk::{gio, glib};

mod ui;
mod core;
mod io;

fn main() -> glib::ExitCode {
    // Initialize standard logging (if needed, or use env_logger)
    tracing_subscriber::fmt::init();

    // Create a new Libadwaita Application
    let app = adw::Application::builder()
        .application_id("com.maskedsyntax.markd")
        .flags(gio::ApplicationFlags::default())
        .build();

    // Connect to the "activate" signal
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &adw::Application) {
    // We will implement the main window logic in ui/main_window.rs
    ui::main_window::build(app);
}
