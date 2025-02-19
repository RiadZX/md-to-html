use md_to_html::MarkdownConverter;
use std::fs;
use std::path::Path;

#[test]
fn test_convert_content() {
    let converter = MarkdownConverter::new(Some(&"default".to_string()));
    let markdown = "# Hello\nThis is a test";
    let html = converter.convert_content(markdown);
    assert!(html.contains("<h1>Hello</h1>"));
    assert!(html.contains("<p>This is a test</p>"));
}

#[test]
fn test_convert_file() {
    let temp_dir = tempfile::tempdir().unwrap();
    let input_path = temp_dir.path().join("test.md");
    let output_path = temp_dir.path().join("test.html");
    
    fs::write(&input_path, "# Test\nHello world").unwrap();
    
    let converter = MarkdownConverter::new(Some(&"default".to_string()));
    converter.convert_file(&input_path, &output_path).unwrap();
    
    assert!(Path::new(&output_path).exists());
    let content = fs::read_to_string(&output_path).unwrap();
    assert!(content.contains("<h1>Test</h1>"));
    assert!(content.contains("<p>Hello world</p>"));
}