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
            background: rgb(0x2e3440).into(),
            foreground: rgb(0xd8dee9).into(),
            editor_background: rgb(0x2e3440).into(),
            preview_background: rgb(0x3b4252).into(),
            border: rgb(0x4c566a).into(),
            toolbar_background: rgb(0x24292e).into(),
            status_bar_background: rgb(0x24292e).into(),
            text_color: rgb(0xd8dee9).into(),
            heading_color: rgb(0x88c0d0).into(),
            accent_color: rgb(0x81a1c1).into(),
        }
    }
}

impl Global for Theme {}
