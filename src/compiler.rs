use anyhow::Result;
use pulldown_cmark::{html, Options, Parser};
use std::fs;
use std::path::Path;

use crate::template::MarkdRenderer;
use crate::indexer::MarkdIndexer;

pub struct MarkdCompiler {
    pub options: Options,
    pub renderer: Option<MarkdRenderer>,
    pub indexer: Option<MarkdIndexer>,
    pub site_title: String,
}

impl MarkdCompiler {
    pub fn new(site_title: String, template_dir: Option<&Path>, index_path: Option<&Path>) -> Result<Self> {
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

        let indexer = if let Some(path) = index_path {
            Some(MarkdIndexer::new(path)?)
        } else {
            None
        };

        Ok(Self { options, renderer, indexer, site_title })
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
        
        let page_title = input_path.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("Untitled");

        if let Some(indexer) = &self.indexer {
            indexer.index_document(page_title, &md_content, output_path.to_str().unwrap_or(""))?;
        }

        let final_content = if let Some(renderer) = &self.renderer {
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

        let mut pages = Vec::new();

        for entry in WalkDir::new(source_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().map_or(false, |ext| ext == "md"))
        {
            let relative_path = entry.path().strip_prefix(source_dir)?;
            let mut output_path = output_dir.join(relative_path);
            output_path.set_extension("html");

            let page_title = entry.path().file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("Untitled")
                .to_string();

            println!("Compiling {:?}...", relative_path);
            self.compile_file(entry.path(), &output_path)?;
            
            let mut link_path = relative_path.to_path_buf();
            link_path.set_extension("html");
            pages.push((page_title, link_path.to_str().unwrap_or("").to_string()));
        }

        self.generate_index(output_dir, pages)?;

        Ok(())
    }

    fn generate_index(&self, output_dir: &Path, pages: Vec<(String, String)>) -> Result<()> {
        if let Some(renderer) = &self.renderer {
            let mut index_content = String::from("<h1>Table of Contents</h1><ul>");
            for (title, path) in pages {
                index_content.push_str(&format!("<li><a href=\"{}\">{}</a></li>", path, title));
            }
            index_content.push_str("</ul>");

            let rendered = renderer.render(&self.site_title, "Home", &index_content)?;
            fs::write(output_dir.join("index.html"), rendered)?;
        }
        Ok(())
    }
}
