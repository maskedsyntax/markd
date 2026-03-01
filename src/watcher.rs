use anyhow::Result;
use notify::{Watcher, RecursiveMode, Event, RecommendedWatcher, Config};
use std::path::Path;
use std::sync::mpsc::channel;
use std::time::Duration;
use crate::compiler::MarkdCompiler;

pub struct MarkdWatcher {
    compiler: MarkdCompiler,
}

impl MarkdWatcher {
    pub fn new(compiler: MarkdCompiler) -> Self {
        Self { compiler }
    }

    pub fn watch(&self, source_dir: &Path, output_dir: &Path) -> Result<()> {
        let (tx, rx) = channel();

        let mut watcher = RecommendedWatcher::new(tx, Config::default())?;

        watcher.watch(source_dir, RecursiveMode::Recursive)?;

        println!("Watching for changes in {:?}...", source_dir);

        loop {
            match rx.recv() {
                Ok(Ok(event)) => {
                    if self.is_relevant_event(&event) {
                        println!("Change detected: {:?}. Rebuilding...", event.paths);
                        if let Err(e) = self.compiler.build_all(source_dir, output_dir) {
                            eprintln!("Build error: {}", e);
                        }
                    }
                }
                Ok(Err(e)) => eprintln!("Watch error: {:?}", e),
                Err(e) => eprintln!("Channel error: {:?}", e),
            }
        }
    }

    fn is_relevant_event(&self, event: &Event) -> bool {
        event.paths.iter().any(|p| {
            p.extension().map_or(false, |ext| ext == "md")
        })
    }
}
