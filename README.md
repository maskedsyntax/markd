# Markd 📝

Markd is a fast, clean, and minimal Markdown editor written in Rust. It features a GPU-accelerated split-pane interface: a high-performance Markdown text editor on the left and a live, GitHub-style preview on the right.

Built with **GPUI** (the Rust-native GPU-accelerated UI framework used by Zed) for a modern, fluid, and highly responsive experience on Linux.

## Features

- **GPU-Accelerated UI**: Powered by GPUI for high frame rates and low latency.
- **Resizable Split-Pane**: Draggable vertical divider to balance editor and preview.
- **Advanced Code Editor**: High-performance editing with line numbers and syntax awareness.
- **GitHub-Flavored Markdown**: Real-time rendering of headers, blockquotes, tables, links, task lists, and more.
- **Syntax Highlighting**: Beautiful code block highlighting powered by `syntect`.
- **Auto-Render**: Debounced live preview that updates as you type (150ms delay).
- **Status Bar**: Real-time cursor position (line and column) and file encoding.
- **Autosave**: Background task that periodically saves your work every 30 seconds.
- **Native File Operations**: Integrated New, Open, and Save dialogs.

## Installation

### Prerequisites

Ensure you have the following installed:
- Rust toolchain (via [rustup](https://rustup.rs/))
- System dependencies for GPUI (varies by distro, typically `libx11`, `libxkbcommon`, `vulkan-loader`, etc.)

### Build & Run

```bash
cargo run
```

## Architecture

Markd follows a clean, modular architecture:
- **UI Layer**: GPUI components and resizable layouts.
- **Core Layer**: Markdown parsing (`pulldown-cmark`) and rendering engine.
- **IO Layer**: File persistence and background autosave management.

## Keyboard Shortcuts

- **Ctrl+N**: New File
- **Ctrl+O**: Open File
- **Ctrl+S**: Save File
- **Ctrl+R**: Manual Render
- **Ctrl+Shift+R**: Toggle Auto-Render

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
