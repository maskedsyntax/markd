use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct MarkdConfig {
    pub source_dir: PathBuf,
    pub output_dir: PathBuf,
    pub site_title: String,
    pub theme_dir: Option<PathBuf>,
}

impl Default for MarkdConfig {
    fn default() -> Self {
        Self {
            source_dir: PathBuf::from("notes"),
            output_dir: PathBuf::from("dist"),
            site_title: String::from("My Markd Notes"),
            theme_dir: None,
        }
    }
}
