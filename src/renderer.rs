use gpui::*;
use pulldown_cmark::{Parser, Options, Event, Tag};
use crate::theme::Theme;

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

        let parser = Parser::new_ext(&self.text, options);
        
        let mut stack: Vec<Option<Div>> = vec![Some(div().flex().flex_col().gap_2())];

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
                            stack.push(Some(div().text_size(size).font_weight(FontWeight::BOLD).text_color(theme.heading_color)));
                        }
                        Tag::Paragraph => {
                            stack.push(Some(div().flex().flex_wrap()));
                        }
                        Tag::Strong => {
                            stack.push(Some(div().font_weight(FontWeight::BOLD)));
                        }
                        Tag::List(_) => {
                            stack.push(Some(div().flex().flex_col().pl_4()));
                        }
                        Tag::Item => {
                            stack.push(Some(div().flex().child("• ")));
                        }
                        _ => {
                            stack.push(Some(div()));
                        }
                    }
                }
                Event::Text(text) => {
                    if let Some(top_opt) = stack.last_mut() {
                        if let Some(top) = top_opt.take() {
                            *top_opt = Some(top.child(text.to_string()));
                        }
                    }
                }
                Event::End(_tag_end) => {
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
                _ => {}
            }
        }

        stack.pop().flatten().unwrap_or_else(|| div())
    }
}
