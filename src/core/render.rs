use pulldown_cmark::{Event, Options, Parser, Tag, TagEnd};
use gtk::glib;

pub fn markdown_to_pango(input: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_STRIKETHROUGH);

    let parser = Parser::new_ext(input, options);
    
    let mut output = String::new();
    let mut in_code_block = false;
    // Pango doesn't have a direct "list" tag, so we simulate it.
    let mut list_stack = Vec::new(); // Stores list type (ordered/unordered)

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
                    Tag::CodeBlock(_) => {
                        in_code_block = true;
                        output.push_str("<tt><span background='#f0f0f0'>");
                    }
                    Tag::List(kind) => {
                        list_stack.push(kind);
                    }
                    Tag::Item => {
                        output.push_str("
");
                        // Add indentation based on nesting depth
                        for _ in 0..list_stack.len() {
                            output.push_str("  ");
                        }
                        
                        if let Some(Some(start)) = list_stack.last().map(|k| *k) {
                             // Ordered list not easily supported with auto-increment in simple pango parsing loop without extra state
                             // simplifying to bullet for now for all or simple number check if we tracked index
                             output.push_str("• ");
                        } else {
                             output.push_str("• ");
                        }
                    }
                    _ => {}
                }
            }
            Event::End(tag) => {
                match tag {
                    TagEnd::Heading(_) => output.push_str("</span>

"),
                    TagEnd::Paragraph => output.push_str("</span>

"),
                    TagEnd::Emphasis => output.push_str("</i>"),
                    TagEnd::Strong => output.push_str("</b>"),
                    TagEnd::Strikethrough => output.push_str("</s>"),
                    TagEnd::CodeBlock => {
                        in_code_block = false;
                        output.push_str("</span></tt>

");
                    }
                    TagEnd::List(_) => {
                        list_stack.pop();
                        if list_stack.is_empty() {
                            output.push_str("
");
                        }
                    }
                    TagEnd::Item => {} // Newline handled at start
                    _ => {}
                }
            }
            Event::Text(text) => {
                let escaped = glib::markup_escape_text(&text);
                output.push_str(&escaped);
            }
            Event::Code(text) => {
                let escaped = glib::markup_escape_text(&text);
                output.push_str(&format!("<tt>{}</tt>", escaped));
            }
            Event::SoftBreak => output.push(' '),
            Event::HardBreak => output.push_str("
"),
            _ => {}
        }
    }
    output
}
