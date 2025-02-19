# Markdown to HTML Converter

A command-line tool that converts Markdown files to styled HTML documents with support for multiple themes.

## Features

- Convert Markdown files to HTML
- Multiple theme support (default, dark, and light)
- Responsive design

## Installation

Make sure you have Rust installed on your system. Then clone this repository and build the project:

```bash
git clone https://github.com/yourusername/md-to-html.git
cd md-to-html
cargo build --release
```

## Usage
Basic usage:

```bash
md-to-html --input example.md --output example.html
```

With theme selection:

```bash
md-to-html --input example.md --output example.html --theme dark
```

Available options:
- `-i, --input` : Input markdown file path
- `-o, --output`: Output HTML file path
- `-t, --theme`: Theme selection (default, dark, or light)
## Available Themes
- Default : A clean, minimal theme
- Dark : Dark mode theme for comfortable reading in low-light conditions
- Light : GitHub-like light theme