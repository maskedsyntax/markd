use anyhow::Result;
use tera::{Context, Tera};
use std::path::Path;

pub struct MarkdRenderer {
    pub tera: Tera,
}

impl MarkdRenderer {
    pub fn new(template_dir: &Path) -> Result<Self> {
        let template_pattern = format!("{}/**/*.html", template_dir.display());
        let tera = Tera::new(&template_pattern)?;
        Ok(Self { tera })
    }

    pub fn render(&self, site_title: &str, page_title: &str, content: &str) -> Result<String> {
        let mut context = Context::new();
        context.insert("site_title", site_title);
        context.insert("page_title", page_title);
        context.insert("content", content);
        
        Ok(self.tera.render("layout.html", &context)?)
    }
}
