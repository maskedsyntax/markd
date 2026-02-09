# Markd 📝

Markd is a fast, clean, and minimal Markdown editor written in Rust. It features a split-pane interface: a native Markdown text editor on the left and a live, GitHub-style preview on the right.

Built with **GTK4** and **Libadwaita** for a native and modern experience on GNOME-based desktops.

## Features

- **Split-Pane Layout**: Adjustable vertical divider to balance editor and preview.
- **Live Preview**: Real-time rendering of Markdown as you type.
- **Native Components**: Powered by `GtkSourceView` for high-performance editing.
- **Markdown Support**: Headers, lists, emphasis (bold/italic), code blocks, and more.
- **Lightweight**: Fast startup and responsive interface.

## Installation

### Prerequisites

Ensure you have the following installed:
- Rust toolchain (via [rustup](https://rustup.rs/))
- GTK4 and Libadwaita development libraries:
  ```bash
  sudo apt install libgtk-4-dev libadwaita-1-dev libgtksourceview-5-dev
  ```

### Build & Run

```bash
cargo run
```

## Architecture

Markd follows a modular design:
- **UI Layer**: GTK4/Libadwaita implementation.
- **Core Layer**: Markdown parsing and Pango rendering engine.
- **IO Layer**: File management and persistence.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.