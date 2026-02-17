use gpui::*;
use pulldown_cmark::{Parser, Options, Event, Tag, TagEnd, CodeBlockKind};
use crate::theme::Theme;
use syntect::parsing::SyntaxSet;
use syntect::highlighting::ThemeSet;
use syntect::easy::HighlightLines;
use syntect::util::LinesWithEndings;

pub struct MarkdownRenderer {
    text: String,
}

impl MarkdownRenderer {
    pub fn new(text: String) -> Self {
        Self { text }
    }

    pub fn render(&self, theme: &Theme) -> impl IntoElement {
        let mut options = Options::empty();
        options.insert(Options::ENABLE_TABLES);
        options.insert(Options::ENABLE_FOOTNOTES);
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_TASKLISTS);
        options.insert(Options::ENABLE_HEADING_ATTRIBUTES);

        let ps = SyntaxSet::load_defaults_newlines();
        let ts = ThemeSet::load_defaults();
        let syn_theme = &ts.themes["base16-ocean.dark"];

        let parser = Parser::new_ext(&self.text, options);
        
        let mut stack: Vec<Option<Div>> = vec![Some(div().flex().flex_col().gap_2().w_full())];

        let mut current_code_block_lang: Option<String> = None;

        for event in parser {
            match event {
                Event::Start(tag) => {
                    match tag {
                        Tag::Heading { level, .. } => {
                            let size = match level {
                                pulldown_cmark::HeadingLevel::H1 => px(32.0),
                                pulldown_cmark::HeadingLevel::H2 => px(24.0),
                                pulldown_cmark::HeadingLevel::H3 => px(20.0),
                                _ => px(16.0),
                            };
                            stack.push(Some(div().text_size(size).font_weight(FontWeight::BOLD).text_color(theme.heading_color).mt_4().mb_2()));
                        }
                        Tag::Paragraph => {
                            stack.push(Some(div().flex().flex_wrap().gap_1()));
                        }
                        Tag::Strong => {
                            stack.push(Some(div().font_weight(FontWeight::BOLD)));
                        }
                        Tag::Emphasis => {
                            stack.push(Some(div().italic()));
                        }
                        Tag::Strikethrough => {
                            stack.push(Some(div().line_through()));
                        }
                        Tag::BlockQuote => {
                            stack.push(Some(div().flex().flex_col().pl_4().border_l(px(4.0)).border_color(theme.border).text_color(theme.text_color.opacity(0.7))));
                        }
                        Tag::CodeBlock(kind) => {
                            let lang = match kind {
                                CodeBlockKind::Fenced(l) => l.to_string(),
                                CodeBlockKind::Indented => String::new(),
                            };
                            current_code_block_lang = Some(lang);
                            stack.push(Some(div().bg(theme.editor_background).p_4().rounded_md().w_full().font_family("monospace").flex().flex_col()));
                        }
                        Tag::List(_) => {
                            stack.push(Some(div().flex().flex_col().pl_4().gap_1()));
                        }
                        Tag::Item => {
                            stack.push(Some(div().flex().gap_2()));
                            if let Some(top_opt) = stack.last_mut() {
                                if let Some(top) = top_opt.take() {
                                    *top_opt = Some(top.child("•"));
                                }
                            }
                        }
                        Tag::Table(_) => {
                            stack.push(Some(div().flex().flex_col().border(px(1.0)).border_color(theme.border)));
                        }
                        Tag::TableHead => {
                            stack.push(Some(div().flex().bg(theme.toolbar_background).font_weight(FontWeight::BOLD)));
                        }
                        Tag::TableRow => {
                            stack.push(Some(div().flex().border_b(px(1.0)).border_color(theme.border)));
                        }
                        Tag::TableCell => {
                            stack.push(Some(div().flex_1().p_2()));
                        }
                        Tag::Link { dest_url, .. } => {
                            stack.push(Some(div().text_color(theme.accent_color).underline()));
                        }
                        _ => {
                            stack.push(Some(div()));
                        }
                    }
                }
                Event::Text(text) => {
                    if let Some(lang) = &current_code_block_lang {
                        let syntax = ps.find_syntax_by_token(lang).unwrap_or_else(|| ps.find_syntax_plain_text());
                        let mut h = HighlightLines::new(syntax, syn_theme);
                        
                        let mut code_div = div().flex().flex_col();
                        for line in LinesWithEndings::from(&text) {
                            let ranges: Vec<(syntect::highlighting::Style, &str)> = h.highlight_line(line, &ps).unwrap_or_default();
                            let mut line_div = div().flex().flex_wrap();
                            for (s, t) in ranges {
                                let gpui_color: Hsla = Rgba {
                                    r: s.foreground.r as f32 / 255.0,
                                    g: s.foreground.g as f32 / 255.0,
                                    b: s.foreground.b as f32 / 255.0,
                                    a: s.foreground.a as f32 / 255.0,
                                }.into();
                                line_div = line_div.child(div().text_color(gpui_color).child(t.to_string()));
                            }
                            code_div = code_div.child(line_div);
                        }

                        if let Some(top_opt) = stack.last_mut() {
                            if let Some(top) = top_opt.take() {
                                *top_opt = Some(top.child(code_div));
                            }
                        }
                    } else {
                        if let Some(top_opt) = stack.last_mut() {
                            if let Some(top) = top_opt.take() {
                                *top_opt = Some(top.child(text.to_string()));
                            }
                        }
                    }
                }
                Event::Code(text) => {
                    if let Some(top_opt) = stack.last_mut() {
                        if let Some(top) = top_opt.take() {
                            *top_opt = Some(top.child(
                                div()
                                    .bg(theme.editor_background)
                                    .px_1()
                                    .rounded_sm()
                                    .font_family("monospace")
                                    .child(text.to_string())
                            ));
                        }
                    }
                }
                Event::End(tag_end) => {
                    if let TagEnd::CodeBlock = tag_end {
                        current_code_block_lang = None;
                    }
                    if stack.len() > 1 {
                        let completed = stack.pop().flatten();
                        if let Some(top_opt) = stack.last_mut() {
                            if let Some(top) = top_opt.take() {
                                if let Some(c) = completed {
                                    *top_opt = Some(top.child(c));
                                } else {
                                    *top_opt = Some(top);
                                }
                            }
                        }
                    }
                }
                Event::SoftBreak => {
                    if let Some(top_opt) = stack.last_mut() {
                        if let Some(top) = top_opt.take() {
                            *top_opt = Some(top.child(" "));
                        }
                    }
                }
                Event::HardBreak => {
                    if let Some(top_opt) = stack.last_mut() {
                        if let Some(top) = top_opt.take() {
                            *top_opt = Some(top.child(div().h_2()));
                        }
                    }
                }
                _ => {}
            }
        }

        stack.pop().flatten().unwrap_or_else(|| div())
    }
}
