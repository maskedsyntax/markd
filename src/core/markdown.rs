use pulldown_cmark::{Event, Options, Parser, Tag, TagEnd};

#[derive(Debug, Clone)]
pub enum MarkdownItem {
    Heading(u8, String),
    Paragraph(String),
    CodeBlock { lang: String, code: String },
    List(Vec<String>),
}

pub fn parse_markdown(input: &str) -> Vec<MarkdownItem> {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_TASKLISTS);
    
    let parser = Parser::new_ext(input, options);
    let mut items = Vec::new();
    let mut current_text = String::new();
    let mut current_heading_level = None;
    let mut is_in_code_block = None;
    let mut current_list = None;
    let mut is_in_item = false;

    for event in parser {
        match event {
            Event::Start(tag) => {
                match tag {
                    Tag::Heading { level, .. } => {
                        let lvl = match level {
                             pulldown_cmark::HeadingLevel::H1 => 1,
                             pulldown_cmark::HeadingLevel::H2 => 2,
                             pulldown_cmark::HeadingLevel::H3 => 3,
                             pulldown_cmark::HeadingLevel::H4 => 4,
                             pulldown_cmark::HeadingLevel::H5 => 5,
                             pulldown_cmark::HeadingLevel::H6 => 6,
                        };
                        current_heading_level = Some(lvl);
                    }
                    Tag::CodeBlock(kind) => {
                         let lang = match kind {
                             pulldown_cmark::CodeBlockKind::Fenced(l) => l.to_string(),
                             _ => String::new(),
                         };
                         is_in_code_block = Some(lang);
                    }
                    Tag::List(_) => {
                        current_list = Some(Vec::new());
                    }
                    Tag::Item => {
                        is_in_item = true;
                    }
                    _ => {}
                }
                current_text.clear();
            }
            Event::Text(text) => {
                current_text.push_str(&text);
            }
            Event::End(tag_end) => {
                match tag_end {
                    TagEnd::Heading(_) => {
                         if let Some(level) = current_heading_level {
                             items.push(MarkdownItem::Heading(level, current_text.clone()));
                         }
                         current_heading_level = None;
                    }
                    TagEnd::Paragraph => {
                        if !is_in_item {
                            items.push(MarkdownItem::Paragraph(current_text.clone()));
                        }
                    }
                    TagEnd::Item => {
                        if let Some(ref mut list) = current_list {
                            list.push(current_text.clone());
                        }
                        is_in_item = false;
                    }
                    TagEnd::CodeBlock => {
                         let lang = is_in_code_block.take().unwrap_or_default();
                         items.push(MarkdownItem::CodeBlock { lang, code: current_text.clone() });
                    }
                    TagEnd::List(_) => {
                        if let Some(list) = current_list.take() {
                            items.push(MarkdownItem::List(list));
                        }
                    }
                     _ => {}
                }
                current_text.clear();
            }
            Event::Code(text) => {
                 current_text.push_str("`");
                 current_text.push_str(&text);
                 current_text.push_str("`");
            }
            _ => {}
        }
    }
    items
}
