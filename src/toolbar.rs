use gpui::*;
use gpui_component::button::*;
use crate::theme::Theme;

pub enum ToolbarEvent {
    New,
    Open,
    Save,
    Render,
    ToggleAutoRender(bool),
}

pub struct Toolbar {
    auto_render: bool,
}

impl EventEmitter<ToolbarEvent> for Toolbar {}

impl Toolbar {
    pub fn new() -> Self {
        Self { auto_render: true }
    }
}

impl Render for Toolbar {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        
        div()
            .flex()
            .h_8()
            .bg(theme.background)
            .border_b(px(1.0))
            .border_color(theme.border)
            .items_center()
            .px_2()
            .gap_2()
            .child(
                div()
                    .flex()
                    .gap_1()
                    .child(
                        Button::new("new")
                            .label("New")
                            .on_click(cx.listener(|_, _, _, cx| {
                                cx.emit(ToolbarEvent::New);
                            }))
                    )
                    .child(
                        Button::new("open")
                            .label("Open")
                            .on_click(cx.listener(|_, _, _, cx| {
                                cx.emit(ToolbarEvent::Open);
                            }))
                    )
                    .child(
                        Button::new("save")
                            .label("Save")
                            .on_click(cx.listener(|_, _, _, cx| {
                                cx.emit(ToolbarEvent::Save);
                            }))
                    )
            )
            .child(
                div()
                    .w_px()
                    .h_6()
                    .bg(theme.border)
            )
            .child(
                Button::new("render")
                    .label("Render")
                    .on_click(cx.listener(|_, _, _, cx| {
                        cx.emit(ToolbarEvent::Render);
                    }))
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .child("Auto-render")
                    .child(
                        Button::new("auto_render")
                            .label(if self.auto_render { "ON" } else { "OFF" })
                            .on_click(cx.listener(|this: &mut Self, _, _, cx| {
                                this.auto_render = !this.auto_render;
                                cx.emit(ToolbarEvent::ToggleAutoRender(this.auto_render));
                                cx.notify();
                            }))
                    )
            )
    }
}
