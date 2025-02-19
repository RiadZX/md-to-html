use std::fs::{self, File};
use std::io::{self, Read};
use std::path::Path;
mod themes;

pub struct MarkdownConverter{
    theme: String,
}

impl<'a> MarkdownConverter {
    pub fn new(theme: Option<& String>) -> MarkdownConverter {
        MarkdownConverter {
            theme: theme.unwrap_or(&String::from("default")).to_string(),
        }
    }

    pub fn convert_file<P: AsRef<Path>>(&self, input_path: P, output_path: P) -> io::Result<()> {
        let content = self.read_file(input_path)?;
        let html = self.convert_content(&content);
        let styled_html = self.wrap_with_html(&html)?;
        self.write_file(output_path, &styled_html)?;
        Ok(())
    }

    pub fn convert_content(&self, markdown: &str) -> String {
        markdown::to_html(markdown)
    }

    fn wrap_with_html(&self, content: &str) -> io::Result<String> {
        let css = self.read_theme()?;
        Ok(format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <style>
{}
    </style>
</head>
<body>
{}
</body>
</html>"#,
            css, content
        ))
    }

    fn read_theme(&self) -> io::Result<String> {
        Ok(themes::get_theme(&self.theme).to_string())
    }

    fn read_file<P: AsRef<Path>>(&self, path: P) -> io::Result<String> {
        let mut content = String::new();
        let mut file = File::open(path)?;
        file.read_to_string(&mut content)?;
        Ok(content)
    }

    fn write_file<P: AsRef<Path>>(&self, path: P, content: &str) -> io::Result<()> {
        fs::write(path, content)
    }
}