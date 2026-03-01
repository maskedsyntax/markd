use anyhow::Result;
use clap::{Parser, Subcommand};
use markd::{MarkdConfig, MarkdCompiler};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "markd")]
#[command(about = "Offline Markdown Note Compiler & Publisher", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Build the static site from markdown notes
    Build {
        /// Source directory of notes
        #[arg(short, long)]
        source: Option<PathBuf>,
        
        /// Output directory for the site
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    /// Watch the notes directory and rebuild on change
    Watch {
        /// Source directory of notes
        #[arg(short, long)]
        source: Option<PathBuf>,
    },
    /// Initialize a new markd project
    Init,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Build { source, output }) => {
            let config = MarkdConfig {
                source_dir: source.unwrap_or_else(|| PathBuf::from("notes")),
                output_dir: output.unwrap_or_else(|| PathBuf::from("dist")),
                ..Default::default()
            };
            
            println!("Building site from {:?} to {:?}...", config.source_dir, config.output_dir);
            let template_dir = PathBuf::from("templates");
            let compiler = MarkdCompiler::new(config.site_title, Some(&template_dir))?;
            compiler.build_all(&config.source_dir, &config.output_dir)?;
            println!("Build complete!");
        }
        Some(Commands::Watch { source }) => {
            let config = MarkdConfig {
                source_dir: source.unwrap_or_else(|| PathBuf::from("notes")),
                ..Default::default()
            };
            
            let template_dir = PathBuf::from("templates");
            let compiler = MarkdCompiler::new(config.site_title, Some(&template_dir))?;
            
            // Initial build
            println!("Initial build...");
            compiler.build_all(&config.source_dir, &config.output_dir)?;
            
            let watcher = markd::watcher::MarkdWatcher::new(compiler);
            watcher.watch(&config.source_dir, &config.output_dir)?;
        }
        Some(Commands::Init) => {
            println!("Initializing new markd project...");
            // TODO: Implement init logic
        }
        None => {
            println!("Use --help for usage information.");
        }
    }

    Ok(())
}
