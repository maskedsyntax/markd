use gpui::*;
mod theme;
mod workspace;
mod toolbar;
mod renderer;
mod preview;

use theme::Theme;
use workspace::Workspace;

actions!(markd, [NewFile, OpenFile, SaveFile, Quit, Undo, Redo, Cut, Copy, Paste]);

fn main() {
    Application::new().run(|cx| {
        gpui_component::init(cx);
        cx.set_global(Theme::dark());

        cx.set_menus(vec![
            Menu {
                name: "File".into(),
                items: vec![
                    MenuItem::action("New", NewFile),
                    MenuItem::action("Open", OpenFile),
                    MenuItem::separator(),
                    MenuItem::action("Save", SaveFile),
                    MenuItem::separator(),
                    MenuItem::action("Quit", Quit),
                ],
            },
            Menu {
                name: "Edit".into(),
                items: vec![
                    MenuItem::os_action("Undo", Undo, gpui::OsAction::Undo),
                    MenuItem::os_action("Redo", Redo, gpui::OsAction::Redo),
                    MenuItem::separator(),
                    MenuItem::os_action("Cut", Cut, gpui::OsAction::Cut),
                    MenuItem::os_action("Copy", Copy, gpui::OsAction::Copy),
                    MenuItem::os_action("Paste", Paste, gpui::OsAction::Paste),
                ],
            },
        ]);

        cx.on_action(|_: &Quit, cx| cx.quit());
        
        cx.open_window(WindowOptions::default(), |window, cx| {
            let workspace = cx.new(|cx| Workspace::new(window, cx));
            
            let workspace_handle = workspace.clone();
            cx.on_action(move |_: &NewFile, cx| {
                workspace_handle.update(cx, |this, cx| this.new_file(cx));
            });
            let workspace_handle = workspace.clone();
            cx.on_action(move |_: &OpenFile, cx| {
                workspace_handle.update(cx, |this, cx| this.open_file(cx));
            });
            let workspace_handle = workspace.clone();
            cx.on_action(move |_: &SaveFile, cx| {
                workspace_handle.update(cx, |this, cx| this.save_file(cx));
            });

            cx.new(|cx| gpui_component::Root::new(workspace, window, cx))
        }).expect("failed to open window");
    });
}
