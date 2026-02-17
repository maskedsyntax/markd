use gpui::*;
use gpui_component::input::*;
use crate::theme::Theme;
use crate::toolbar::{Toolbar, ToolbarEvent};
use crate::preview::Preview;
use std::time::Duration;
use std::path::PathBuf;
use std::fs;

pub enum EditorEvent {
    Changed(String),
}

pub struct Workspace {
    window_handle: AnyWindowHandle,
    toolbar: Entity<Toolbar>,
    editor: Entity<Editor>,
    preview: Entity<Preview>,
    auto_render: bool,
    last_text: String,
    debounce_task: Option<Task<()>>,
    current_path: Option<PathBuf>,
}

impl Workspace {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let window_handle = window.window_handle();
        let toolbar = cx.new(|_cx| Toolbar::new());
        let editor = cx.new(|cx| Editor::new(window, cx));
        let preview = cx.new(|_cx| Preview::new());
        
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
                    this.last_text = text.clone();
                    if this.auto_render {
                        this.schedule_render(cx);
                    }
                }
            }
        }).detach();
        
        Self { 
            window_handle,
            toolbar, 
            editor, 
            preview, 
            auto_render: true, 
            last_text: String::new(),
            debounce_task: None,
            current_path: None,
        }
    }

    fn new_file(&mut self, cx: &mut Context<Self>) {
        self.current_path = None;
        self.last_text = String::new();
        let editor = self.editor.clone();
        cx.update_window(self.window_handle, |_, window, cx| {
            editor.update(cx, |editor, cx| {
                editor.set_text(String::new(), window, cx);
            });
        }).ok();
        self.render_now(cx);
    }

    fn open_file(&mut self, cx: &mut Context<Self>) {
        if let Some(path) = rfd::FileDialog::new().pick_file() {
            if let Ok(content) = fs::read_to_string(&path) {
                self.current_path = Some(path);
                self.last_text = content.clone();
                let editor = self.editor.clone();
                cx.update_window(self.window_handle, |_, window, cx| {
                    editor.update(cx, |editor, cx| {
                        editor.set_text(content, window, cx);
                    });
                }).ok();
                self.render_now(cx);
            }
        }
    }

    fn save_file(&mut self, _cx: &mut Context<Self>) {
        let path = self.current_path.clone().or_else(|| rfd::FileDialog::new().save_file());
        if let Some(p) = path {
            if fs::write(&p, &self.last_text).is_ok() {
                self.current_path = Some(p);
            }
        }
    }

    fn render_now(&mut self, cx: &mut Context<Self>) {
        let text = self.last_text.clone();
        self.preview.update(cx, |preview, cx| {
            preview.set_text(text, cx);
        });
    }

    fn schedule_render(&mut self, cx: &mut Context<Self>) {
        self.debounce_task = Some(cx.spawn(|this: WeakEntity<Workspace>, cx: &mut AsyncApp| {
            let mut cx = cx.clone();
            async move {
                cx.background_executor().timer(Duration::from_millis(150)).await;
                let _ = this.update(&mut cx, |this, cx| {
                    this.render_now(cx);
                });
            }
        }));
    }
}

impl Render for Workspace {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        
        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(theme.background)
            .child(self.toolbar.clone())
            .child(
                div()
                    .flex()
                    .flex_1()
                    .child(self.editor.clone())
                    .child(
                        div()
                            .w_1()
                            .bg(theme.border)
                    )
                    .child(self.preview.clone())
            )
    }
}

pub struct Editor {
    input: Entity<InputState>,
}

impl EventEmitter<EditorEvent> for Editor {}

impl Editor {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let input = cx.new(|cx| InputState::new(window, cx));
        
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
            .flex_1()
            .bg(theme.editor_background)
            .text_color(theme.text_color)
            .p_4()
            .child(Input::new(&self.input))
    }
}
