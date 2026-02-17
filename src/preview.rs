use gpui::*;
use gpui_component::scroll::ScrollableElement;
use crate::theme::Theme;
use crate::renderer::MarkdownRenderer;

pub struct Preview {
    text: String,
}

impl Preview {
    pub fn new() -> Self {
        Self { text: String::new() }
    }

    pub fn set_text(&mut self, text: String, cx: &mut Context<Self>) {
        self.text = text;
        cx.notify();
    }
}

impl Render for Preview {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let renderer = MarkdownRenderer::new(self.text.clone());
        
        div()
            .size_full()
            .bg(theme.preview_background)
            .text_color(theme.text_color)
            .p_4()
            .overflow_y_scrollbar()
            .child(renderer.render(theme))
    }
}
