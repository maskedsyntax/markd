# Markd

**Offline Markdown Note Compiler & Publisher**

Markd is a Rust-based tool that watches a folder of Markdown files (your personal notes/knowledge base), compiles them into a clean, searchable static HTML bundle, applies custom themes/CSS, and generates a table of contents.

## Features

- **Folder Watcher:** Automatically rebuilds your site on every change.
- **Search Indexing:** Full-text search with Tantivy.
- **Custom Templating:** Uses Tera for easy HTML layout customization.
- **Table of Contents:** Automatic generation of an index page.
- **Custom CSS:** Easy themes via the `theme` directory.

## Getting Started

### Initialize a project
```bash
markd init
```

### Build the site
```bash
markd build
```

### Watch for changes
```bash
markd watch
```

## Project Structure

- `notes/`: Put your Markdown files here.
- `templates/`: HTML layouts (Tera templates).
- `theme/`: CSS and other assets.
- `dist/`: The generated static site.
- `dist/.index`: Search index data.
