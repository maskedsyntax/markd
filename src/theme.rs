use gpui::*;

pub struct Theme {
    pub background: Hsla,
    pub foreground: Hsla,
    pub editor_background: Hsla,
    pub preview_background: Hsla,
    pub border: Hsla,
    pub toolbar_background: Hsla,
    pub status_bar_background: Hsla,
    pub text_color: Hsla,
    pub heading_color: Hsla,
    pub accent_color: Hsla,
}

impl Theme {
    pub fn dark() -> Self {
        Self {
            background: rgb(0x181818).into(),
            foreground: rgb(0xcccccc).into(),
            editor_background: rgb(0x181818).into(),
            preview_background: rgb(0x181818).into(),
            border: rgb(0x333333).into(),
            toolbar_background: rgb(0x242424).into(),
            status_bar_background: rgb(0x1e1e1e).into(),
            text_color: rgb(0xcccccc).into(),
            heading_color: rgb(0x569cd6).into(),
            accent_color: rgb(0x007acc).into(),
        }
    }
}

impl Global for Theme {}
