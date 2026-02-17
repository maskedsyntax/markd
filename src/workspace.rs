use gpui::*;
use gpui_component::input::*;
use gpui_component::resizable::*;
use gpui_component::tab::*;
use gpui_component::menu::AppMenuBar;
use crate::theme::Theme;
use crate::toolbar::{Toolbar, ToolbarEvent};
use crate::preview::Preview;
use std::time::Duration;
use std::path::PathBuf;
use std::fs;

pub enum EditorEvent {
    Changed(String),
}

pub struct TabState {
    pub path: Option<PathBuf>,
    pub text: String,
}

pub struct Workspace {
    window_handle: AnyWindowHandle,
    menu_bar: Entity<AppMenuBar>,
    toolbar: Entity<Toolbar>,
    editor: Entity<Editor>,
    preview: Entity<Preview>,
    status_bar: Entity<StatusBar>,
    tabs: Vec<TabState>,
    active_tab_index: usize,
    auto_render: bool,
    debounce_task: Option<Task<()>>,
    autosave_task: Option<Task<()>>,
}

impl Workspace {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let window_handle = window.window_handle();
        let menu_bar = AppMenuBar::new(window, cx);
        let toolbar = cx.new(|_cx| Toolbar::new());
        let editor = cx.new(|cx| Editor::new(window, cx));
        let preview = cx.new(|_cx| Preview::new());
        let status_bar = cx.new(|_cx| StatusBar::new());
        
        let status_bar_handle = status_bar.clone();
        cx.observe(&editor, move |_this, editor, cx| {
            let input = editor.read(cx).input.read(cx);
            let cursor = input.cursor_position();
            status_bar_handle.update(cx, |status_bar, cx| {
                status_bar.set_cursor(cursor.line as usize + 1, cursor.character as usize + 1, cx);
            });
        }).detach();

        cx.subscribe(&toolbar, |this, _toolbar, event: &ToolbarEvent, cx| {
            match event {
                ToolbarEvent::New => this.new_file(cx),
                ToolbarEvent::Open => this.open_file(cx),
                ToolbarEvent::Save => this.save_file(cx),
                ToolbarEvent::Render => this.render_now(cx),
                ToolbarEvent::ToggleAutoRender(enabled) => this.auto_render = *enabled,
            }
        }).detach();

        cx.subscribe(&editor, |this, _editor, event: &EditorEvent, cx| {
            match event {
                EditorEvent::Changed(text) => {
                    let text_clone = text.clone();
                    if let Some(tab) = this.tabs.get_mut(this.active_tab_index) {
                        tab.text = text.clone();
                    }
                    if this.auto_render {
                        this.schedule_render(text_clone, cx);
                    }
                }
            }
        }).detach();
        
        let mut workspace = Self { 
            window_handle,
            menu_bar,
            toolbar, 
            editor, 
            preview, 
            status_bar,
            tabs: vec![TabState { path: None, text: String::new() }],
            active_tab_index: 0,
            auto_render: true, 
            debounce_task: None,
            autosave_task: None,
        };
        workspace.start_autosave(cx);
        workspace
    }

    fn start_autosave(&mut self, cx: &mut Context<Self>) {
        self.autosave_task = Some(cx.spawn(|this: WeakEntity<Workspace>, cx: &mut AsyncApp| {
            let mut cx = cx.clone();
            async move {
                loop {
                    cx.background_executor().timer(Duration::from_secs(30)).await;
                    let _ = this.update(&mut cx, |this, _cx| {
                        for tab in &this.tabs {
                            if let Some(path) = &tab.path {
                                let _ = fs::write(path, &tab.text);
                            }
                        }
                    });
                }
            }
        }));
    }

    pub(crate) fn new_file(&mut self, cx: &mut Context<Self>) {
        let new_tab = TabState { path: None, text: String::new() };
        self.tabs.push(new_tab);
        self.active_tab_index = self.tabs.len() - 1;
        self.update_editor_from_active_tab(cx);
        self.render_now(cx);
    }

    pub(crate) fn open_file(&mut self, cx: &mut Context<Self>) {
        if let Some(path) = rfd::FileDialog::new().pick_file() {
            if let Ok(content) = fs::read_to_string(&path) {
                let new_tab = TabState { path: Some(path), text: content };
                self.tabs.push(new_tab);
                self.active_tab_index = self.tabs.len() - 1;
                self.update_editor_from_active_tab(cx);
                self.render_now(cx);
            }
        }
    }

    pub(crate) fn save_file(&mut self, _cx: &mut Context<Self>) {
        if let Some(tab) = self.tabs.get_mut(self.active_tab_index) {
            let path = tab.path.clone().or_else(|| rfd::FileDialog::new().save_file());
            if let Some(p) = path {
                if fs::write(&p, &tab.text).is_ok() {
                    tab.path = Some(p);
                }
            }
        }
    }

    fn update_editor_from_active_tab(&mut self, cx: &mut Context<Self>) {
        if let Some(tab) = self.tabs.get(self.active_tab_index) {
            let text = tab.text.clone();
            let editor = self.editor.clone();
            cx.update_window(self.window_handle, |_, window, cx| {
                editor.update(cx, |editor, cx| {
                    editor.set_text(text, window, cx);
                });
            }).ok();
        }
    }

    fn render_now(&mut self, cx: &mut Context<Self>) {
        if let Some(tab) = self.tabs.get(self.active_tab_index) {
            let text = tab.text.clone();
            self.preview.update(cx, |preview, cx| {
                preview.set_text(text, cx);
            });
        }
    }

    fn schedule_render(&mut self, text: String, cx: &mut Context<Self>) {
        self.debounce_task = Some(cx.spawn(|this: WeakEntity<Workspace>, cx: &mut AsyncApp| {
            let mut cx = cx.clone();
            async move {
                cx.background_executor().timer(Duration::from_millis(150)).await;
                let _ = this.update(&mut cx, |this, cx| {
                    this.preview.update(cx, |preview, cx| {
                        preview.set_text(text, cx);
                    });
                });
            }
        }));
    }
}

impl Render for Workspace {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let active_index = self.active_tab_index;
        
        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(theme.background)
            .child(self.menu_bar.clone())
            .child(self.toolbar.clone())
            .child(
                TabBar::new("tab_bar")
                    .selected_index(active_index)
                    .on_click(cx.listener(move |this, index, _window, cx| {
                        this.active_tab_index = *index;
                        this.update_editor_from_active_tab(cx);
                        this.render_now(cx);
                        cx.notify();
                    }))
                    .children(self.tabs.iter().enumerate().map(|(_, tab)| {
                        let label = tab.path.as_ref()
                            .and_then(|p| p.file_name())
                            .and_then(|n| n.to_str())
                            .map(|s| s.to_string())
                            .unwrap_or_else(|| "Untitled".to_string());
                        Tab::new().label(label)
                    }))
            )
            .child(
                div()
                    .flex_1()
                    .child(
                        h_resizable("workspace_split")
                            .child(
                                resizable_panel()
                                    .child(self.editor.clone())
                            )
                            .child(
                                resizable_panel()
                                    .child(self.preview.clone())
                            )
                    )
            )
            .child(self.status_bar.clone())
    }
}

pub struct StatusBar {
    line: usize,
    character: usize,
}

impl StatusBar {
    pub fn new() -> Self {
        Self {
            line: 1,
            character: 1,
        }
    }

    pub fn set_cursor(&mut self, line: usize, character: usize, cx: &mut Context<Self>) {
        self.line = line;
        self.character = character;
        cx.notify();
    }
}

impl Render for StatusBar {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        
        div()
            .flex()
            .h_6()
            .bg(theme.status_bar_background)
            .text_color(theme.text_color)
            .text_size(px(12.0))
            .items_center()
            .px_4()
            .gap_4()
            .child(format!("Ln {}, Col {}", self.line, self.character))
            .child(div().flex_1())
            .child("UTF-8")
    }
}

pub struct Editor {
    input: Entity<InputState>,
}

impl EventEmitter<EditorEvent> for Editor {}

impl Editor {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let input = cx.new(|cx| {
            InputState::new(window, cx)
                .code_editor("markdown")
                .line_number(true)
        });
        
        cx.observe(&input, |_this, input, cx| {
            let text = input.read(cx).value().to_string();
            cx.emit(EditorEvent::Changed(text));
        }).detach();
        
        Self { input }
    }

    pub fn set_text(&mut self, text: String, window: &mut Window, cx: &mut Context<Self>) {
        self.input.update(cx, |input, cx| {
            input.set_value(&text, window, cx);
        });
    }
}

impl Render for Editor {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let theme = _cx.global::<Theme>();
        div()
            .size_full()
            .bg(theme.editor_background)
            .text_color(theme.text_color)
            .p_4()
            .child(
                Input::new(&self.input)
                    .size_full()
            )
    }
}
