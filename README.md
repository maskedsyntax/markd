# Markd

**Offline Markdown Note Compiler & Publisher**

Markd is a Rust-based tool that watches a folder of Markdown files (your personal notes/knowledge base), compiles them into a clean, searchable static HTML bundle, applies custom themes/CSS, and generates a table of contents.

## Features

- **Folder Watcher:** Automatically rebuilds your site on every change.
- **Search Indexing:** Full-text search with Tantivy.
- **Custom Templating:** Uses Tera for easy HTML layout customization.
- **Table of Contents:** Automatic generation of an index page.
- **Custom CSS:** Easy themes via the `theme` directory.

## Usage

### 1. Installation

To use `markd` as a global command:

```bash
# From the project root
cargo install --path .
```

### 2. Available Commands

- **Initialize a project**
  Creates the `notes/`, `templates/`, and `theme/` directories with sample files.
  ```bash
  markd init
  ```

- **Build the static site**
  Compiles all Markdown files from the source directory to HTML in the output directory.
  ```bash
  markd build
  ```
  *Options:*
  - `-s, --source <PATH>`: Source directory of notes (default: `notes`)
  - `-o, --output <PATH>`: Output directory for the site (default: `dist`)

- **Watch for changes**
  Starts a file watcher that rebuilds your site automatically whenever you save a note.
  ```bash
  markd watch
  ```
  *Options:*
  - `-s, --source <PATH>`: Source directory of notes (default: `notes`)

### 3. Development (Without Installation)

If you haven't installed the binary globally, you can run all commands through Cargo:

```bash
cargo run -- build
cargo run -- watch
```

## Project Structure

- `notes/`: Put your Markdown files here.
- `templates/`: HTML layouts (Tera templates).
- `theme/`: CSS and other assets.
- `dist/`: The generated static site.
- `dist/.index`: Search index data.
