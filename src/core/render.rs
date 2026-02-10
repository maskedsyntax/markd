use pulldown_cmark::{Event, Options, Parser, Tag, TagEnd};
use glib;

pub fn markdown_to_pango(input: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);

    let parser = Parser::new_ext(input, options);
    
    let mut output = String::new();
    let mut list_stack = Vec::new(); 

    for event in parser {
        match event {
            Event::Start(tag) => {
                match tag {
                    Tag::Heading { level, .. } => {
                        let size = match level {
                            pulldown_cmark::HeadingLevel::H1 => "xx-large",
                            pulldown_cmark::HeadingLevel::H2 => "x-large",
                            pulldown_cmark::HeadingLevel::H3 => "large",
                            _ => "medium",
                        };
                        output.push_str(&format!("<span weight='bold' size='{}'>", size));
                    }
                    Tag::Paragraph => {
                        output.push_str("<span>");
                    }
                    Tag::Emphasis => output.push_str("<i>"),
                    Tag::Strong => output.push_str("<b>"),
                    Tag::Strikethrough => output.push_str("<s>"),
                    Tag::Link { .. } => {
                        output.push_str("<span foreground='#3584e4' underline='single'>");
                    }
                    Tag::CodeBlock(_) => {
                        output.push_str("\n<span font_family='monospace' foreground='#888888'>");
                    }
                    Tag::List(kind) => {
                        list_stack.push(kind);
                        output.push_str("\n");
                    }
                    Tag::Item => {
                        output.push_str("\n");
                        for _ in 0..list_stack.len().saturating_sub(1) {
                            output.push_str("    ");
                        }
                        output.push_str(" • ");
                    }
                    _ => {}
                }
            }
            Event::End(tag) => {
                match tag {
                    TagEnd::Heading(_) => output.push_str("</span>\n\n"),
                    TagEnd::Paragraph => output.push_str("</span>\n\n"),
                    TagEnd::Emphasis => output.push_str("</i>"),
                    TagEnd::Strong => output.push_str("</b>"),
                    TagEnd::Strikethrough => output.push_str("</s>"),
                    TagEnd::Link => output.push_str("</span>"),
                    TagEnd::CodeBlock => {
                        output.push_str("</span>\n\n");
                    }
                    TagEnd::List(_) => {
                        list_stack.pop();
                        if list_stack.is_empty() {
                            output.push_str("\n");
                        }
                    }
                    TagEnd::Item => {} 
                    _ => {}
                }
            }
            Event::Text(text) => {
                let escaped = glib::markup_escape_text(&text);
                output.push_str(&escaped);
            }
            Event::Code(text) => {
                let escaped = glib::markup_escape_text(&text);
                output.push_str(&format!("<span font_family='monospace' foreground='#e01b24'>{}</span>", escaped));
            }
            Event::Html(html) | Event::InlineHtml(html) => {
                let escaped = glib::markup_escape_text(&html);
                output.push_str(&format!("<span foreground='#888888'>{}</span>", escaped));
            }
            Event::SoftBreak => output.push(' '),
            Event::HardBreak => output.push_str("\n"),
            Event::Rule => output.push_str("\n────────────────────────────────\n\n"),
            _ => {}
        }
    }
    output
}