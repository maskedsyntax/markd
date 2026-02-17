use gpui::*;
mod theme;
mod workspace;
mod toolbar;
mod renderer;
mod preview;

use theme::Theme;
use workspace::Workspace;

fn main() {
    Application::new().run(|cx| {
        gpui_component::init(cx);
        cx.set_global(Theme::dark());
        
        cx.open_window(WindowOptions::default(), |window, cx| {
            cx.new(|cx| Workspace::new(window, cx))
        }).expect("failed to open window");
    });
}
