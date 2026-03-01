use anyhow::Result;
use pulldown_cmark::{html, Options, Parser};
use std::fs;
use std::path::Path;

use crate::template::MarkdRenderer;

pub struct MarkdCompiler {
    pub options: Options,
    pub renderer: Option<MarkdRenderer>,
    pub site_title: String,
}

impl MarkdCompiler {
    pub fn new(site_title: String, template_dir: Option<&Path>) -> Result<Self> {
        let mut options = Options::empty();
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_TABLES);
        options.insert(Options::ENABLE_FOOTNOTES);
        options.insert(Options::ENABLE_TASKLISTS);

        let renderer = if let Some(dir) = template_dir {
            Some(MarkdRenderer::new(dir)?)
        } else {
            None
        };

        Ok(Self { options, renderer, site_title })
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
        
        let final_content = if let Some(renderer) = &self.renderer {
            let page_title = input_path.file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("Untitled");
            renderer.render(&self.site_title, page_title, &html_content)?
        } else {
            html_content
        };

        // Ensure output directory exists
        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        fs::write(output_path, final_content)?;
        Ok(())
    }

    pub fn build_all(&self, source_dir: &Path, output_dir: &Path) -> Result<()> {
        use walkdir::WalkDir;

        if !source_dir.exists() {
            anyhow::bail!("Source directory {:?} does not exist", source_dir);
        }

        for entry in WalkDir::new(source_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().map_or(false, |ext| ext == "md"))
        {
            let relative_path = entry.path().strip_prefix(source_dir)?;
            let mut output_path = output_dir.join(relative_path);
            output_path.set_extension("html");

            println!("Compiling {:?}...", relative_path);
            self.compile_file(entry.path(), &output_path)?;
        }

        Ok(())
    }
}
