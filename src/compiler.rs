use anyhow::Result;
use pulldown_cmark::{html, Options, Parser};
use std::fs;
use std::path::{Path, PathBuf};

pub struct MarkdCompiler {
    pub options: Options,
}

impl MarkdCompiler {
    pub fn new() -> Self {
        let mut options = Options::empty();
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_TABLES);
        options.insert(Options::ENABLE_FOOTNOTES);
        options.insert(Options::ENABLE_TASKLISTS);
        Self { options }
    }

    pub fn compile_md(&self, md_content: &str) -> String {
        let parser = Parser::new_ext(md_content, self.options);
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);
        html_output
    }

    pub fn compile_file(&self, input_path: &Path, output_path: &Path) -> Result<()> {
        let md_content = fs::read_to_string(input_path)?;
        let html_content = self.compile_md(&md_content);
        
        // Ensure output directory exists
        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        fs::write(output_path, html_content)?;
        Ok(())
    }
}
